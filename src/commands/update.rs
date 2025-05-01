use super::Command;
use crate::manager::Manager;
use crate::manager::models::InstallRef;
use anyhow::Result;
use clap::Args;
use std::path::Path;

const LONG_ABOUT: &str = "\
Update installed packages or update packages defined in the project file.

You can specify a specific version to update as following:
$ mrpm update <project>@<version>";

/// Update packages
#[derive(Args)]
#[command(alias = "u", long_about = LONG_ABOUT)]
pub struct Update {
    // List of packages to update
    packages: Vec<InstallRef>,
}

impl Command for Update {
    fn run(&self, target_dir: &Path) -> Result<()> {
        let mut manager = Manager::new(target_dir)?;

        let is_err = if self.packages.is_empty() {
            let p: Vec<InstallRef> = manager
                .project
                .dependencies
                .keys()
                .map(|k| InstallRef {
                    id_or_slug: k.into(),
                    version: None,
                })
                .collect();
            let ok_install = manager.install_packages(p.iter(), true);
            let ok_remove = manager.uninstall_removed_packages();
            ok_install && ok_remove
        } else {
            manager.install_packages(self.packages.iter(), true)
        };

        manager.store()?;

        if is_err {
            anyhow::bail!("Failed to update some packages.")
        }

        Ok(())
    }
}
