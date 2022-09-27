//! HTTP client

use hyper::{body::HttpBody, header, Body, Method, Request};

use crate::{
    error::Error,
    traits::{ServiceMethod, StatusSrvMethod},
    *,
};

use super::codec::HttpCodec;

/// HTTP client
#[derive(Debug)]
pub struct HttpClient {
    /// API endpoint
    uri: String,
}

impl HttpClient {
    /// Instantiates a new [HttpClient]
    pub fn new(uri: &str) -> Self {
        Self {
            uri: uri.to_string(),
        }
    }

    /// Sends a request
    async fn send_request<T>(&self, params: T::Params) -> Result<T::RetValue>
    where
        T: ServiceMethod,
    {
        let client = hyper::Client::new();

        // Prepare the request
        let req_body = HttpCodec::encode_request::<T>(params)?;
        let content_length = req_body.len();

        // Send the request
        let req = Request::builder()
            .method(Method::POST)
            .uri(&self.uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::CONTENT_LENGTH, content_length)
            .body(Body::from(req_body))
            .map_err(|err| Error::invalid_body(err.to_string()))?;
        let mut res = client
            .request(req)
            .await
            .map_err(|err| Error::no_send(err.to_string()))?;

        // Read the body
        let mut res_body: Vec<u8> = vec![];
        while let Some(chunk) = res.body_mut().data().await {
            let bytes = chunk.unwrap();
            res_body.extend_from_slice(&bytes);
        }

        // Process the response
        let status = res.status();
        HttpCodec::decode_response::<T>(status, &res_body)
    }
}

// ---------------------------------------------------------------
// Service implementation
//
// NB: This could be derived by a macro on the trait
// ---------------------------------------------------------------

// NB: Client methods could be implemented via a macro
impl HttpClient {
    /// Returns the service status
    pub async fn status(&self) -> Result<ServiceStatus> {
        let params = ();
        self.send_request::<StatusSrvMethod>(params).await
    }
}
