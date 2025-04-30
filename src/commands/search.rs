use super::Command;
use crate::{
    manager::models::Project,
    modrinth::{
        self,
        models::{Index, Loader},
    },
};
use anyhow::Result;
use clap::Args;
use std::{num::NonZeroUsize, path::Path};
use yansi::Paint;

const LONG_ABOUT: &str = "\
Search for packages on Modrinth by the specified filters.

For more details, take a look at the Modrinth documentation:
https://docs.modrinth.com/api/operations/searchprojects/";

/// Search for packages
#[derive(Args)]
#[command(alias = "s", long_about = LONG_ABOUT)]
pub struct Search {
    /// Search query
    query: String,

    /// Show a short summary of results
    #[arg(short, long)]
    short: bool,

    /// Used mod or plugin loader
    #[arg(short, long)]
    loader: Option<Loader>,

    /// Used Minecraft game version
    #[arg(short, long)]
    version: Option<String>,

    /// Search result limit
    #[arg(long, default_value_t = NonZeroUsize::new(10).unwrap())]
    limit: NonZeroUsize,

    /// Search result offset
    #[arg(long, default_value_t = 0)]
    offset: usize,

    /// Search order / index
    #[arg(long, alias = "index", default_value_t = Index::Relevance)]
    order: Index,
}

impl Command for Search {
    fn run(&self, target_dir: &Path) -> Result<()> {
        let project = Project::load(target_dir)?;

        let loader = self
            .loader
            .as_ref()
            .or(project.as_ref().and_then(|p| Some(&p.loader)));

        let version = self
            .version
            .as_ref()
            .or(project.as_ref().and_then(|p| Some(&p.game_version)));

        let mut facets = vec![];
        if let Some(loader) = loader {
            facets.push([format!("categories:{loader}")]);
        }
        if let Some(version) = version {
            facets.push([format!("versions:{version}")]);
        }

        let results = modrinth::search_projects(
            &self.query,
            Some(&facets),
            Some(&self.order),
            Some(self.offset),
            Some(self.limit.get()),
        )?;

        for r in &results.hits {
            if self.short {
                println!(
                    "{} {} {}",
                    r.title.green().bold(),
                    format!("by {}", r.author).dim(),
                    format!("{}{}{}", '['.dim(), r.project_id.cyan(), ']'.dim()),
                );
            } else {
                println!(
                    "{} {} {}\n\
                    {}\n\
                    Categories: {}\n",
                    r.title.green().bold(),
                    format!("by {}", r.author).dim(),
                    format!("{}{}{}", '['.dim(), r.project_id.cyan(), ']'.dim()),
                    r.description,
                    r.categories.join(", ")
                );
            }
        }

        println!(
            "{}",
            format!(
                "{} total results; {} shown",
                results.total_hits,
                results.hits.len()
            )
            .italic()
            .dim()
        );

        Ok(())
    }
}
