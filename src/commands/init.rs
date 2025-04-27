use super::Command;
use crate::{
    manager::models::Project,
    modrinth::{
        self,
        models::{Loader, VersionType},
    },
};
use anyhow::Result;
use clap::Args;
use std::{collections::HashMap, path::PathBuf};

/// Initialize a ner project
#[derive(Args)]
pub struct Init {
    #[arg(default_value = ".")]
    directory: PathBuf,

    #[arg(short, long)]
    loader: Option<Loader>,

    #[arg(short, long)]
    game_version: Option<String>,
}

impl Command for Init {
    fn run(&self) -> Result<()> {
        if Project::load(&self.directory)?.is_some() {
            let ok = inquire::Confirm::new(
                "A project file already exists in this location. Do you want to re-initialize the project?",
            ).with_default(false).prompt()?;
            if !ok {
                println!("Canceled.");
                return Ok(());
            }
        }

        let game_versions = modrinth::get_game_versions()?;

        let loader = match &self.loader {
            Some(v) => v.clone(),
            None => {
                let p = inquire::Select::new("Loader", Loader::as_vec());
                p.prompt()?
            }
        };

        let game_version = match &self.game_version {
            Some(v) => {
                if !game_versions.iter().any(|c| &c.version == v) {
                    anyhow::bail!("the specified game version does not exist");
                }
                v.clone()
            }
            None => {
                let release_versions = game_versions
                    .iter()
                    .filter(|v| v.version_type == VersionType::Release)
                    .collect();
                let p = inquire::Select::new("Game Version", release_versions);
                p.prompt()?.version.clone()
            }
        };

        let project = Project {
            game_version,
            loader,
            dependencies: HashMap::new(),
        };

        project.store(&self.directory)?;

        println!("Project initialized.");

        Ok(())
    }
}
