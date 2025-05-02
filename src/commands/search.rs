use super::Command;
use crate::manager::models::Project;
use crate::modrinth::models::{Index, Loader};
use crate::modrinth::{self};
use anyhow::Result;
use clap::Args;
use std::num::NonZeroUsize;
use std::path::Path;
use yansi::Paint;

const LONG_ABOUT: &str = "\
Search for packages on Modrinth by the specified filters.

For more details, take a look at the Modrinth documentation:
https://docs.modrinth.com/api/operations/searchprojects/";

/// Search for packages
#[derive(Args)]
#[command(visible_aliases = ["s"], long_about = LONG_ABOUT)]
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
    version: Option<Vec<loose_semver::Version>>,

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

        let loader = self.loader.as_ref().or(project.as_ref().map(|p| &p.loader));

        let versions = self
            .version
            .clone()
            .or(project.as_ref().map(|p| p.game_version.as_list().to_vec()));

        let mut facets = vec![];
        if let Some(loader) = loader {
            facets.push(vec![format!("categories:{loader}")]);
        }
        if let Some(versions) = versions {
            facets.push(versions.iter().map(|v| format!("versions:{v}")).collect());
        }

        let results = modrinth::search_projects(
            &self.query,
            if facets.is_empty() { None } else { Some(&facets) },
            Some(&self.order),
            Some(self.offset),
            Some(self.limit.get()),
        )?;

        for r in &results.hits {
            if self.short {
                println!(
                    "{} {} {}",
                    r.title.green().bold(),
                    format_args!("by {}", r.author).dim(),
                    format_args!("{}{}{}", '['.dim(), r.project_id.cyan(), ']'.dim()),
                );
            } else {
                println!(
                    "{} {} {}\n\
                    {}\n\
                    Categories: {}\n",
                    r.title.green().bold(),
                    format_args!("by {}", r.author).dim(),
                    format_args!("{}{}{}", '['.dim(), r.project_id.cyan(), ']'.dim()),
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
