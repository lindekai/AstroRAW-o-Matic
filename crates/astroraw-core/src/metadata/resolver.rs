use astroraw_models::{FitsHeader, FrameType, RawMetadata, SessionMetadata};
use chrono::Utc;
use tracing::warn;

/// The fully resolved output of metadata merging for one file.
pub struct ResolvedMetadata {
    pub header: FitsHeader,
    pub warnings: Vec<String>,
}

/// Merges metadata from four sources with clear priority:
///   1. file-specific override (session.file_overrides[filename])
///   2. session-level JSON
///   3. RAW EXIF / camera data
///   4. hard defaults
pub struct MetadataResolver<'a> {
    session: &'a SessionMetadata,
    raw: RawMetadata,
    filename: String,
}

impl<'a> MetadataResolver<'a> {
    pub fn new(session: &'a SessionMetadata, raw: RawMetadata, filename: &str) -> Self {
        Self {
            session,
            raw,
            filename: filename.to_string(),
        }
    }

    pub fn resolve(
        &self,
        frame_type_cli: Option<FrameType>,
        object_cli: Option<&str>,
    ) -> ResolvedMetadata {
        let mut header = FitsHeader::new();
        let mut warnings = Vec::new();

        let file_override = self.session.file_overrides.get(&self.filename);
        let eq = self.session.equipment.as_ref();
        let loc = self.session.location.as_ref();
        let out = self.session.output.as_ref();

        // --- Mandatory FITS keywords ---
        header.push_bool("SIMPLE", true, "Conforms to FITS standard");
        header.push_int("BITPIX", 16, "16-bit unsigned integers");
        header.push_int("NAXIS", 2, "Number of array dimensions");

        let width = self.raw.width.unwrap_or(0) as i64;
        let height = self.raw.height.unwrap_or(0) as i64;
        header.push_int("NAXIS1", width, "Image width in pixels");
        header.push_int("NAXIS2", height, "Image height in pixels");
        header.push_bool("EXTEND", true, "Extensions are permitted");

        // --- Camera / instrument ---
        let instrument = self.raw.camera_model.clone()
            .or_else(|| eq.and_then(|e| e.camera.as_ref()).and_then(|c| c.model.clone()))
            .unwrap_or_else(|| {
                warnings.push("INSTRUME unknown. Specify camera model in session JSON.".to_string());
                "Unknown".to_string()
            });
        header.push_str("INSTRUME", &instrument, "Camera / instrument");

        // --- Exposure time ---
        let exptime = self.raw.exposure_time;
        match exptime {
            Some(t) => header.push_float("EXPTIME", t, "Exposure time [s]"),
            None => {
                warnings.push(
                    "The universe refused to provide EXPTIME. Please supply it via metadata JSON.".to_string(),
                );
                warn!("EXPTIME missing for {}", self.filename);
            }
        }

        // --- ISO ---
        match self.raw.iso_speed {
            Some(iso) => header.push_int("ISOSPEED", iso as i64, "ISO/gain setting"),
            None => {
                warnings.push("ISOSPEED not found in RAW metadata.".to_string());
            }
        }

        // --- Date of observation — priority: file_override > session > EXIF ---
        let date_obs = file_override.and_then(|f| f.date_obs.clone())
            .or_else(|| self.session.date_obs.clone())
            .or_else(|| self.raw.date_obs.map(|d| d.format("%Y-%m-%dT%H:%M:%S").to_string()));
        match date_obs {
            Some(ref d) => header.push_str("DATE-OBS", d, "Date and time of observation (UTC)"),
            None => {
                warnings.push("DATE-OBS could not be determined. Supply it via --date-obs or session JSON.".to_string());
            }
        }

        // --- Bayer / CFA ---
        let header_mode = out.map(|o| o.header_mode.as_str()).unwrap_or("astro");
        if header_mode == "astro" {
            let bayer = self.raw.bayer_pattern.clone().unwrap_or_else(|| {
                warnings.push(
                    "Don't panic. Could not determine Bayer pattern. \
                     BAYERPAT will be omitted; debayering would be a bad idea right now."
                        .to_string(),
                );
                "UNKNOWN".to_string()
            });

            if bayer != "UNKNOWN" {
                header.push_str("BAYERPAT", &bayer, "Bayer color filter array pattern");
                header.push_str("CFATYPE", &bayer, "Color filter array type");
            }

            header.push_str("COLORTYP", "RAW", "Color type: RAW Bayer mosaic");

            // Pixel size (µm) — Canon 600D: 4.3µm
            let px = eq.and_then(|e| e.camera.as_ref()).and_then(|c| c.pixel_size_x).unwrap_or(4.3);
            let py = eq.and_then(|e| e.camera.as_ref()).and_then(|c| c.pixel_size_y).unwrap_or(4.3);
            header.push_float("XPIXSZ", px, "Pixel width [micron]");
            header.push_float("YPIXSZ", py, "Pixel height [micron]");

            // Bit depth
            let bitdepth = self.raw.bit_depth.unwrap_or(14);
            header.push_int("BITDEPTH", bitdepth as i64, "ADC bit depth of sensor");

            // Frame type — priority: CLI > file_override > session
            let frame_type = frame_type_cli
                .or_else(|| file_override.and_then(|f| f.frame_type))
                .or(self.session.frame_type)
                .unwrap_or(FrameType::Light);
            header.push_str("IMAGETYP", frame_type.fits_imagetyp(), "Frame type");

            // Object — priority: CLI > file_override > session
            let object = object_cli
                .map(|s| s.to_string())
                .or_else(|| file_override.and_then(|f| f.object.clone()))
                .or_else(|| self.session.object.clone());
            if let Some(ref obj) = object {
                header.push_str("OBJECT", obj, "Target object name");
            }

            // Telescope
            if let Some(tel) = eq.and_then(|e| e.telescope.as_ref()) {
                header.push_str("TELESCOP", tel, "Telescope or lens");
            }

            // Focal length — session JSON overrides EXIF (important for telescopes)
            let focallen = eq.and_then(|e| e.focal_length).or(self.raw.focal_length);
            if let Some(fl) = focallen {
                header.push_float("FOCALLEN", fl, "Focal length [mm]");
            }

            // Aperture — skip if 0 or negative (EXIF sometimes returns 0 for manual lenses)
            let aperture = eq.and_then(|e| e.aperture).or(self.raw.aperture);
            if let Some(ap) = aperture {
                if ap > 0.0 {
                    header.push_float("APERTURE", ap, "Aperture f-number");
                }
            }

            // Filter
            let filter = file_override
                .and_then(|f| f.filter.clone())
                .or_else(|| eq.and_then(|e| e.filter.clone()));
            if let Some(ref flt) = filter {
                header.push_str("FILTER", flt, "Filter used");
            }

            // Observer
            if let Some(ref obs) = self.session.observer {
                header.push_str("OBSERVER", obs, "Observer name");
            }

            // Location
            if let Some(loc) = loc {
                if let Some(lat) = loc.latitude {
                    header.push_float("SITELAT", lat, "Site latitude [deg]");
                }
                if let Some(lon) = loc.longitude {
                    header.push_float("SITELONG", lon, "Site longitude [deg]");
                }
                if let Some(elev) = loc.elevation {
                    header.push_float("SITEELEV", elev, "Site elevation [m]");
                }
            }

            // Black/White levels
            if let Some(bl) = self.raw.black_level {
                header.push_int("BLACKLVL", bl as i64, "Black level (ADU)");
                // PEDESTAL is the keyword Siril reads for black level subtraction
                header.push_int("PEDESTAL", bl as i64, "Black level pedestal (ADU)");
            }
            if let Some(wl) = self.raw.white_level {
                header.push_int("WHITELEV", wl as i64, "White/saturation level (ADU)");
            }

            // As-shot white balance coefficients (RGBE order, normalised to G=1)
            if let Some(wb) = self.raw.wb_coeffs {
                let [r, g, b, _g2] = wb;
                if !r.is_nan() && !g.is_nan() && !b.is_nan() && g != 0.0 {
                    header.push_float("CBLACK_R", (r / g) as f64, "WB coefficient Red (normalised to G)");
                    header.push_float("CBLACK_G", 1.0_f64,         "WB coefficient Green (reference)");
                    header.push_float("CBLACK_B", (b / g) as f64, "WB coefficient Blue (normalised to G)");
                }
            }
        }

        // --- Software creator ---
        let sw = format!(
            "AstroRAW-o-Matic v{}",
            env!("CARGO_PKG_VERSION")
        );
        header.push_str("SWCREATE", &sw, "Software that created this file");

        // --- HISTORY ---
        let write_history = out.map(|o| o.write_history).unwrap_or(true);
        if write_history {
            let hist = format!(
                "Converted from RAW by AstroRAW-o-Matic on {}",
                Utc::now().format("%Y-%m-%dT%H:%M:%SZ")
            );
            header.push_str("HISTORY", &hist, ());
        }

        ResolvedMetadata { header, warnings }
    }
}
