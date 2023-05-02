// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `com.atproto.sync.getCommitPath` namespace.

/// Gets the path of repo commits
#[async_trait::async_trait]
pub trait GetCommitPath: crate::xrpc::XrpcClient {
    async fn get_commit_path(&self, params: Parameters) -> Result<Output, Box<dyn std::error::Error>> {
        let body = crate::xrpc::XrpcClient::send::<Error>(
            self,
            http::Method::GET,
            "com.atproto.sync.getCommitPath",
            Some(serde_urlencoded::to_string(&params)?),
            None,
            None,
        )
        .await?;
        serde_json::from_slice(&body).map_err(|e| e.into())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    /// The DID of the repo.
    pub did: String,
    /// The earliest commit to start from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest: Option<String>,
    /// The most recent commit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub commits: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
}
