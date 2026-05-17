use astroraw_models::{FitsHeader, FrameType, RawMetadata, SessionMetadata};
use chrono::Utc;
use tracing::warn;

pub struct ResolvedMetadata {
    pub header: FitsHeader,
    pub warnings: Vec<String>,
}

pub struct MetadataResolver<'a> {
    session: &'a SessionMetadata,
    raw: RawMetadata,
    filename: String,
}

impl<'a> MetadataResolver<'a> {
    pub fn new(session: &'a SessionMetadata, raw: RawMetadata, filename: &str) -> Self {
        Self { session, raw, filename: filename.to_string() }
    }

    pub fn resolve(
        &self,
        frame_type_cli: Option<FrameType>,
        object_cli: Option<&str>,
    ) -> ResolvedMetadata {
        let mut header = FitsHeader::new();
        let mut warnings = Vec::new();

        let file_override = self.session.file_overrides.get(&self.filename);
        let eq   = self.session.equipment.as_ref();
        let cam  = eq.and_then(|e| e.camera.as_ref());
        let loc  = self.session.location.as_ref();
        let tgt  = self.session.target.as_ref();
        let wx   = self.session.weather.as_ref();
        let wcs  = self.session.wcs.as_ref();
        let out  = self.session.output.as_ref();
        let mnt  = eq.and_then(|e| e.mount.as_ref());
        let foc  = eq.and_then(|e| e.focuser.as_ref());
        let rot  = eq.and_then(|e| e.rotator.as_ref());
        let fw   = eq.and_then(|e| e.filter_wheel.as_ref());

        // ── 1. Mandatory FITS ─────────────────────────────────────────────────
        header.push_bool("SIMPLE",  true, "Conforms to FITS standard");
        header.push_int ("BITPIX",  16,   "16-bit unsigned integers");
        header.push_int ("NAXIS",   2,    "Number of array dimensions");
        header.push_int ("NAXIS1",  self.raw.width.unwrap_or(0) as i64,  "Image width in pixels");
        header.push_int ("NAXIS2",  self.raw.height.unwrap_or(0) as i64, "Image height in pixels");
        header.push_bool("EXTEND",  true, "Extensions are permitted");
        // Note: no BZERO/BSCALE — Canon 14-bit RAW fits in signed 16-bit (0..16383)
        // BZERO=32768 would shift all values by +32768 causing wrong color interpretation

        let header_mode = out.map(|o| o.header_mode.as_str()).unwrap_or("astro");
        if header_mode != "astro" {
            // minimal mode: stop here
            self.push_swcreate(&mut header);
            return ResolvedMetadata { header, warnings };
        }

        // ── 2. Software / Global ──────────────────────────────────────────────
        self.push_swcreate(&mut header);
        // Note: ROWORDER omitted — can cause Siril to flip the image and shift Bayer pattern
        // Note: EQUINOX only relevant when WCS plate-solve data is present

        // ── 3. Image / Exposure ───────────────────────────────────────────────
        let frame_type = frame_type_cli
            .or_else(|| file_override.and_then(|f| f.frame_type))
            .or(self.session.frame_type)
            .unwrap_or(FrameType::Light);
        header.push_str("IMAGETYP", frame_type.fits_imagetyp(), "Frame type");

        match self.raw.exposure_time {
            Some(t) => {
                header.push_float("EXPTIME",  t, "Exposure duration [s]");
                header.push_float("EXPOSURE", t, "Exposure duration [s] (alias)");
            }
            None => warnings.push(
                "The universe refused to provide EXPTIME. Please supply it via metadata JSON.".to_string()
            ),
        }

        // DATE-OBS priority: file_override > session.date_obs > session_date+time > EXIF
        let date_obs = file_override.and_then(|f| f.date_obs.clone())
            .or_else(|| self.session.date_obs.clone())
            .or_else(|| match (&self.session.session_date, &self.session.session_time) {
                (Some(d), Some(t)) => Some(format!("{}T{}", d, t)),
                (Some(d), None)    => Some(d.clone()),
                _ => None,
            })
            .or_else(|| self.raw.date_obs.map(|d| d.format("%Y-%m-%dT%H:%M:%S%.3f").to_string()));

        match date_obs {
            Some(ref d) => {
                header.push_str("DATE-OBS", d, "UTC date/time of exposure start");
                header.push_str("DATE-LOC", d, "Local date/time of exposure start");
                // MJD-OBS: days since 1858-11-17T00:00:00 UTC
                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&format!("{}Z", d.trim_end_matches('Z'))) {
                    let mjd = (dt.timestamp() as f64 / 86400.0) + 40587.0;
                    header.push_float("MJD-OBS", mjd, "Modified Julian Date of exposure start");
                    if let Some(exp) = self.raw.exposure_time {
                        let mjd_avg = mjd + exp / 172800.0;
                        header.push_float("MJD-AVG", mjd_avg, "Modified Julian Date of exposure midpoint");
                    }
                }
            }
            None => warnings.push("DATE-OBS could not be determined. Supply it via --date-obs or session JSON.".to_string()),
        }

        // ISO / gain
        match self.raw.iso_speed {
            Some(iso) => header.push_int("ISOSPEED", iso as i64, "ISO/gain setting"),
            None => warnings.push("ISOSPEED not found in RAW metadata.".to_string()),
        }

        // ── 4. Camera ─────────────────────────────────────────────────────────
        let instrument = self.raw.camera_model.clone()
            .or_else(|| cam.and_then(|c| c.model.clone()))
            .unwrap_or_else(|| { warnings.push("INSTRUME unknown.".to_string()); "Unknown".to_string() });
        header.push_str("INSTRUME", &instrument, "Camera model");

        if let Some(id) = cam.and_then(|c| c.camera_id.as_deref()) {
            header.push_str("CAMERAID", id, "Camera unique identifier");
        }

        // Binning
        let binx = cam.and_then(|c| c.binning_x).unwrap_or(1);
        let biny = cam.and_then(|c| c.binning_y).unwrap_or(1);
        header.push_int("XBINNING", binx as i64, "X-axis binning factor");
        header.push_int("YBINNING", biny as i64, "Y-axis binning factor");

        // Gain / Offset (electronic, not ISO)
        if let Some(g) = cam.and_then(|c| c.gain) {
            header.push_int("GAIN", g as i64, "Sensor gain");
        }
        if let Some(o) = cam.and_then(|c| c.offset).filter(|&o| o >= 0) {
            header.push_int("OFFSET", o as i64, "Sensor offset");
        }
        if let Some(eg) = cam.and_then(|c| c.egain) {
            header.push_float("EGAIN", eg, "Electrons per ADU [e-/ADU]");
        }

        // Pixel size (×binning as N.I.N.A. does)
        let px = cam.and_then(|c| c.pixel_size_x).unwrap_or(4.3) * binx as f64;
        let py = cam.and_then(|c| c.pixel_size_y).unwrap_or(4.3) * biny as f64;
        header.push_float("XPIXSZ", px, "Pixel width [µm]");
        header.push_float("YPIXSZ", py, "Pixel height [µm]");

        // Temperature
        if let Some(t) = cam.and_then(|c| c.set_temp) {
            header.push_float("SET-TEMP", t, "Cooler setpoint temperature [°C]");
        }
        if let Some(t) = cam.and_then(|c| c.ccd_temp) {
            header.push_float("CCD-TEMP", t, "Actual sensor temperature [°C]");
        }

        if let Some(rm) = cam.and_then(|c| c.readout_mode.as_deref()) {
            header.push_str("READOUTM", rm, "Readout mode");
        }
        if let Some(ul) = cam.and_then(|c| c.usb_limit).filter(|&u| u >= 0) {
            header.push_int("USBLIMIT", ul as i64, "USB bandwidth limit");
        }

        // Bayer pattern
        let bayer = self.raw.bayer_pattern.clone()
            .or_else(|| cam.and_then(|c| c.bayer_y_offset.map(|_| String::new())));

        if let Some(ref bp) = self.raw.bayer_pattern {
            if bp != "UNKNOWN" {
                header.push_str("BAYERPAT", bp, "Bayer color filter array pattern");
                header.push_str("CFATYPE",  bp, "Color filter array type");
                header.push_str("COLORTYP", "RAW", "Color type: RAW Bayer mosaic");
                let xoff = cam.and_then(|c| c.bayer_x_offset).unwrap_or(0);
                let yoff = cam.and_then(|c| c.bayer_y_offset).unwrap_or(0);
                header.push_int("XBAYROFF", xoff as i64, "Bayer pattern X offset");
                header.push_int("YBAYROFF", yoff as i64, "Bayer pattern Y offset");
            }
        } else {
            warnings.push("Don't panic. Could not determine Bayer pattern.".to_string());
        }

        // Bit depth
        let bitdepth = self.raw.bit_depth.unwrap_or(14);
        header.push_int("BITDEPTH", bitdepth as i64, "ADC bit depth of sensor");

        // Black / White / WB
        if let Some(bl) = self.raw.black_level {
            header.push_int("BLACKLVL", bl as i64, "Black level (ADU)");
            header.push_int("PEDESTAL", bl as i64, "Black level pedestal for Siril (ADU)");
        }
        if let Some(wl) = self.raw.white_level {
            header.push_int("WHITELEV", wl as i64, "White/saturation level (ADU)");
        }
        if let Some(wb) = self.raw.wb_coeffs {
            let [r, g, b, _] = wb;
            if !r.is_nan() && g != 0.0 {
                header.push_float("CBLACK_R", (r / g) as f64, "WB coefficient Red (norm. to G)");
                header.push_float("CBLACK_G", 1.0_f64,        "WB coefficient Green (reference)");
                header.push_float("CBLACK_B", (b / g) as f64, "WB coefficient Blue (norm. to G)");
            }
        }

        // ── 5. Telescope / Mount ──────────────────────────────────────────────
        if let Some(tel) = eq.and_then(|e| e.telescope.as_deref()) {
            header.push_str("TELESCOP", tel, "Telescope or lens");
        }

        let focallen = eq.and_then(|e| e.focal_length).or(self.raw.focal_length);
        if let Some(fl) = focallen {
            header.push_float("FOCALLEN", fl, "Focal length [mm]");
        }

        let aperture = eq.and_then(|e| e.aperture).or(self.raw.aperture);
        if let Some(ap) = aperture.filter(|&a| a > 0.0) {
            header.push_float("APERTURE", ap, "Aperture diameter [mm]");
            header.push_float("APTDIA",   ap, "Aperture diameter [mm]");
            if let Some(fl) = focallen {
                header.push_float("FOCRATIO", fl / ap, "Focal ratio (f/number)");
            }
        }

        // Mount pointing
        if let Some(ra) = mnt.and_then(|m| m.ra_deg) {
            header.push_float("RA", ra, "Telescope pointing RA [deg]");
        }
        if let Some(dec) = mnt.and_then(|m| m.dec_deg) {
            header.push_float("DEC", dec, "Telescope pointing Dec [deg]");
        }
        if let Some(alt) = mnt.and_then(|m| m.altitude) {
            header.push_float("CENTALT", alt, "Telescope altitude [deg]");
        }
        if let Some(az) = mnt.and_then(|m| m.azimuth) {
            header.push_float("CENTAZ", az, "Telescope azimuth [deg]");
        }
        if let Some(am) = mnt.and_then(|m| m.airmass) {
            header.push_float("AIRMASS", am, "Airmass at frame center");
        }
        if let Some(ref ps) = mnt.and_then(|m| m.pier_side.clone()) {
            header.push_str("PIERSIDE", ps, "Pier side (East/West)");
        }

        // ── 6. Observer / Site ────────────────────────────────────────────────
        if let Some(ref obs) = self.session.observer {
            header.push_str("OBSERVER", obs, "Observer name");
        }
        if let Some(obs_name) = loc.and_then(|l| l.observatory_name.as_deref()) {
            header.push_str("OBSERVAT", obs_name, "Observatory name");
        }
        if let Some(lat) = loc.and_then(|l| l.latitude) {
            header.push_float("SITELAT", lat, "Site latitude [deg]");
        }
        if let Some(lon) = loc.and_then(|l| l.longitude) {
            header.push_float("SITELONG", lon, "Site longitude [deg]");
        }
        if let Some(elev) = loc.and_then(|l| l.elevation) {
            header.push_float("SITEELEV", elev, "Site elevation [m]");
        }
        if let Some(sn) = loc.and_then(|l| l.site_name.as_deref()) {
            header.push_str("SITENAME", sn, "Observatory site name");
        }

        // ── 7. Target / Object ────────────────────────────────────────────────
        let object = object_cli.map(|s| s.to_string())
            .or_else(|| file_override.and_then(|f| f.object.clone()))
            .or_else(|| self.session.object.clone())
            .or_else(|| tgt.and_then(|t| t.name.clone()));
        if let Some(ref obj) = object {
            header.push_str("OBJECT", obj, "Target object name");
        }
        if let Some(ra) = tgt.and_then(|t| t.ra_hms.as_deref()) {
            header.push_str("OBJCTRA", ra, "Target RA [H M S]");
        }
        if let Some(dec) = tgt.and_then(|t| t.dec_dms.as_deref()) {
            header.push_str("OBJCTDEC", dec, "Target Dec [D M S]");
        }
        if let Some(rot) = tgt.and_then(|t| t.rotation) {
            header.push_float("OBJCTROT", rot, "Target position angle [deg]");
        }

        // ── 8. Filter ─────────────────────────────────────────────────────────
        if let Some(fwn) = fw.and_then(|f| f.name.as_deref()) {
            header.push_str("FWHEEL", fwn, "Filter wheel name");
        }
        let filter = file_override.and_then(|f| f.filter.clone())
            .or_else(|| eq.and_then(|e| e.filter.clone()));
        if let Some(ref flt) = filter {
            header.push_str("FILTER", flt, "Filter name");
        }

        // ── 9. Focuser ────────────────────────────────────────────────────────
        if let Some(fn_) = foc.and_then(|f| f.name.as_deref()) {
            header.push_str("FOCNAME", fn_, "Focuser device name");
        }
        if let Some(fp) = foc.and_then(|f| f.position) {
            header.push_int("FOCPOS",   fp as i64, "Focuser position [steps]");
            header.push_int("FOCUSPOS", fp as i64, "Focuser position [steps] (alias)");
        }
        if let Some(fs) = foc.and_then(|f| f.step_size) {
            header.push_float("FOCUSSZ", fs, "Focuser step size [µm]");
        }
        if let Some(ft) = foc.and_then(|f| f.temperature) {
            header.push_float("FOCTEMP",  ft, "Focuser temperature [°C]");
            header.push_float("FOCUSTEM", ft, "Focuser temperature [°C] (alias)");
        }

        // ── 10. Rotator ───────────────────────────────────────────────────────
        if let Some(rn) = rot.and_then(|r| r.name.as_deref()) {
            header.push_str("ROTNAME", rn, "Rotator device name");
        }
        if let Some(ra) = rot.and_then(|r| r.mechanical_angle) {
            header.push_float("ROTATOR",  ra, "Rotator mechanical angle [deg]");
            header.push_float("ROTATANG", ra, "Rotator mechanical angle [deg] (alias)");
        }
        if let Some(rs) = rot.and_then(|r| r.step_size) {
            header.push_float("ROTSTPSZ", rs, "Rotator minimum step size [deg]");
        }

        // ── 11. Weather ───────────────────────────────────────────────────────
        if let Some(v) = wx.and_then(|w| w.cloud_cover)   { header.push_float("CLOUDCVR", v, "Cloud coverage [%]"); }
        if let Some(v) = wx.and_then(|w| w.dew_point)     { header.push_float("DEWPOINT", v, "Dew point [°C]"); }
        if let Some(v) = wx.and_then(|w| w.humidity)      { header.push_float("HUMIDITY", v, "Relative humidity [%]"); }
        if let Some(v) = wx.and_then(|w| w.pressure)      { header.push_float("PRESSURE", v, "Atmospheric pressure [hPa]"); }
        if let Some(v) = wx.and_then(|w| w.sky_brightness){ header.push_float("SKYBRGHT", v, "Sky brightness [lux]"); }
        if let Some(v) = wx.and_then(|w| w.mpsas)         { header.push_float("MPSAS",    v, "Sky quality [mag/arcsec²]"); }
        if let Some(v) = wx.and_then(|w| w.sky_temp)      { header.push_float("SKYTEMP",  v, "Sky temperature [°C]"); }
        if let Some(v) = wx.and_then(|w| w.star_fwhm)     { header.push_float("STARFWHM", v, "Star FWHM [arcsec]"); }
        if let Some(v) = wx.and_then(|w| w.ambient_temp)  { header.push_float("AMBTEMP",  v, "Ambient temperature [°C]"); }
        if let Some(v) = wx.and_then(|w| w.wind_direction){ header.push_float("WINDDIR",  v, "Wind direction [deg, 0=N]"); }
        if let Some(v) = wx.and_then(|w| w.wind_gust)     { header.push_float("WINDGUST", v, "Wind gust speed [kph]"); }
        if let Some(v) = wx.and_then(|w| w.wind_speed)    { header.push_float("WINDSPD",  v, "Wind speed [kph]"); }

        // ── 12. WCS (plate solver) ────────────────────────────────────────────
        if let Some(w) = wcs {
            if let Some(ref v) = w.ctype1  { header.push_str("CTYPE1", v, "Coordinate type axis 1"); }
            if let Some(ref v) = w.ctype2  { header.push_str("CTYPE2", v, "Coordinate type axis 2"); }
            if let Some(v) = w.crpix1      { header.push_float("CRPIX1", v, "Reference pixel X"); }
            if let Some(v) = w.crpix2      { header.push_float("CRPIX2", v, "Reference pixel Y"); }
            if let Some(v) = w.crval1      { header.push_float("CRVAL1", v, "Reference RA [deg]"); }
            if let Some(v) = w.crval2      { header.push_float("CRVAL2", v, "Reference Dec [deg]"); }
            // CDn_m matrix (modern)
            if let Some(v) = w.cd1_1 { header.push_float("CD1_1", v, "WCS matrix [1,1]"); }
            if let Some(v) = w.cd1_2 { header.push_float("CD1_2", v, "WCS matrix [1,2]"); }
            if let Some(v) = w.cd2_1 { header.push_float("CD2_1", v, "WCS matrix [2,1]"); }
            if let Some(v) = w.cd2_2 { header.push_float("CD2_2", v, "WCS matrix [2,2]"); }
            // CDELT/CROTA2 (legacy, written for compatibility)
            if let Some(v) = w.cdelt1  { header.push_float("CDELT1", v, "Pixel scale RA [deg/px]"); }
            if let Some(v) = w.cdelt2  { header.push_float("CDELT2", v, "Pixel scale Dec [deg/px]"); }
            if let Some(v) = w.crota2  { header.push_float("CROTA2", v, "Image rotation [deg]"); }
        }

        // ── HISTORY ───────────────────────────────────────────────────────────
        let write_history = out.map(|o| o.write_history).unwrap_or(true);
        if write_history {
            let hist = format!(
                "Converted from RAW by AstroRAW-o-Matic v{} on {}",
                env!("CARGO_PKG_VERSION"),
                Utc::now().format("%Y-%m-%dT%H:%M:%SZ")
            );
            header.push_str("HISTORY", &hist, ());
        }

        ResolvedMetadata { header, warnings }
    }

    fn push_swcreate(&self, header: &mut FitsHeader) {
        let sw = format!("AstroRAW-o-Matic v{}", env!("CARGO_PKG_VERSION"));
        header.push_str("SWCREATE", &sw, "Software that created this file");
    }
}
