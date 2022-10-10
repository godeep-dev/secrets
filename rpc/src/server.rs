//! RPC server

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::{Request, Response};

/// RPC service handler
#[async_trait]
pub trait Handler {
    /// Handles a service request
    async fn handle<R>(&self, receiver: R, request: R::Request) -> R::Response
    where
        R: Receiver;
}

/// RPC receiver
#[async_trait]
pub trait Receiver: Send + Sync {
    /// Receiver request
    type Request: Send;

    /// Receiver response
    type Response;

    /// Decodes a request
    async fn decode_request<E>(&self, req: Self::Request) -> Result<Request<Vec<u8>>, E>
    where
        E: From<String>;

    /// Decodes the request payload
    async fn decode_payload<T, E>(&self, data: &[u8]) -> Result<T, E>
    where
        T: DeserializeOwned,
        E: From<String>;

    /// Encodes a value
    async fn encode_ok<T>(&self, value: T) -> Self::Response
    where
        T: Serialize + Send;

    /// Encodes an error
    async fn encode_err<E>(&self, error: E) -> Self::Response
    where
        E: Serialize + Send;

    /// Encodes a service response
    async fn encode_response<T, E>(&self, res: Response<T, E>) -> Self::Response
    where
        T: Serialize + Send,
        E: Serialize + Send,
    {
        match res {
            Ok(ok) => self.encode_ok(ok).await,
            Err(err) => self.encode_err(err).await,
        }
    }
}

/// RPC server
#[derive(Debug, Clone)]
pub struct Server<H, R>
where
    H: Handler,
    R: Receiver,
{
    /// Server receiver
    pub(crate) receiver: R,
    /// Service handler
    pub(crate) handler: H,
}

impl<H, R> Server<H, R>
where
    H: Handler,
    R: Receiver,
{
    /// Instantiates a new [Server]
    pub fn new(receiver: R, handler: H) -> Self {
        Self { receiver, handler }
    }
}
