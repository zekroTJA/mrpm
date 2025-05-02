use super::Command;
use crate::manager::models::Project;
use crate::modrinth::models::{Loader, VersionType};
use crate::modrinth::{self};
use anyhow::Result;
use clap::Args;
use std::fmt;
use std::path::Path;
use yansi::Paint;

const LONG_ABOUT: &str = "\
Search for packages on Modrinth by the specified filters.

For more details, take a look at the Modrinth documentation:
https://docs.modrinth.com/api/operations/searchprojects/";

/// Search for packages
#[derive(Args)]
#[command(long_about = LONG_ABOUT)]
pub struct Versions {
    /// ID or slug of the project
    id_or_slug: String,

    /// Show a short summary of results
    #[arg(short, long)]
    short: bool,

    /// Used mod or plugin loader
    #[arg(short, long)]
    loader: Option<Loader>,

    /// Used Minecraft game version
    #[arg(short, long)]
    version: Option<Vec<loose_semver::Version>>,

    /// Show only featured packages
    #[arg(short, long)]
    featured: bool,

    /// Output result in JSON format
    #[arg(long)]
    json: bool,
}

impl Command for Versions {
    fn run(&self, target_dir: &Path) -> Result<()> {
        let project = Project::load(target_dir)?;

        let loader = self.loader.as_ref().or(project.as_ref().map(|p| &p.loader));

        let versions = self
            .version
            .clone()
            .or(project.as_ref().map(|p| p.game_version.as_list().to_vec()));

        let versions = modrinth::get_project_versions(
            &self.id_or_slug,
            loader.map(|l| [l]),
            versions,
            if self.featured { Some(true) } else { None },
        )?;

        if self.json {
            return serde_json::to_writer_pretty(std::io::stdout(), &versions)
                .map_err(|e| e.into());
        }

        for v in versions {
            if self.short {
                println!("{}", color_by_type(&v.version_number, &v.version_type));
            } else {
                println!(
                    "{}\n\
                    Version:     {}\n\
                    MC Versions: {}\n",
                    v.name.green(),
                    color_by_type(&v.version_number, &v.version_type),
                    if let Some(v) = v.game_versions {
                        v.iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    } else {
                        "no game versions specified".into()
                    }
                );
            }
        }

        Ok(())
    }
}

fn color_by_type<'a>(v: &'a impl fmt::Display, typ: &'a VersionType) -> impl fmt::Display + 'a {
    match typ {
        VersionType::Snapshot => v.blue(),
        VersionType::Alpha => v.red(),
        VersionType::Beta => v.yellow(),
        VersionType::Release => v.cyan(),
    }
}
