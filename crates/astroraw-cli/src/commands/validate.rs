use astroraw_core::{load_session_file, session::validator::validate_session};
use crate::args::ValidateArgs;

pub fn run(args: ValidateArgs) -> i32 {
    match load_session_file(&args.input) {
        Ok(session) => {
            let report = validate_session(&session);

            if report.warnings.is_empty() && report.errors.is_empty() {
                eprintln!("  Session JSON is valid. The Guide would approve.");
            }

            for warn in &report.warnings {
                eprintln!("  [WARN] {}", warn);
            }

            for err in &report.errors {
                eprintln!("  [ERROR] {}", err);
            }

            if report.is_valid() { 0 } else { 1 }
        }
        Err(e) => {
            eprintln!("  {}", e);
            1
        }
    }
}
