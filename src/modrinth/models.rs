use core::fmt;
use serde::{Deserialize, Serialize};

macro_rules! parse_enum {
    ($enum_name:ident, $( $field:ident => $name:expr),+) => {
        #[derive(Debug, Deserialize, Serialize, Clone)]
        #[serde(rename_all = "snake_case")]
        pub enum $enum_name {
            $(
                $field,
            )+
        }

        impl std::str::FromStr for $enum_name {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        $name => Ok(Self::$field),
                    )+
                    _ => Err(concat!(
                        "invalid value for ",
                        stringify!($enum_name),
                        "; valid values are: ",
                        $("\n  - ", $name,)+ ))
                }
            }
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$field => write!(f, $name),
                    )+
                }
            }
        }

        impl $enum_name {
            pub fn as_vec() -> Vec<$enum_name> {
                vec![$($enum_name::$field,)+]
            }
        }
    };
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Requirement {
    Required,
    Optional,
    Unsupported,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Approved,
    Archived,
    Unlisted,
    Private,
    Draft,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectType {
    Mod,
    Modpack,
    Resourcepack,
    Shader,
}

parse_enum! {
    Loader,

    Bukkit => "bukkit",
    Bungeecord => "bungeecord",
    Canvas => "canvas",
    Fabric => "fabric",
    Folia => "folia",
    Forge => "forge",
    Liteloader => "liteloader",
    Neoforge => "neoforge",
    Paper => "paper",
    Purpur => "purpur",
    Quilt => "quilt",
    Rift => "rift",
    Spigot => "spigot",
    Sponge => "sponge",
    Velocity => "velocity",
    Waterfall => "waterfall"
}

parse_enum! {
    Index,

    Relevance => "relevance",
    Downloads => "downloads",
    Follows => "follows",
    Newest => "newest",
    Updated => "updated"
}

#[derive(Deserialize, Debug)]
pub struct DontationUrl {
    pub id: String,
    pub platform: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct License {
    pub id: String,
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Project {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub client_side: Requirement,
    pub server_side: Requirement,
    pub body: String,
    pub project_type: ProjectType,
    pub downloads: usize,
    pub icon_url: Option<String>,
    pub color: Option<usize>,
    pub status: Status,
    pub license: Option<License>,
    pub requested_status: Option<Status>,
    pub additional_categories: Option<Vec<String>>,
    pub issues_url: Option<String>,
    pub source_url: Option<String>,
    pub wiki_url: Option<String>,
    pub discord_url: Option<String>,
    pub donation_urls: Option<Vec<DontationUrl>>,
    pub versions: Vec<String>,
    pub game_versions: Vec<String>,
    pub loaders: Vec<Loader>,
}

#[derive(Deserialize, Debug)]
pub struct SearchHit {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub client_side: Requirement,
    pub server_side: Requirement,
    pub project_type: ProjectType,
    pub downloads: usize,
    pub icon_url: Option<String>,
    pub color: Option<usize>,
    pub project_id: String,
    pub author: String,
    pub versions: Vec<String>,
    pub license: Option<String>,
    pub display_categories: Option<Vec<String>>,
    pub latest_version: String,
}

#[derive(Deserialize, Debug)]
pub struct SearchResults {
    pub hits: Vec<SearchHit>,
    pub offset: usize,
    pub limit: usize,
    pub total_hits: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyType {
    Required,
    Optional,
    Incompatible,
    Embedded,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VersionType {
    Release,
    Snapshot,
    Beta,
    Alpha,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VersionStatus {
    Listed,
    Archived,
    Draft,
    Unlisted,
    Scheduled,
    Unknown,
}

#[derive(Deserialize, Debug)]
pub struct Dependency {
    pub version_id: Option<String>,
    pub project_id: Option<String>,
    pub file_name: Option<String>,
    pub dependency_type: DependencyType,
}

#[derive(Deserialize, Debug)]
pub struct VersionFile {
    pub hashes: Hashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: usize,
    pub file_type: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Hashes {
    pub sha512: String,
    pub sha1: String,
}

#[derive(Deserialize, Debug)]
pub struct Version {
    pub name: String,
    pub version_number: String,
    pub changelog: Option<String>,
    pub dependencies: Option<Vec<Dependency>>,
    pub game_versions: Option<Vec<String>>,
    pub version_type: VersionType,
    pub loaders: Vec<Loader>,
    pub featured: bool,
    pub status: Option<VersionStatus>,
    pub id: String,
    pub project_id: String,
    pub author_id: String,
    pub downloads: usize,
    pub changelog_url: Option<String>,
    pub files: Vec<VersionFile>,
}

#[derive(Deserialize, Debug)]
pub struct GameVersion {
    pub version: String,
    pub version_type: VersionType,
    pub date: String,
    pub major: bool,
}

impl fmt::Display for GameVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.version)
    }
}
