use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};

use astroraw_core::{
    convert_batch, load_session_file,
    raw::reader::RawReader,
    session::validator::validate_session as core_validate,
    ConvertRequest as CoreConvertRequest,
};
use astroraw_models::SessionMetadata;

// ── Inspect ───────────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct InspectResult {
    pub success: bool,
    pub error: Option<String>,
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub exposure_time: Option<f64>,
    pub iso_speed: Option<u32>,
    pub date_obs: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub bit_depth: Option<u8>,
    pub bayer_pattern: Option<String>,
    pub focal_length: Option<f64>,
    pub aperture: Option<f64>,
}

#[tauri::command]
pub fn inspect_file(path: String) -> InspectResult {
    match RawReader::read_metadata(&PathBuf::from(&path)) {
        Ok(meta) => InspectResult {
            success: true,
            error: None,
            camera_make: meta.camera_make,
            camera_model: meta.camera_model,
            exposure_time: meta.exposure_time,
            iso_speed: meta.iso_speed,
            date_obs: meta.date_obs.map(|d| d.format("%Y-%m-%dT%H:%M:%S").to_string()),
            width: meta.width,
            height: meta.height,
            bit_depth: meta.bit_depth,
            bayer_pattern: meta.bayer_pattern,
            focal_length: meta.focal_length,
            aperture: meta.aperture,
        },
        Err(e) => InspectResult {
            success: false,
            error: Some(e.to_string()),
            camera_make: None, camera_model: None, exposure_time: None,
            iso_speed: None, date_obs: None, width: None, height: None,
            bit_depth: None, bayer_pattern: None, focal_length: None, aperture: None,
        },
    }
}

// ── Convert ───────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct GuiConvertRequest {
    pub input_paths: Vec<String>,
    pub output_dir: String,
    pub session: SessionMetadata,
    pub overwrite: bool,
    pub dry_run: bool,
}

#[derive(Serialize)]
pub struct ConvertSummary {
    pub total: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub results: Vec<FileResult>,
}

#[derive(Serialize)]
pub struct FileResult {
    pub input: String,
    pub output: String,
    pub success: bool,
    pub warnings: Vec<String>,
    pub error: Option<String>,
}

#[tauri::command]
pub fn convert_files(request: GuiConvertRequest) -> ConvertSummary {
    let mut session = request.session;
    session.output.as_mut().map(|o| o.overwrite = request.overwrite);

    let core_req = CoreConvertRequest {
        inputs: request.input_paths.iter().map(PathBuf::from).collect(),
        output_dir: PathBuf::from(&request.output_dir),
        session,
        frame_type_override: None,
        object_override: None,
        dry_run: request.dry_run,
        overwrite: request.overwrite,
    };

    let summary = convert_batch(&core_req);

    ConvertSummary {
        total: summary.total,
        succeeded: summary.succeeded,
        failed: summary.failed,
        results: summary.results.into_iter().map(|r| FileResult {
            input: r.input.display().to_string(),
            output: r.output.display().to_string(),
            success: r.success,
            warnings: r.warnings,
            error: r.error,
        }).collect(),
    }
}

// ── Session JSON Load / Save ──────────────────────────────────────────────────

#[tauri::command]
pub fn load_session_json(path: String) -> Result<SessionMetadata, String> {
    load_session_file(&PathBuf::from(&path))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_session_json(path: String, session: SessionMetadata) -> Result<(), String> {
    let json = serde_json::to_string_pretty(&session)
        .map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())
}

// ── Validate ──────────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

#[tauri::command]
pub fn validate_session_file(path: String) -> ValidationResult {
    match load_session_file(&PathBuf::from(&path)) {
        Ok(session) => {
            let report = core_validate(&session);
            ValidationResult {
                valid: report.is_valid(),
                warnings: report.warnings,
                errors: report.errors,
            }
        }
        Err(e) => ValidationResult {
            valid: false,
            warnings: vec![],
            errors: vec![e.to_string()],
        },
    }
}
