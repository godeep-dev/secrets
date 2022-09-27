//! HTTP encoding/decoding

use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    traits::ServiceMethod,
};

/// HTTP request body
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpReqBody<P> {
    /// Service method
    method: String,
    /// Method parameters
    params: P,
}

/// HTTP encoder/decoder
pub struct HttpCodec;

impl HttpCodec {
    /// Encodes a HTTP request
    pub fn encode_request<T>(params: T::Params) -> Result<Vec<u8>>
    where
        T: ServiceMethod,
    {
        let payload = HttpReqBody {
            method: T::ID.to_string(),
            params,
        };
        serde_json::to_vec(&payload)
            .map_err(|err| Error::internal(format!("Cannot serialize request body: {err}")))
    }

    /// Decodes a http request
    pub fn decode_request(body: &[u8]) -> Result<(String, Vec<u8>)> {
        let http_body = serde_json::from_slice::<HttpReqBody<Vec<u8>>>(body)
            .map_err(|err| Error::invalid_body(err.to_string()))?;

        Ok((http_body.method, http_body.params))
    }

    /// Decodes a http request parameters
    pub fn decode_request_params<T>(params: &[u8]) -> Result<T::Params>
    where
        T: ServiceMethod,
    {
        serde_json::from_slice::<T::Params>(params)
            .map_err(|err| Error::invalid_body(err.to_string()))
    }

    /// Encodes a HTTP response
    pub fn encode_response<T>(value: T) -> Result<Vec<u8>>
    where
        T: Serialize,
    {
        serde_json::to_vec(&value).map_err(|err| Error::internal(err.to_string()))
    }

    /// Decodes a HTTP response
    pub fn decode_response<T>(status: StatusCode, body: &[u8]) -> Result<T::RetValue>
    where
        T: ServiceMethod,
    {
        if !status.is_success() {
            let error = serde_json::from_slice::<Error>(&body)
                .map_err(|err| Error::invalid_body(err.to_string()))?;
            return Err(error);
        }

        let ret_value = serde_json::from_slice::<T::RetValue>(&body)
            .map_err(|err| Error::invalid_body(err.to_string()))?;
        Ok(ret_value)
    }
}
