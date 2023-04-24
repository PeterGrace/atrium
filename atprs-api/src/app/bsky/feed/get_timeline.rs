// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `app.bsky.feed.getTimeline` namespace.

/// A view of the user's home timeline.
pub trait GetTimeline {
    fn get_timeline(&self, input: Parameters) -> Result<Output, Error>;
}

pub struct Parameters {
    pub algorithm: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Output {
    pub cursor: Option<String>,
    pub feed: Vec<crate::app::bsky::feed::defs::FeedViewPost>,
}

pub enum Error {
}
