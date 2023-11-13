// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.repo.createRecord` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    ///The NSID of the record collection.
    pub collection: String,
    ///The record to create.
    pub record: crate::records::Record,
    ///The handle or DID of the repo.
    pub repo: String,
    ///The key of the record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rkey: Option<String>,
    ///Compare and swap with the previous commit by cid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_commit: Option<String>,
    ///Validate the record?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub cid: String,
    pub uri: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    InvalidSwap(Option<String>),
}
