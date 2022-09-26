//! HTTP routes

use std::net::SocketAddr;

use axum::{
    http::{Method, StatusCode},
    routing::get,
    Extension,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{error::Error, *};

use super::HttpClient;

mod handlers;

/// Trait to get the service HTTP server
pub trait IntoHttpServer: Service + Sized {
    /// Returns a [HttpServer]
    fn server(self) -> HttpServer<Self> {
        HttpServer { service: self }
    }
}

impl<T> IntoHttpServer for T where T: Service {}

/// HTTP server
pub struct HttpServer<S>
where
    S: Service,
{
    service: S,
}

impl<S> HttpServer<S>
where
    S: Service + Clone + Send + Sync + 'static,
{
    /// Starts the server
    pub async fn start(self, addr: &SocketAddr) {
        let service = self.service;

        let app = axum::Router::new()
            .route("/status", get(handlers::get_status))
            .layer(Extension(service));

        axum::Server::bind(addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

/// HTTP payload
#[derive(Debug, Serialize, Deserialize)]
struct HttpPayload<T, E> {
    /// Response data
    data: Option<T>,
    /// Response error
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<HttpPayloadError<E>>,
}

/// HTTP payload error
#[derive(Debug, Serialize, Deserialize)]
struct HttpPayloadError<E> {
    /// Error message
    message: String,
    /// Error custom data
    data: E,
}

impl Error {
    /// Returns the HTTP status code
    pub fn http_code(&self) -> StatusCode {
        match self.code {
            crate::error::ErrorCode::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            crate::error::ErrorCode::NotImplemented => StatusCode::BAD_REQUEST,
            crate::error::ErrorCode::InvalidParam => StatusCode::BAD_REQUEST,
            crate::error::ErrorCode::NoSend => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// A single http route
pub trait HttpRoute {
    /// Route path
    ///
    /// This path will be processed by the crate `tinytemplate`
    const PATH: &'static str;

    /// Method
    const METHOD: Method;

    /// Path parameters
    type Params: DeserializeOwned;

    /// Query string parameters shape
    type Query: DeserializeOwned;

    /// Request body shape
    type ReqBody: DeserializeOwned;

    /// Response body
    type RespBody: Serialize;
}

/// Route GET `/status`
pub struct GetStatus(());

impl HttpRoute for GetStatus {
    const PATH: &'static str = "/status";
    const METHOD: Method = Method::GET;
    type Params = ();
    type Query = ();
    type ReqBody = ();
    type RespBody = ServiceStatus;
}

// impl<S> HttpServer<S>
// where
//     S: Service,
// {
//     /// Handles a request
//     pub async fn handle(&self, request: Request<impl Body>) -> (StatusCode, HeaderMap, Vec<u8>) {
//         // Extract request parameters
//         let method = request.method();
//         let uri = request.uri();
//         let headers = request.headers();
//         let body = request.into_body().data();

//         (StatusCode::INTERNAL_SERVER_ERROR, HeaderMap::new(), vec![])
//     }
// }
