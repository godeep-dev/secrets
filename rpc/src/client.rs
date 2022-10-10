//! RPC client

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::{Request, Response};

/// RPC sender
#[async_trait]
pub trait Sender {
    /// Sender request
    type Request;

    /// Sender response
    type Response;

    /// Sends a request
    async fn send<E>(&self, req: Self::Request) -> Result<Self::Response, E>
    where
        E: From<String>;

    /// Encodes a RPC request
    async fn encode_request<T, E>(&self, req: Request<T>) -> Result<Self::Request, E>
    where
        T: Serialize + Send,
        E: From<String>;

    /// Decodes a received response
    async fn decode_response<T, E>(&self, res: Self::Response) -> Response<T, E>
    where
        T: DeserializeOwned,
        E: DeserializeOwned + From<String>;
}

/// RPC Client
#[derive(Debug)]
pub struct Client<S>
where
    S: Sender,
{
    /// Sender
    sender: S,
}

impl<S> Client<S>
where
    S: Sender,
{
    /// Instantiates a new service [Client]
    pub const fn new(sender: S) -> Self {
        Self { sender }
    }
}

impl<S> Client<S>
where
    S: Sender,
{
    /// Calls a RPC method
    pub async fn call<P, R, E>(&self, req: Request<P>) -> Response<R, E>
    where
        P: Serialize + Send,
        R: DeserializeOwned,
        E: DeserializeOwned + From<String>,
    {
        let sender = &self.sender;

        let req = sender.encode_request::<P, E>(req).await?;
        let res = sender.send::<E>(req).await?;
        sender.decode_response::<R, E>(res).await
    }
}
