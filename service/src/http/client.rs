//! HTTP client

use reqwest::Url;

use crate::error::{Error, Result};

/// Returns a service [HttpClient]
pub fn client(url: &str) -> Result<HttpClient> {
    HttpClient::new(url)
}

/// HTTP client
#[derive(Debug, Clone)]
pub struct HttpClient {
    /// Url
    url: Url,
}

impl HttpClient {
    /// Instantiates a new [HttpClient]
    pub fn new(url: &str) -> Result<Self> {
        Ok(Self {
            url: Url::parse(url).map_err(|err| Error::internal(err.to_string()))?,
        })
    }

    // /// Sends a request to the server
    // async fn request<R>(
    //     &self,
    //     method: Method,
    //     path: &str,
    //     query: Option<R::Query>,
    //     data: Option<R::ReqBody>,
    // ) -> Result<R>
    // where
    //     R: HttpRoute,
    // {
    //     // Prepare the request
    //     let url = self
    //         .url
    //         .join(path)
    //         .map_err(|err| Error::internal(err.to_string()))?;
    //     let mut req = reqwest::Client::new().request(method, url);

    //     // Pass the query string
    //     if let Some(q) = &query {
    //         req = req.query(q)
    //     }

    //     // Pass the body
    //     if let Some(data) = &data {
    //         req = req.json(data);
    //     }

    //     // Send the request
    //     let res = req
    //         .send()
    //         .await
    //         .map_err(|err| Error::no_send(err.to_string()))?;

    //     // Parse the payload
    //     let status = res.status();
    //     let payload = res
    //         .json::<HttpPayload<R, Error>>()
    //         .await
    //         .map_err(|err| Error::invalid_body(err.to_string()))?;

    //     if status.is_success() {
    //         let data = payload
    //             .data
    //             .ok_or_else(|| Error::invalid_body("Invalid body: mssing data"))?;
    //         Ok(data)
    //     } else if status.is_client_error() || status.is_server_error() {
    //         let error = payload
    //             .error
    //             .ok_or_else(|| Error::invalid_body("Invalid body: mssing error"))?
    //             .data;
    //         Err(error)
    //     } else {
    //         Err(Error::internal(format!(
    //             "Invalid API HTTP code: {}",
    //             status
    //         )))
    //     }
    // }
}

// impl HttpClient {
//     /// Get the service status
//     pub async fn status(&self) -> Result<ServiceStatus> {
//         self.request::<(), (), ServiceStatus>(Method::GET, "/status", None, None)
//             .await
//     }
// }
