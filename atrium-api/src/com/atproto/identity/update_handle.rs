// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `com.atproto.identity.updateHandle` namespace.

/// Updates the handle of the account
#[async_trait::async_trait]
pub trait UpdateHandle: crate::xrpc::XrpcClient {
    async fn update_handle(&self, input: Input) -> Result<(), Box<dyn std::error::Error>> {
        let _ = crate::xrpc::XrpcClient::send::<Error>(
            self,
            http::Method::POST,
            "com.atproto.identity.updateHandle",
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
    pub handle: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
}
