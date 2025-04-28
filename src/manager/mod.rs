pub mod models;

use crate::{
    modrinth::{self, models::VersionType},
    print_install_error, print_install_skipped, print_install_success,
};
use anyhow::Result;
use models::{InstallRef, InstallState, Project};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::ErrorKind,
    path::PathBuf,
};
use yansi::Paint;

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
            game_version: project.game_version.clone(),
            loader: project.loader.clone(),
            installed_dependencies: HashMap::new(),
        });

        Ok(Self {
            dir,
            project,
            state,
        })
    }

    pub fn install(
        &mut self,
        slug_or_id: &str,
        version: Option<&str>,
        force: bool,
    ) -> Result<bool> {
        let installed_dep = self.state.installed_dependencies.get(slug_or_id);

        if !force
            && installed_dep
                .is_some_and(|d| version.is_some_and(|v| v == d.version) || version.is_none())
        {
            return Ok(false);
        }

        let versions = modrinth::get_project_versions(
            slug_or_id,
            Some(&[&self.project.loader]),
            Some(&[&self.project.game_version]),
            None,
        )?;

        let target_version = match version {
            Some(version) => versions.iter().find(|v| v.version_number == version),
            None => versions
                .iter()
                .find(|v| v.version_type == VersionType::Release)
                .or(versions
                    .iter()
                    .find(|v| v.version_type == VersionType::Beta))
                .or(versions.first()),
        }
        .ok_or_else(|| anyhow::anyhow!("no suitable version found"))?;

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

        self.state.installed_dependencies.insert(
            slug_or_id.into(),
            models::InstalledDependency {
                version: target_version.version_number.clone(),
                file_name: file.filename.clone(),
            },
        );

        self.project
            .dependencies
            .insert(slug_or_id.into(), target_version.version_number.clone());

        Ok(true)
    }

    pub fn install_packages<'a, I>(&mut self, packages: I, force: bool) -> bool
    where
        I: Iterator<Item = &'a InstallRef>,
    {
        let mut is_err = false;

        for package in packages {
            match self.install(&package.id_or_slug, package.version.as_deref(), force) {
                Ok(true) => print_install_success!(package),
                Ok(false) => print_install_skipped!(package),
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
