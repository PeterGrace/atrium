// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `app.bsky.feed.like` namespace.

// app.bsky.feed.like
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub created_at: String,
    pub subject: crate::com::atproto::repo::strong_ref::Main,
}
