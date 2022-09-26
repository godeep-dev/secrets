//! HTTP connector

mod client;
mod server;

pub use client::*;
pub use server::*;

// --------------------------------------------------------
// HTTP SERVER & CLIENT
// --------------------------------------------------------

// /// Returns the response parts
// fn to_resp_parts<R>(code: StatusCode, res: Result<R>) -> (StatusCode, HeaderMap, R) {
//     let mut headers = HeaderMap::new();
//     headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());

//     match res {
//         Ok(ok) => {
//             //
//         }
//         Err(err) => {
//             let code = err;
//         }
//     }

//     let payload = HttpPayload {
//         data: Some(data),
//         error: None as Option<HttpPayloadError<()>>,
//     };
//     todo!()
//     // (code, headers, payload.encode())
// }

// /// Returns the ERROR response part
// fn err_resp_parts(error: Error) -> (StatusCode, HeaderMap, Vec<u8>) {
//     let mut headers = HeaderMap::new();
//     headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());

//     let payload = HttpPayload {
//         data: None as Option<()>,
//         error: Some(HttpPayloadError {
//             message: error.to_string(),
//             data: error,
//         }),
//     };
//     todo!()
//     // (code, headers, payload.encode())
// }
