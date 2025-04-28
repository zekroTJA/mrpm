mod commands;
mod manager;
mod modrinth;
mod output;

use anyhow::Result;
use clap::{Parser, command};
use commands::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
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

    cli.commands.run()?;

    Ok(())
}
