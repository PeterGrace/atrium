// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `com.atproto.repo.deleteRecord` namespace.

/// Delete a record, or ensure it doesn't exist.
#[async_trait::async_trait]
pub trait DeleteRecord: crate::xrpc::XrpcClient {
    async fn delete_record(&self, input: Input) -> Result<(), Box<dyn std::error::Error>> {
        let _ = crate::xrpc::XrpcClient::send::<Error>(
            self,
            http::Method::POST,
            "com.atproto.repo.deleteRecord",
            None,
            Some(serde_json::to_vec(&input)?),
            Some(String::from("application/json")),
        )
        .await?;
        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    /// The NSID of the record collection.
    pub collection: String,
    /// The handle or DID of the repo.
    pub repo: String,
    /// The key of the record.
    pub rkey: String,
    /// Compare and swap with the previous commit by cid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_commit: Option<String>,
    /// Compare and swap with the previous record by cid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_record: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    InvalidSwap(Option<String>),
}
