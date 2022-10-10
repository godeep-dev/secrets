//! RPC service

use serde::{Deserialize, Serialize};

pub mod client;
pub mod server;
pub mod transports;

/// RPC request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request<T> {
    /// Method
    pub method: String,
    /// Authentication token
    pub token: Option<String>,
    /// Data
    pub data: T,
}

impl<T> Request<T> {
    /// Instantiates a new [Request]
    pub fn new(method: impl AsRef<str>, token: Option<String>, data: T) -> Self {
        Self {
            method: method.as_ref().to_string(),
            token,
            data,
        }
    }
}

/// RPC response
pub type Response<T, E> = Result<T, E>;
