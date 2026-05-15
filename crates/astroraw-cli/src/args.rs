use clap::{Parser, Subcommand};
use std::path::PathBuf;

const BANNER: &str = "\
AstroRAW-o-Matic
Mostly harmless RAW conversion.\n";

#[derive(Parser, Debug)]
#[command(
    name = "astroraw-o-matic",
    author,
    version,
    about = BANNER,
    long_about = None,
    after_help = "Aliases: arom\n\nFor more information see https://github.com/lindekai/AstroRAW-o-Matic"
)]
pub struct Cli {
    /// Enable verbose debug output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Write log output as JSON (useful for machine parsing)
    #[arg(long, global = true, hide = true)]
    pub log_json: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Convert RAW file(s) to FITS
    Convert(ConvertArgs),

    /// Inspect a RAW file and display its metadata
    Inspect(InspectArgs),

    /// Validate a session JSON file
    Validate(ValidateArgs),
}

// ── convert ──────────────────────────────────────────────────────────────────

#[derive(Parser, Debug)]
pub struct ConvertArgs {
    /// Input: a single RAW file or a directory of RAW files
    pub input: PathBuf,

    /// Output directory for FITS files
    #[arg(short, long, value_name = "DIR")]
    pub output: PathBuf,

    /// Session metadata JSON file
    #[arg(short, long, value_name = "FILE")]
    pub metadata: Option<PathBuf>,

    /// Recurse into subdirectories
    #[arg(short, long)]
    pub recursive: bool,

    /// Frame type (light, dark, flat, bias)
    #[arg(long, value_name = "TYPE")]
    pub r#type: Option<String>,

    /// Target object name (e.g. M31, NGC 7000)
    #[arg(long)]
    pub object: Option<String>,

    /// Telescope or lens name
    #[arg(long)]
    pub telescope: Option<String>,

    /// Filter name (e.g. L, Ha, OIII)
    #[arg(long)]
    pub filter: Option<String>,

    /// Observer name
    #[arg(long)]
    pub observer: Option<String>,

    /// Override observation date/time (ISO 8601: "2024-01-09T21:34:00")
    #[arg(long, value_name = "DATETIME")]
    pub date_obs: Option<String>,

    /// Header mode: minimal or astro (default: astro)
    #[arg(long, default_value = "astro")]
    pub header_mode: String,

    /// Overwrite existing FITS files
    #[arg(long)]
    pub overwrite: bool,

    /// Show what would happen without writing any files
    #[arg(long)]
    pub dry_run: bool,

    /// Write log output to a file
    #[arg(long, value_name = "FILE")]
    pub log_file: Option<PathBuf>,
}

// ── inspect ───────────────────────────────────────────────────────────────────

#[derive(Parser, Debug)]
pub struct InspectArgs {
    /// RAW file to inspect
    pub input: PathBuf,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

// ── validate ──────────────────────────────────────────────────────────────────

#[derive(Parser, Debug)]
pub struct ValidateArgs {
    /// Session JSON file to validate
    pub input: PathBuf,
}
