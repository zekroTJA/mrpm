pub mod models;

use crate::modrinth::{self};
use crate::{print_install_error, print_install_new, print_install_skipped, print_install_updated};
use anyhow::Result;
use models::{InstallRef, InstallState, Project};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::path::PathBuf;
use yansi::Paint;

pub enum InstallResult {
    Skipped(String),
    New(String),
    Updated(String, String),
}

pub struct Manager {
    pub project: Project,
    state: InstallState,
    dir: PathBuf,
}

impl Manager {
    pub fn new<P: Into<PathBuf>>(dir: P) -> Result<Self> {
        let dir = dir.into();

        let project = Project::load(&dir)?.ok_or_else(|| {
            anyhow::anyhow!("no project has been initialized in the project directory")
        })?;

        let artifacts_dir = dir.join(&project.artifacts_dir);

        if !artifacts_dir.exists() {
            fs::create_dir_all(&artifacts_dir)?;
        }

        let state = InstallState::load(&artifacts_dir)?.unwrap_or_else(|| InstallState {
            installed_dependencies: HashMap::new(),
        });

        Ok(Self {
            dir,
            project,
            state,
        })
    }

    pub fn install_package(
        &mut self,
        slug_or_id: &str,
        version: Option<&str>,
        force: bool,
    ) -> Result<InstallResult> {
        let installed_dep = self.state.installed_dependencies.get(slug_or_id);

        if let Some(d) = installed_dep {
            if !force && (version.is_some_and(|v| v == d.version_name) || version.is_none()) {
                return Ok(InstallResult::Skipped(d.version_name.to_string()));
            }
        }

        let mut versions = modrinth::get_project_versions(
            slug_or_id,
            Some(&[&self.project.loader]),
            Some(self.project.game_version.as_list()),
            None,
        )?;

        versions.sort_by(|a, b| {
            b.game_versions
                .as_ref()
                .and_then(|v| v.first())
                .cmp(&a.game_versions.as_ref().and_then(|v| v.first()))
        });

        let mimimum_version_type = self
            .project
            .minimum_version_type
            .clone()
            .unwrap_or_default();

        let target_version = match version {
            Some(version) => versions.iter().find(|v| v.version_number == version),
            None => versions
                .iter()
                .find(|v| v.version_type >= mimimum_version_type),
        }
        .ok_or_else(|| anyhow::anyhow!("no suitable version found"))?;

        if let Some(d) = installed_dep {
            if d.version_id == target_version.id {
                return Ok(InstallResult::Skipped(d.version_name.to_string()));
            }
        }

        let file = target_version
            .files
            .iter()
            .find(|f| f.primary)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "no file found for version {}",
                    target_version.version_number
                )
            })?;

        if let Some(installed_dep) = installed_dep {
            match fs::remove_file(self.artifacts_dir().join(&installed_dep.file_name)) {
                Ok(_) => {}
                Err(err) if err.kind() == ErrorKind::NotFound => {}
                Err(err) => return Err(err.into()),
            }
        }

        let mut f = File::create(self.artifacts_dir().join(&file.filename))?;
        reqwest::blocking::get(&file.url)?
            .error_for_status()?
            .copy_to(&mut f)?;

        let res = match installed_dep {
            Some(d) => Ok(InstallResult::Updated(
                d.version_name.to_string(),
                target_version.version_number.to_string(),
            )),
            None => Ok(InstallResult::New(target_version.version_number.clone())),
        };

        self.state.installed_dependencies.insert(
            slug_or_id.into(),
            models::InstalledDependency {
                version_name: target_version.version_number.clone(),
                version_id: target_version.id.clone(),
                file_name: file.filename.clone(),
            },
        );

        self.project
            .dependencies
            .insert(slug_or_id.into(), target_version.version_number.clone());

        res
    }

    pub fn install_packages<'a, I>(&mut self, packages: I, force: bool) -> bool
    where
        I: Iterator<Item = &'a InstallRef>,
    {
        let mut is_err = false;

        for package in packages {
            match self.install_package(&package.id_or_slug, package.version.as_deref(), force) {
                Ok(InstallResult::New(v)) => print_install_new!(package, v),
                Ok(InstallResult::Updated(p, v)) => print_install_updated!(package, p, v),
                Ok(InstallResult::Skipped(v)) => print_install_skipped!(package, v),
                Err(err) => {
                    print_install_error!(package, "{err}");
                    is_err = true;
                }
            }
        }

        is_err
    }

    pub fn store(&self) -> Result<()> {
        self.project.store(&self.dir)?;
        self.state.store(self.artifacts_dir())?;
        Ok(())
    }

    fn artifacts_dir(&self) -> PathBuf {
        self.dir.join(&self.project.artifacts_dir)
    }
}
