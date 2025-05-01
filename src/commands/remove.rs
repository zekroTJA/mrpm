use super::Command;
use crate::manager::Manager;
use anyhow::Result;
use clap::Args;
use std::path::Path;

const LONG_ABOUT: &str = "";

/// Remove packages
#[derive(Args)]
#[command(alias = "r", long_about = LONG_ABOUT)]
pub struct Remove {
    // List of packages to remove
    #[arg(required = true)]
    packages: Vec<String>,
}

impl Command for Remove {
    fn run(&self, target_dir: &Path) -> Result<()> {
        let mut manager = Manager::new(target_dir)?;

        let is_err = manager.uninstall_packages(&self.packages);

        manager.store()?;

        if is_err {
            anyhow::bail!("Failed to remove some packages.")
        }

        Ok(())
    }
}
