// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.feed.repost` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub created_at: String,
    pub subject: crate::com::atproto::repo::strong_ref::Main,
}
