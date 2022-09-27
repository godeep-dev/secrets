//! HTTP Server

use std::{convert::Infallible, net::SocketAddr};

use hyper::{
    body, header,
    server::conn::{AddrStream, Http},
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use serde::Serialize;

use crate::{
    error::Error,
    traits::{ServiceMethod, StatusSrvMethod},
    Service,
};

use super::codec::HttpCodec;

/// Http server
pub struct HttpServer<S>
where
    S: Service,
{
    /// Service
    service: S,
    /// Address
    address: SocketAddr,
}

impl<S> HttpServer<S>
where
    S: Service,
{
    /// Instantiates a new [HttpServer]
    pub fn new(service: S, address: SocketAddr) -> Self {
        Self { service, address }
    }

    /// Starts the server
    pub async fn start(self) {
        let service_handler = HttpHandler::new(self.service);

        let make_service = make_service_fn(move |_conn: &AddrStream| {
            let service_handler = service_handler.clone();
            let service = service_fn(move |req| {
                let service_handler = service_handler.clone();
                service_handler.handle(req)
            });
            async move { Ok::<_, Infallible>(service) }
        });

        // Run this server for... forever!
        let server = Server::bind(&self.address).serve(make_service);
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    }
}

/// HTTP handler
#[derive(Debug, Clone)]
struct HttpHandler<S>
where
    S: Service,
{
    service: S,
}

impl<S> HttpHandler<S>
where
    S: Service,
{
    /// Instantiates a [ServiceHandler]
    fn new(service: S) -> Self {
        Self { service }
    }

    /// Handles a request
    async fn handle(self, req: Request<Body>) -> std::result::Result<Response<Body>, Infallible> {
        // Check the method
        let method = req.method();
        if method != Method::POST {
            let err = Error::not_implemented("Invalid method");
            return self.process_response(err, StatusCode::METHOD_NOT_ALLOWED);
        }

        // Read the request body
        let body_u8 = match body::to_bytes(req.into_body()).await {
            Ok(ok) => ok,
            Err(err) => {
                let err = Error::invalid_body(err.to_string());
                return self.process_response(err, StatusCode::BAD_REQUEST);
            }
        };

        // Parse the request body
        let (method, params) = match HttpCodec::decode_request(&body_u8) {
            Ok(ok) => ok,
            Err(err) => {
                return self.process_response(err, StatusCode::BAD_REQUEST);
            }
        };

        // Call the service
        self.handle_srv_method(&method, &params).await
    }

    /// Returns an HTTP error response
    fn process_response<T>(
        &self,
        payload: T,
        code: StatusCode,
    ) -> Result<Response<Body>, Infallible>
    where
        T: Serialize,
    {
        let json = HttpCodec::encode_response(payload).unwrap();
        let content_len = json.len();
        let body = Body::from(json);

        let res = Response::builder()
            .status(code)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::CONTENT_LENGTH, content_len)
            .body(body)
            .unwrap();

        Ok(res)
    }
}

// ---------------------------------------------------------------
// Service implementation
//
// NB: This could be derived by a macro on the trait
// ---------------------------------------------------------------

impl<S> HttpHandler<S>
where
    S: Service,
{
    /// Handles a single method
    async fn handle_srv_method(
        &self,
        method: &str,
        params: &[u8],
    ) -> Result<Response<Body>, Infallible> {
        match method {
            StatusSrvMethod::ID => {
                match HttpCodec::decode_request_params::<StatusSrvMethod>(params) {
                    Ok(p) => p,
                    Err(err) => {
                        return self.process_response(err, StatusCode::BAD_REQUEST);
                    }
                };
                match self.service.status().await {
                    Ok(ok) => {
                        let body = match HttpCodec::encode_response(ok) {
                            Ok(ok) => ok,
                            Err(err) => {
                                return self
                                    .process_response(err, StatusCode::INTERNAL_SERVER_ERROR);
                            }
                        };
                        self.process_response(&body, StatusCode::OK)
                    }
                    Err(err) => self.process_response(err, StatusCode::INTERNAL_SERVER_ERROR),
                }
            }
            method => {
                let error = Error::not_implemented(format!("Invalid method: {method}"));
                self.process_response(error, StatusCode::BAD_REQUEST)
            }
        }
    }
}
