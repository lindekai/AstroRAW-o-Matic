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
    /// As-shot WB coefficients [R, G, B, G2] // None if not available
    pub wb_coeffs: Option<[f32; 4]>,
}

/// Camera-specific metadata (from session JSON or CLI)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CameraMetadata {
    pub make: Option<String>,
    pub model: Option<String>,
    pub pixel_size_x: Option<f64>,
    pub pixel_size_y: Option<f64>,
}

/// Equipment metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EquipmentMetadata {
    pub telescope: Option<String>,
    pub focal_length: Option<f64>,
    pub aperture: Option<f64>,
    pub filter: Option<String>,
    pub camera: Option<CameraMetadata>,
}

/// Observation site / location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocationMetadata {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub elevation: Option<f64>,
    pub site_name: Option<String>,
}
