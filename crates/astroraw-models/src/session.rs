use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{EquipmentMetadata, LocationMetadata, FrameType};

/// Top-level session metadata — the "contract" between CLI, engine and future GUI.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionMetadata {
    /// Schema version for forward-compatibility checks
    #[serde(default = "default_schema_version")]
    pub schema_version: String,

    // --- Observation context ---
    pub object: Option<String>,
    pub observer: Option<String>,
    pub notes: Option<String>,

    // --- Session date/time (used as DATE-OBS if no EXIF; pre-filled from EXIF in GUI) ---
    /// Session date: "2024-01-09"
    pub session_date: Option<String>,
    /// Session time (UTC): "21:34:00"
    pub session_time: Option<String>,
    /// Full DATE-OBS override — takes priority over session_date + session_time + EXIF
    pub date_obs: Option<String>,

    // --- Equipment ---
    pub equipment: Option<EquipmentMetadata>,

    // --- Location ---
    pub location: Option<LocationMetadata>,

    // --- Frame classification ---
    pub frame_type: Option<FrameType>,

    // --- Output options ---
    pub output: Option<OutputOptions>,

    // --- Per-file overrides ---
    /// Keyed by filename (basename, e.g. "IMG_0042.CR2")
    #[serde(default)]
    pub file_overrides: HashMap<String, FileOverride>,
}

fn default_schema_version() -> String {
    "1.0".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputOptions {
    /// "raw_bayer" (default) or "debayered" (future)
    #[serde(default = "default_raw_mode")]
    pub raw_mode: String,

    /// "minimal" or "astro" (default)
    #[serde(default = "default_header_mode")]
    pub header_mode: String,

    /// Overwrite existing FITS files
    #[serde(default)]
    pub overwrite: bool,

    /// Append HISTORY records to FITS header
    #[serde(default = "default_true")]
    pub write_history: bool,

    /// Filename pattern for exported JSON (GUI only). Placeholders: {object}, {date}, {observer}
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_filename_pattern: Option<String>,
}

impl Default for OutputOptions {
    fn default() -> Self {
        Self {
            raw_mode: default_raw_mode(),
            header_mode: default_header_mode(),
            overwrite: false,
            write_history: true,
            json_filename_pattern: None,
        }
    }
}

fn default_raw_mode() -> String {
    "raw_bayer".to_string()
}

fn default_header_mode() -> String {
    "astro".to_string()
}

fn default_true() -> bool {
    true
}

/// Per-file overrides that take priority over session-level values.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileOverride {
    pub object: Option<String>,
    pub frame_type: Option<FrameType>,
    pub filter: Option<String>,
    pub notes: Option<String>,
    pub date_obs: Option<String>,
}
