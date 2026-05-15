pub mod error;
pub mod fits;
pub mod logging;
pub mod metadata;
pub mod raw;
pub mod session;

// Re-export session validator for CLI use
pub use session::validator;

pub use error::{AstroError, Result};

use std::path::{Path, PathBuf};
use tracing::{info, warn};
use walkdir::WalkDir;

use astroraw_models::{SessionMetadata, FrameType};
use crate::{
    fits::writer::FitsWriter,
    metadata::resolver::MetadataResolver,
    raw::reader::RawReader,
    session::loader::load_session,
};

/// Top-level conversion request — everything the engine needs for one batch.
#[derive(Debug, Clone)]
pub struct ConvertRequest {
    pub inputs: Vec<PathBuf>,
    pub output_dir: PathBuf,
    pub session: SessionMetadata,
    /// Override frame type from CLI
    pub frame_type_override: Option<FrameType>,
    /// Override object name from CLI
    pub object_override: Option<String>,
    pub dry_run: bool,
    pub overwrite: bool,
}

/// Result for a single file conversion.
#[derive(Debug)]
pub struct ConvertResult {
    pub input: PathBuf,
    pub output: PathBuf,
    pub success: bool,
    pub warnings: Vec<String>,
    pub error: Option<String>,
}

/// Batch conversion summary.
#[derive(Debug, Default)]
pub struct BatchSummary {
    pub total: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub skipped: usize,
    pub results: Vec<ConvertResult>,
}

/// Convert a batch of RAW files to FITS.
pub fn convert_batch(request: &ConvertRequest) -> BatchSummary {
    let mut summary = BatchSummary::default();
    summary.total = request.inputs.len();

    for input in &request.inputs {
        let result = convert_single(input, request);
        if result.success {
            summary.succeeded += 1;
        } else {
            summary.failed += 1;
        }
        summary.results.push(result);
    }

    summary
}

/// Convert a single RAW file.
pub fn convert_single(input: &Path, request: &ConvertRequest) -> ConvertResult {
    let stem = input.file_stem().unwrap_or_default().to_string_lossy();
    let output = request.output_dir.join(format!("{}.fits", stem));

    let mut warnings = Vec::new();

    if output.exists() && !request.overwrite && !request.dry_run {
        return ConvertResult {
            input: input.to_path_buf(),
            output,
            success: false,
            warnings,
            error: Some(format!(
                "Output file already exists. Use --overwrite to replace it."
            )),
        };
    }

    // 1. Read RAW metadata (EXIF)
    let mut raw_meta = match RawReader::read_metadata(input) {
        Ok(m) => m,
        Err(e) => {
            warn!("Could not read RAW metadata from {}: {}", input.display(), e);
            warnings.push(format!("Could not read RAW metadata: {}", e));
            Default::default()
        }
    };

    if request.dry_run {
        let filename = input.file_name().unwrap_or_default().to_string_lossy().to_string();
        let resolver = MetadataResolver::new(&request.session, raw_meta, &filename);
        let resolved = resolver.resolve(request.frame_type_override, request.object_override.as_deref());
        warnings.extend(resolved.warnings);
        info!("[dry-run] Would write: {}", output.display());
        return ConvertResult {
            input: input.to_path_buf(),
            output,
            success: true,
            warnings,
            error: None,
        };
    }

    // 2. Read RAW pixel data (rawler) — provides real Bayer data + black/white levels
    let pixel_data = match RawReader::read_raw_bayer(input) {
        Ok(d) => d,
        Err(e) => {
            return ConvertResult {
                input: input.to_path_buf(),
                output,
                success: false,
                warnings,
                error: Some(format!("Failed to read RAW pixel data: {}", e)),
            };
        }
    };

    // 3. Feed rawler values back into metadata — always use actual pixel dimensions
    //    to avoid stride mismatch between FITS header and data (causes stripes).
    raw_meta.width = Some(pixel_data.width);
    raw_meta.height = Some(pixel_data.height);
    raw_meta.black_level = Some(pixel_data.black_level as u32);
    raw_meta.white_level = Some(pixel_data.white_level as u32);
    if raw_meta.bayer_pattern.is_none() {
        raw_meta.bayer_pattern = pixel_data.bayer_pattern.clone();
    }

    // 4. Resolve merged metadata
    let filename = input.file_name().unwrap_or_default().to_string_lossy().to_string();
    let resolver = MetadataResolver::new(&request.session, raw_meta, &filename);
    let resolved = resolver.resolve(request.frame_type_override, request.object_override.as_deref());
    warnings.extend(resolved.warnings.clone());

    // 5. Write FITS
    let writer = FitsWriter::new();
    match writer.write(&output, &resolved.header, &pixel_data) {
        Ok(_) => {
            info!("Converted: {} -> {}", input.display(), output.display());
            ConvertResult {
                input: input.to_path_buf(),
                output,
                success: true,
                warnings,
                error: None,
            }
        }
        Err(e) => ConvertResult {
            input: input.to_path_buf(),
            output,
            success: false,
            warnings,
            error: Some(format!("Failed to write FITS: {}", e)),
        },
    }
}

/// Collect RAW files from a path (file or directory).
pub fn collect_raw_files(path: &Path, recursive: bool) -> Vec<PathBuf> {
    if path.is_file() {
        return vec![path.to_path_buf()];
    }

    let walker = if recursive {
        WalkDir::new(path)
    } else {
        WalkDir::new(path).max_depth(1)
    };

    walker
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| is_supported_raw(e.path()))
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn is_supported_raw(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => matches!(ext.to_uppercase().as_str(), "CR2" | "CR3" | "NEF" | "ARW" | "RAF"),
        None => false,
    }
}

/// Load session metadata from a JSON file.
pub fn load_session_file(path: &Path) -> Result<SessionMetadata> {
    load_session(path)
}
