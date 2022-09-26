//! Error

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Service error code
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    /// Internal error
    Internal,
    /// Not implemented
    NotImplemented,
    /// Invalid parameter
    InvalidParam,
    /// Failed to send a request
    NoSend,
}

/// Service error
#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    /// Code
    pub code: ErrorCode,
    /// Message
    pub message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error {
    /// Instantiates a new [Error]
    pub fn new(code: ErrorCode, msg: impl AsRef<str>) -> Self {
        Self {
            code,
            message: msg.as_ref().to_string(),
        }
    }
}

macro_rules! impl_error_code_method {
    ($method: ident, $Code: ident) => {
        impl Error {
            /// Returns a new [Error]
            pub fn $method(msg: impl AsRef<str>) -> Self {
                Self {
                    code: ErrorCode::$Code,
                    message: msg.as_ref().to_string(),
                }
            }
        }
    };
}

impl_error_code_method!(internal, Internal);
impl_error_code_method!(not_implemented, NotImplemented);
impl_error_code_method!(invalid_body, InvalidParam);
impl_error_code_method!(no_send, NoSend);

/// Service result
pub type Result<T> = std::result::Result<T, Error>;
