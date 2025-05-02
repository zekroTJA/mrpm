pub mod models;

use crate::modrinth::{self};
use crate::{
    log_verbose, logger, print_install_error, print_install_new, print_install_skipped,
    print_install_updated, print_removed,
};
use anyhow::Result;
use core::fmt;
use models::{InstallRef, InstallState, Project};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
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
        log_verbose!("[manager] project dir = {dir:?}");

        let project = Project::load(&dir)?.ok_or_else(|| {
            anyhow::anyhow!("no project has been initialized in the project directory")
        })?;
        log_verbose!("[manager] project = {project:?}");

        let artifacts_dir = dir.join(&project.artifacts_dir);
        log_verbose!("[manager] artifacts dir = {artifacts_dir:?}");

        if !artifacts_dir.exists() {
            log_verbose!("[manager] creating artifacts dir");
            fs::create_dir_all(&artifacts_dir)?;
        }

        let state = InstallState::load(&artifacts_dir)?.unwrap_or_else(|| InstallState {
            installed_dependencies: HashMap::new(),
        });
        log_verbose!("[manager] state = {state:?}");

        Ok(Self {
            dir,
            project,
            state,
        })
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

    pub fn uninstall_removed_packages(&mut self) -> bool {
        log_verbose!("[manager] uninstalling removed packages");

        let mut is_err = false;

        let removed = self
            .state
            .installed_dependencies
            .iter()
            .filter(|d| !self.project.dependencies.contains_key(d.0));

        let mut deleted = vec![];

        for (key, dep) in removed {
            log_verbose!("[manager] uninstalling dep key={key}");
            match remove_file_oknotfound(self.artifacts_dir().join(&dep.file_name)) {
                Err(err) => {
                    print_install_error!(key, "remove failed: {err}");
                    is_err = true;
                }
                Ok(_) => {
                    print_removed!(key, dep.version_name);
                    deleted.push(key.clone());
                }
            }
        }

        for k in deleted {
            self.state.installed_dependencies.remove(&k);
        }

        is_err
    }

    pub fn uninstall_packages<S>(&mut self, packages: &[S]) -> bool
    where
        S: AsRef<str> + fmt::Display,
    {
        let mut is_err = false;

        for package in packages {
            match self.uninstall_package(package.as_ref()) {
                Ok(v) => print_removed!(package, v),
                Err(err) => {
                    print_install_error!(package, "remove failed: {err}");
                    is_err = true;
                }
            }
        }

        is_err
    }

    fn install_package(
        &mut self,
        slug_or_id: &str,
        version: Option<&str>,
        force: bool,
    ) -> Result<InstallResult> {
        log_verbose!(
            "[manager] installing package slug_or_id={slug_or_id} version={version:?} force={force}"
        );

        let installed_dep = self.state.installed_dependencies.get(slug_or_id);
        log_verbose!("[manager] installed_dep = {installed_dep:?}");

        if let Some(d) = installed_dep {
            if !force && (version.is_some_and(|v| v == d.version_name) || version.is_none()) {
                return Ok(InstallResult::Skipped(d.version_name.to_string()));
            }
        }

        let mut versions = modrinth::get_project_versions(
            slug_or_id,
            Some([&self.project.loader]),
            Some(self.project.game_version.as_list()),
            None,
        )?;

        versions.sort_by(|a, b| {
            b.game_versions
                .as_ref()
                .and_then(|v| v.first())
                .cmp(&a.game_versions.as_ref().and_then(|v| v.first()))
        });

        log_verbose!(
            "[manager] fetched project versions = {}",
            logger::DisplayList(&versions)
        );

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

        log_verbose!("[manager] target_version = {target_version}");

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

        log_verbose!("[manager] file = {file:?}");

        if let Some(installed_dep) = installed_dep {
            log_verbose!(
                "[manager] removing previously installed file name={}",
                &installed_dep.file_name
            );
            match remove_file_oknotfound(self.artifacts_dir().join(&installed_dep.file_name)) {
                Ok(_) => {}
                Err(err) => return Err(err.into()),
            }
        }

        log_verbose!("[manager] downloading new file url={}", &file.url);
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

        self.state
            .installed_dependencies
            .insert(slug_or_id.into(), models::InstalledDependency {
                version_name: target_version.version_number.clone(),
                version_id: target_version.id.clone(),
                file_name: file.filename.clone(),
            });

        self.project
            .dependencies
            .insert(slug_or_id.into(), target_version.version_number.clone());

        res
    }

    fn uninstall_package(&mut self, key: &str) -> Result<String> {
        log_verbose!("[manager] uninstalling package key={key}");

        let version = match self.state.installed_dependencies.get(key) {
            Some(dep) => {
                remove_file_oknotfound(self.artifacts_dir().join(&dep.file_name))?;
                Some(dep.version_name.clone())
            }
            None => self.project.dependencies.get(key).cloned(),
        }
        .ok_or_else(|| anyhow::anyhow!("package {key} is not installed"))?;

        log_verbose!("[manager] package removed version={version}");

        self.state.installed_dependencies.remove(key);
        self.project.dependencies.remove(key);

        Ok(version)
    }

    pub fn store(&self) -> Result<()> {
        log_verbose!("[manager] storing to project file");
        self.project.store(&self.dir)?;

        log_verbose!("[manager] storing to state file");
        self.state.store(self.artifacts_dir())?;

        Ok(())
    }

    fn artifacts_dir(&self) -> PathBuf {
        self.dir.join(&self.project.artifacts_dir)
    }
}

fn remove_file_oknotfound<P: AsRef<Path>>(path: P) -> io::Result<()> {
    match fs::remove_file(path) {
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(()),
        res => res,
    }
}
