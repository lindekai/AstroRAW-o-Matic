use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    EquipmentMetadata, LocationMetadata, FrameType,
    TargetMetadata, WeatherMetadata, WcsMetadata,
};

/// Top-level session metadata — the "contract" between CLI, engine, GUI and N.I.N.A. plugin.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionMetadata {
    #[serde(default = "default_schema_version")]
    pub schema_version: String,

    // --- Observation context ---
    pub object: Option<String>,
    pub observer: Option<String>,
    pub notes: Option<String>,

    // --- Session date/time ---
    pub session_date: Option<String>,
    pub session_time: Option<String>,
    pub date_obs: Option<String>,

    // --- Equipment ---
    pub equipment: Option<EquipmentMetadata>,

    // --- Location ---
    pub location: Option<LocationMetadata>,

    // --- Target coordinates ---
    pub target: Option<TargetMetadata>,

    // --- Weather / environment ---
    pub weather: Option<WeatherMetadata>,

    // --- WCS (from plate solver) ---
    pub wcs: Option<WcsMetadata>,

    // --- Frame classification ---
    pub frame_type: Option<FrameType>,

    // --- Output options ---
    pub output: Option<OutputOptions>,

    // --- Per-file overrides ---
    #[serde(default)]
    pub file_overrides: HashMap<String, FileOverride>,
}

fn default_schema_version() -> String {
    "1.1".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputOptions {
    #[serde(default = "default_raw_mode")]
    pub raw_mode: String,
    #[serde(default = "default_header_mode")]
    pub header_mode: String,
    #[serde(default)]
    pub overwrite: bool,
    #[serde(default = "default_true")]
    pub write_history: bool,
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

fn default_raw_mode() -> String    { "raw_bayer".to_string() }
fn default_header_mode() -> String { "astro".to_string() }
fn default_true() -> bool          { true }

/// Per-file overrides — highest priority in metadata chain.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileOverride {
    pub object: Option<String>,
    pub frame_type: Option<FrameType>,
    pub filter: Option<String>,
    pub notes: Option<String>,
    pub date_obs: Option<String>,
}
