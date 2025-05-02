mod commands;
mod logger;
mod manager;
mod modrinth;
mod output;

use anyhow::Result;
use clap::{Parser, command};
use commands::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Target project directory
    #[arg(short = 'D', long, default_value = ".")]
    directory: PathBuf,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    commands: Commands,
}

// List the names of your sub commands here.
register_commands! {
    Init
    Install
    Update
    Search
    Remove
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    logger::set_enable_verbose_logging(cli.verbose);

    cli.commands.run(&cli.directory)?;

    Ok(())
}
