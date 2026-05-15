mod args;
mod commands;
mod display;

use clap::Parser;
use std::process;

use args::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    // Init logging before any output
    astroraw_core::logging::init_logging(cli.verbose, cli.log_json);

    let exit_code = match cli.command {
        Commands::Convert(args) => commands::convert::run(args, cli.verbose),
        Commands::Inspect(args) => commands::inspect::run(args),
        Commands::Validate(args) => commands::validate::run(args),
    };

    process::exit(exit_code);
}
