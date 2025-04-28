mod commands;
mod config;
mod manager;
mod modrinth;
mod output;

use anyhow::Result;
use clap::{Parser, command};
use commands::*;
use config::Config;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to a config file
    #[arg(short, long)]
    config: Option<String>,

    #[command(subcommand)]
    commands: Commands,
}

// List the names of your sub commands here.
register_commands! {
    Init
    Install
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let cfg = Config::parse(cli.config)?;

    cli.commands.run()?;

    Ok(())
}
