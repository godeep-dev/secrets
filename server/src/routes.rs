//! Routes

// use axum::{
//     body::Body,
//     http::Request,
//     response::{IntoResponse, Response},
//     Extension,
// };

// use crate::service::ServiceImpl;
// use service::http::{HttpRoute, HttpServer};

// /// Extension trait to convert a [HttpRoute] to an axum service
// pub trait HttpRouteExt: HttpRoute {}

// /// ANY `/*key`
// pub async fn service_handler(
//     Extension(service): Extension<ServiceImpl>,
//     request: Request<Body>,
// ) -> Response {
//     let server = HttpServer::new(service);

//     eprintln!("REQUEST={:?}", request);
//     let (code, headers, body) = server.handle(request).await;
//     (code, headers, body).into_response()
// }
