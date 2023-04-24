// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `app.bsky.actor.getSuggestions` namespace.

/// Get a list of actors suggested for following. Used in discovery UIs.
pub trait GetSuggestions {
    fn get_suggestions(&self, input: Parameters) -> Result<Output, Error>;
}

pub struct Parameters {
    pub cursor: Option<String>,
    pub limit: Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Output {
    pub actors: Vec<crate::app::bsky::actor::defs::ProfileView>,
    pub cursor: Option<String>,
}

pub enum Error {
}
