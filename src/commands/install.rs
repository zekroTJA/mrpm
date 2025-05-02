use super::Command;
use crate::manager::Manager;
use crate::manager::models::InstallRef;
use anyhow::Result;
use clap::Args;
use std::path::Path;

const LONG_ABOUT: &str = "\
Install new packages or install not installed packages defined in the project file.

You can specify a specific version to install as following:
$ mrpm install <project>@<version>";

/// Install packages
#[derive(Args)]
#[command(visible_aliases = ["i", "add", "a"], long_about = LONG_ABOUT)]
pub struct Install {
    // List of packages to install
    packages: Vec<InstallRef>,
}

impl Command for Install {
    fn run(&self, target_dir: &Path) -> Result<()> {
        let mut manager = Manager::new(target_dir)?;

        let is_err = if self.packages.is_empty() {
            let p: Vec<InstallRef> = manager
                .project
                .dependencies
                .iter()
                .map(|kv| kv.into())
                .collect();
            let ok_install = manager.install_packages(p.iter(), false);
            let ok_remove = manager.uninstall_removed_packages();
            ok_install && ok_remove
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
