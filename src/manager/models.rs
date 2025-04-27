use crate::modrinth::models::Loader;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{ErrorKind, Read, Write},
    path::{Path, PathBuf},
};

const PROJECT_FILENAME: &str = "mrpm.project.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub game_version: String,
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
