use std::path::PathBuf;

use astroraw_core::{collect_raw_files, convert_batch, load_session_file, ConvertRequest};
use astroraw_models::{FrameType, SessionMetadata, OutputOptions};

use crate::args::ConvertArgs;
use crate::display;

pub fn run(args: ConvertArgs, verbose: bool) -> i32 {
    display::section_header("Converting RAW files");

    if args.dry_run {
        eprintln!("  [dry-run] No files will be written.");
    }

    // Load session metadata
    let mut session = if let Some(ref metadata_path) = args.metadata {
        match load_session_file(metadata_path) {
            Ok(s) => {
                if verbose {
                    eprintln!("  Loaded session metadata from {}", metadata_path.display());
                }
                s
            }
            Err(e) => {
                eprintln!("  Error: {}", e);
                return 1;
            }
        }
    } else {
        SessionMetadata::default()
    };

    // Apply CLI overrides to session
    apply_cli_overrides(&mut session, &args);

    // Collect input files
    let files = collect_raw_files(&args.input, args.recursive);
    if files.is_empty() {
        eprintln!(
            "  Nothing to convert. Space is big, but this input folder is empty: {}",
            args.input.display()
        );
        return 1;
    }

    eprintln!("  Found {} RAW file(s) to convert.", files.len());

    let frame_type_override = args.r#type.as_deref().and_then(parse_frame_type);
    let object_override = args.object.clone()
        .or_else(|| session.object.clone());

    let request = ConvertRequest {
        inputs: files,
        output_dir: args.output.clone(),
        session,
        frame_type_override,
        object_override,
        dry_run: args.dry_run,
        overwrite: args.overwrite,
    };

    let summary = convert_batch(&request);

    eprintln!();
    display::batch_summary(&summary);

    if summary.failed > 0 {
        for result in &summary.results {
            if !result.success {
                if let Some(ref err) = result.error {
                    eprintln!("  [FAIL] {}: {}", result.input.display(), err);
                }
            }
            for warn in &result.warnings {
                eprintln!("  [WARN] {}: {}", result.input.display(), warn);
            }
        }
        1
    } else {
        0
    }
}

fn apply_cli_overrides(session: &mut SessionMetadata, args: &ConvertArgs) {
    if let Some(ref obs) = args.observer {
        session.observer = Some(obs.clone());
    }

    // Ensure output options exist
    let out = session.output.get_or_insert_with(OutputOptions::default);
    out.header_mode = args.header_mode.clone();
    if args.overwrite {
        out.overwrite = true;
    }
}

fn parse_frame_type(s: &str) -> Option<FrameType> {
    match s.to_lowercase().as_str() {
        "light" | "l" => Some(FrameType::Light),
        "dark" | "d" => Some(FrameType::Dark),
        "flat" | "f" => Some(FrameType::Flat),
        "bias" | "b" => Some(FrameType::Bias),
        _ => {
            eprintln!(
                "  Warning: Unknown frame type '{}'. Using 'light'. \
                 Valid options: light, dark, flat, bias",
                s
            );
            None
        }
    }
}
