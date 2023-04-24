// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `com.atproto.admin.searchRepos` namespace.

/// Find repositories based on a search term.
pub trait SearchRepos {
    fn search_repos(&self, input: Parameters) -> Result<Output, Error>;
}

pub struct Parameters {
    pub cursor: Option<String>,
    pub invited_by: Option<String>,
    pub limit: Option<i32>,
    pub term: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Output {
    pub cursor: Option<String>,
    pub repos: Vec<crate::com::atproto::admin::defs::RepoView>,
}

pub enum Error {
}
