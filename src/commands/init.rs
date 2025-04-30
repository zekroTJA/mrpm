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
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
};
use yansi::Paint;

/// Initialize a new project
#[derive(Args)]
pub struct Init {
    /// Used mod or plugin loader
    #[arg(short, long)]
    loader: Option<Loader>,

    /// Used Minecraft game version
    #[arg(short, long)]
    version: Option<String>,

    /// Directory where artifacts will be stored
    /// relative to the project directory
    #[arg(short, long)]
    artifacts_dir: Option<PathBuf>,
}

impl Command for Init {
    fn run(&self, target_dir: &Path) -> Result<()> {
        if Project::load(target_dir)?.is_some() {
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

        let game_version = match &self.version {
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
                let p = inquire::Select::new("Game version", release_versions);
                p.prompt()?.version.clone()
            }
        };

        let artifacts_dir = match &self.artifacts_dir {
            Some(v) => v.clone(),
            None => {
                let p = inquire::Text::new("Artifacts directory")
                    .with_default(default_artifacts_dir_for_loader(&loader));
                PathBuf::from_str(&(p.prompt()?))?
            }
        };

        let project = Project {
            game_version,
            loader,
            artifacts_dir,
            dependencies: HashMap::new(),
        };

        project.store(target_dir)?;

        println!("{}", "Project initialized.".green());

        Ok(())
    }
}

fn default_artifacts_dir_for_loader(loader: &Loader) -> &'static str {
    match loader {
        Loader::Bukkit
        | Loader::Bungeecord
        | Loader::Canvas
        | Loader::Folia
        | Loader::Paper
        | Loader::Purpur
        | Loader::Spigot
        | Loader::Sponge
        | Loader::Velocity
        | Loader::Waterfall => "plugins",

        Loader::Fabric
        | Loader::Forge
        | Loader::Liteloader
        | Loader::Neoforge
        | Loader::Quilt
        | Loader::Rift => "mods",
    }
}
