use crate::modrinth::models::Loader;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs::{self, File};
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

const PROJECT_FILENAME: &str = "mrpm.project.toml";
const INSTALLSTATE_FILENAME: &str = ".mrpm.install-state.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum GameVersion {
    Single(String),
    Multiple(Vec<String>),
}

impl GameVersion {
    pub fn as_list(&self) -> &[String] {
        match self {
            GameVersion::Single(s) => std::slice::from_ref(s),
            GameVersion::Multiple(v) => v.as_slice(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub game_version: GameVersion,
    pub loader: Loader,
    pub artifacts_dir: PathBuf,
    pub dependencies: HashMap<String, String>, // name: version
}

impl Project {
    pub fn load<P: AsRef<Path>>(dir: P) -> Result<Option<Self>> {
        let mut f = match File::open(dir.as_ref().join(PROJECT_FILENAME)) {
            Ok(f) => f,
            Err(e) if e.kind() == ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(e.into()),
        };
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn store<P: AsRef<Path>>(&self, dir: P) -> Result<()> {
        let dir = dir.as_ref();
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }

        let mut f = File::create(dir.join(PROJECT_FILENAME))?;
        let content = toml::to_string_pretty(self)?;
        Ok(f.write_all(content.as_bytes())?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstalledDependency {
    pub version_name: String,
    pub version_id: String,
    pub file_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstallState {
    pub game_version: GameVersion,
    pub loader: Loader,
    pub installed_dependencies: HashMap<String, InstalledDependency>,
}

impl InstallState {
    pub fn load<P: AsRef<Path>>(dir: P) -> Result<Option<Self>> {
        let f = match File::open(dir.as_ref().join(INSTALLSTATE_FILENAME)) {
            Ok(f) => f,
            Err(e) if e.kind() == ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(e.into()),
        };
        Ok(serde_json::from_reader(f)?)
    }

    pub fn store<P: AsRef<Path>>(&self, dir: P) -> Result<()> {
        let dir = dir.as_ref();
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }

        let f = File::create(dir.join(INSTALLSTATE_FILENAME))?;
        Ok(serde_json::to_writer_pretty(f, &self)?)
    }
}

#[derive(Clone)]
pub struct InstallRef {
    pub id_or_slug: String,
    pub version: Option<String>,
}

impl FromStr for InstallRef {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some((id, version)) = s.split_once("@") {
            if id.is_empty() {
                return Err("package id or slug must be defined before @");
            }
            if version.is_empty() {
                return Err("package version must be defined after @");
            }
            return Ok(InstallRef {
                id_or_slug: id.to_lowercase(),
                version: Some(version.to_string()),
            });
        }
        Ok(InstallRef {
            id_or_slug: s.to_lowercase(),
            version: None,
        })
    }
}

impl fmt::Display for InstallRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id_or_slug)?;
        if let Some(ref v) = self.version {
            write!(f, "@{v}")?;
        }
        Ok(())
    }
}

impl<T> From<(T, T)> for InstallRef
where
    T: Into<String>,
{
    fn from((id, version): (T, T)) -> Self {
        Self {
            id_or_slug: id.into(),
            version: Some(version.into()),
        }
    }
}
