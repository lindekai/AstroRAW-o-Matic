use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Metadata extracted directly from the RAW file (EXIF etc.)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RawMetadata {
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub exposure_time: Option<f64>,
    pub iso_speed: Option<u32>,
    pub date_obs: Option<DateTime<Utc>>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub bit_depth: Option<u8>,
    pub bayer_pattern: Option<String>,
    pub black_level: Option<u32>,
    pub white_level: Option<u32>,
    pub focal_length: Option<f64>,
    pub aperture: Option<f64>,
    pub wb_coeffs: Option<[f32; 4]>,
}

/// Camera-specific metadata (from session JSON or CLI)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CameraMetadata {
    pub make: Option<String>,
    pub model: Option<String>,
    pub camera_id: Option<String>,
    pub pixel_size_x: Option<f64>,
    pub pixel_size_y: Option<f64>,
    pub gain: Option<i32>,
    pub offset: Option<i32>,
    pub egain: Option<f64>,
    pub binning_x: Option<u32>,
    pub binning_y: Option<u32>,
    pub set_temp: Option<f64>,
    pub ccd_temp: Option<f64>,
    pub readout_mode: Option<String>,
    pub usb_limit: Option<i32>,
    pub bayer_x_offset: Option<i32>,
    pub bayer_y_offset: Option<i32>,
}

/// Equipment metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EquipmentMetadata {
    pub telescope: Option<String>,
    pub focal_length: Option<f64>,
    pub aperture: Option<f64>,
    pub filter: Option<String>,
    pub camera: Option<CameraMetadata>,
    pub focuser: Option<FocuserMetadata>,
    pub rotator: Option<RotatorMetadata>,
    pub filter_wheel: Option<FilterWheelMetadata>,
    pub mount: Option<MountMetadata>,
}

/// Focuser metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FocuserMetadata {
    pub name: Option<String>,
    pub position: Option<i32>,
    pub step_size: Option<f64>,
    pub temperature: Option<f64>,
}

/// Rotator metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RotatorMetadata {
    pub name: Option<String>,
    pub mechanical_angle: Option<f64>,
    pub step_size: Option<f64>,
}

/// Filter wheel metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilterWheelMetadata {
    pub name: Option<String>,
}

/// Mount / telescope pointing metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MountMetadata {
    pub ra_deg: Option<f64>,
    pub dec_deg: Option<f64>,
    pub altitude: Option<f64>,
    pub azimuth: Option<f64>,
    pub pier_side: Option<String>,
    pub airmass: Option<f64>,
}

/// Observation site / location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocationMetadata {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub elevation: Option<f64>,
    pub site_name: Option<String>,
    pub observatory_name: Option<String>,
}

/// Target / object coordinates
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TargetMetadata {
    pub name: Option<String>,
    pub ra_hms: Option<String>,
    pub dec_dms: Option<String>,
    pub rotation: Option<f64>,
}

/// Weather / environment metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WeatherMetadata {
    pub cloud_cover: Option<f64>,
    pub dew_point: Option<f64>,
    pub humidity: Option<f64>,
    pub pressure: Option<f64>,
    pub sky_brightness: Option<f64>,
    pub mpsas: Option<f64>,
    pub sky_temp: Option<f64>,
    pub star_fwhm: Option<f64>,
    pub ambient_temp: Option<f64>,
    pub wind_direction: Option<f64>,
    pub wind_gust: Option<f64>,
    pub wind_speed: Option<f64>,
}

/// WCS (World Coordinate System) — from plate solver
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WcsMetadata {
    pub ctype1: Option<String>,
    pub ctype2: Option<String>,
    pub crpix1: Option<f64>,
    pub crpix2: Option<f64>,
    pub crval1: Option<f64>,
    pub crval2: Option<f64>,
    pub cd1_1: Option<f64>,
    pub cd1_2: Option<f64>,
    pub cd2_1: Option<f64>,
    pub cd2_2: Option<f64>,
    pub cdelt1: Option<f64>,
    pub cdelt2: Option<f64>,
    pub crota2: Option<f64>,
}
