//! Secrets client

#![deny(missing_docs)]

use service::*;

/// Marker struct for the client
pub struct Secrets;

/// Client
pub struct Client {
    /// RPC client
    rpc_client: rpc::Client<rpc::json::JsonTransport>,
    /// Token
    token: Option<String>,
}

impl Client {
    /// Instantiates a new [Client]
    pub const fn new() -> Self {
        let sender = rpc::json::JsonTransport::new();
        let rpc_client = rpc::Client::new(sender);
        Self {
            rpc_client,
            token: None,
        }
    }

    /// Authenticates the client
    pub fn authenticate(&mut self, token: impl AsRef<str>) {
        self.token = Some(token.as_ref().to_string());
    }

    /// Deuthenticates the client
    pub fn deauthenticate(&mut self) {
        self.token = None;
    }
}
