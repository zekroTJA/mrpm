use super::Command;
use crate::manager::{Manager, models::InstallRef};
use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

/// Initialize a new project
#[derive(Args)]
#[command(alias = "i")]
pub struct Install {
    packages: Vec<InstallRef>,

    /// Target project directory
    #[arg(short = 'D', long, default_value = ".")]
    directory: PathBuf,
}

impl Command for Install {
    fn run(&self) -> Result<()> {
        let mut manager = Manager::new(&self.directory)?;

        let is_err = if self.packages.is_empty() {
            let p: Vec<InstallRef> = manager
                .project
                .dependencies
                .iter()
                .map(|kv| kv.into())
                .collect();
            manager.install_packages(p.iter(), false)
        } else {
            manager.install_packages(self.packages.iter(), false)
        };

        manager.store()?;

        if is_err {
            anyhow::bail!("Failed to install some packages.")
        }

        Ok(())
    }
}
