#![allow(dead_code)]

pub mod models;

use anyhow::Result;
use models::{GameVersion, Index, Loader, Project, SearchResults, Version};
use reqwest::Url;
use reqwest::blocking::get;
use serde::Serialize;

const BASE_URL: &str = "https://api.modrinth.com/v2";

/// # Documentation
/// See: https://docs.modrinth.com/api/operations/getproject/
pub fn get_project(slug_or_id: &str) -> Result<Project> {
    Ok(get(format!("{BASE_URL}/project/{slug_or_id}"))?
        .error_for_status()?
        .json()?)
}

/// # Documentation
/// See: https://docs.modrinth.com/api/operations/searchprojects/
pub fn search_projects<S, O>(
    query: &str,
    facets: Option<&[O]>,
    index: Option<&Index>,
    offset: Option<usize>,
    limit: Option<usize>,
) -> Result<SearchResults>
where
    S: AsRef<str> + Serialize,
    O: IntoIterator<Item = S> + Serialize,
{
    let mut url = Url::parse(&format!("{BASE_URL}/search"))?;

    url.query_pairs_mut().append_pair("query", query);

    if let Some(facets) = facets {
        url.query_pairs_mut()
            .append_pair("facets", &serde_json::to_string(&facets)?);
    }

    if let Some(index) = index {
        url.query_pairs_mut()
            .append_pair("index", &index.to_string());
    }

    if let Some(offset) = offset {
        url.query_pairs_mut()
            .append_pair("offset", &offset.to_string());
    }

    if let Some(limit) = limit {
        url.query_pairs_mut()
            .append_pair("limit", &limit.to_string());
    }

    Ok(get(url)?.error_for_status()?.json()?)
}

/// # Documentation
/// See: https://docs.modrinth.com/api/operations/getprojectversions/
pub fn get_project_versions(
    id_or_slug: &str,
    loaders: Option<&[&Loader]>,
    game_versions: Option<&[&str]>,
    featured: Option<bool>,
) -> Result<Vec<Version>> {
    let mut url = Url::parse(&format!("{BASE_URL}/project/{id_or_slug}/version"))?;

    if let Some(loaders) = loaders {
        url.query_pairs_mut()
            .append_pair("loaders", &serde_json::to_string(&loaders)?);
    }

    if let Some(game_versions) = game_versions {
        url.query_pairs_mut()
            .append_pair("game_versions", &serde_json::to_string(&game_versions)?);
    }

    if let Some(featured) = featured {
        url.query_pairs_mut()
            .append_pair("featured", &featured.to_string());
    }

    Ok(get(url)?.error_for_status()?.json()?)
}

pub fn get_game_versions() -> Result<Vec<GameVersion>> {
    Ok(get(format!("{BASE_URL}/tag/game_version"))?
        .error_for_status()?
        .json()?)
}
