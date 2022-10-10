//! JSON transport

use std::{convert::Infallible, net::SocketAddr};

use async_trait::async_trait;
use hyper::{
    header,
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    client::Sender,
    server::{Handler, Receiver, Server},
    Request, Response,
};

/// JSON transport
#[derive(Debug, Clone)]
pub struct JsonTransport {}

impl JsonTransport {
    /// RPC method
    const HEADER_METHOD: &str = "X-RPC-METHOD";

    /// Instantiates a new [HyperTransport]
    pub const fn new() -> Self {
        Self {}
    }
}

impl<H> Server<H, JsonTransport>
where
    H: Handler + Clone + Send + Sync + 'static,
{
    /// Starts the server
    pub async fn start(self, addr: &SocketAddr) -> Result<(), hyper::Error> {
        let handler = self.handler;
        let receiver = self.receiver;

        let make_service = make_service_fn(move |_conn: &AddrStream| {
            let handler = handler.clone();
            let receiver = receiver.clone();

            let service = service_fn(move |req: hyper::Request<hyper::Body>| {
                let handler = handler.clone();
                let receiver = receiver.clone();

                async move {
                    let res = handler.handle(receiver, req).await;
                    Ok(res) as Result<hyper::Response<hyper::Body>, hyper::Error>
                }
            });

            async move { Ok::<_, Infallible>(service) }
        });

        // Listen to the server
        hyper::Server::bind(addr).serve(make_service).await?;
        Ok(())
    }
}

#[async_trait]
impl Receiver for JsonTransport {
    type Request = hyper::Request<hyper::Body>;
    type Response = hyper::Response<hyper::Body>;

    async fn decode_request<E>(&self, req: Self::Request) -> Result<Request<Vec<u8>>, E>
    where
        E: From<String>,
    {
        let method = match req.headers().get(Self::HEADER_METHOD) {
            Some(m) => match m.to_str() {
                Ok(ok) => ok.to_owned(),
                Err(err) => {
                    return Err(E::from(format!("Invalid method header: {err}")));
                }
            },
            None => {
                return Err(E::from("Missing method header".to_string()));
            }
        };

        // Extract bearer token from request
        let token = match req.headers().get("Authorization") {
            Some(m) => match m.to_str() {
                Ok(ok) => match ok.strip_prefix("Bearer ") {
                    Some(s) => Some(s.to_owned()),
                    None => {
                        return Err(E::from(format!("Invalid auth header: {ok}")));
                    }
                },
                Err(err) => {
                    return Err(E::from(format!("Invalid auth header: {err}")));
                }
            },
            None => None,
        };

        // Extract body as bytes
        let data = match hyper::body::to_bytes(req.into_body()).await {
            Ok(ok) => ok.to_vec(),
            Err(err) => {
                return Err(E::from(format!("Invalid body: {}", err)));
            }
        };

        Ok(Request {
            method,
            token,
            data,
        })
    }

    async fn decode_payload<T, E>(&self, data: &[u8]) -> Result<T, E>
    where
        T: DeserializeOwned,
        E: From<String>,
    {
        let value = match serde_json::from_slice::<T>(data) {
            Ok(ok) => ok,
            Err(err) => {
                return Err(E::from(format!("Invalid body: {}", err)));
            }
        };
        Ok(value)
    }

    async fn encode_ok<T>(&self, value: T) -> Self::Response
    where
        T: Serialize + Send,
    {
        let data = serde_json::to_vec(&value).unwrap();
        let len = data.len();
        let body = hyper::Body::from(data);
        hyper::Response::builder()
            .status(hyper::StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::CONTENT_LENGTH, len)
            .body(body)
            .unwrap()
    }

    async fn encode_err<E>(&self, error: E) -> Self::Response
    where
        E: Serialize + Send,
    {
        let data = serde_json::to_vec(&error).unwrap();
        let len = data.len();
        let body = hyper::Body::from(data);
        hyper::Response::builder()
            .status(hyper::StatusCode::BAD_REQUEST)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::CONTENT_LENGTH, len)
            .body(body)
            .unwrap()
    }
}

#[async_trait]
impl Sender for JsonTransport {
    type Request = hyper::Request<hyper::Body>;
    type Response = hyper::Response<hyper::Body>;

    async fn send<E>(&self, req: Self::Request) -> Result<Self::Response, E>
    where
        E: From<String>,
    {
        let hyper_client = hyper::Client::new();
        hyper_client
            .request(req)
            .await
            .map_err(|err| E::from(err.to_string()))
    }

    async fn encode_request<T, E>(&self, req: Request<T>) -> Result<Self::Request, E>
    where
        T: Serialize + Send,
        E: From<String>,
    {
        let bytes = match serde_json::to_vec(&req.data) {
            Ok(ok) => ok,
            Err(err) => {
                return Err(E::from(format!("Cannot encode value: {}", err)));
            }
        };

        let req = hyper::Request::builder()
            .uri(uri)
            .header(Self::HEADER_METHOD, req.method)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::CONTENT_LENGTH, bytes.len())
            .body(bytes.into())
            .unwrap();

        Ok(req)
    }

    async fn decode_response<T, E>(&self, res: Self::Response) -> Response<T, E>
    where
        T: DeserializeOwned,
        E: DeserializeOwned + From<String>,
    {
        let status = res.status();

        let bytes = match hyper::body::to_bytes(res.into_body()).await {
            Ok(ok) => ok,
            Err(err) => {
                return Err(E::from(format!("Cannot read response body: {}", err)));
            }
        };

        if status.is_success() {
            let value = match serde_json::from_slice::<T>(&bytes) {
                Ok(ok) => ok,
                Err(err) => {
                    return Err(E::from(format!("Cannot deserialize value: {}", err)));
                }
            };
            Ok(value)
        } else {
            let error = match serde_json::from_slice::<E>(&bytes) {
                Ok(ok) => ok,
                Err(err) => {
                    return Err(E::from(format!("Cannot deserialize value: {}", err)));
                }
            };
            Err(error)
        }
    }
}
