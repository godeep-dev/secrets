//! Shared resources

use std::fmt::Debug;

use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

// ---------------------------------------------------------------
// API response
// ---------------------------------------------------------------

/// API response
#[derive(Debug)]
pub enum ApiResponse<T, U> {
    /// Response data
    Ok(StatusCode, T),
    /// Response error
    Err(StatusCode, U),
}

/// API response payload
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiData<T, U> {
    /// Data payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// Error payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError<U>>,
}

/// API response payload (error)
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError<T> {
    /// Message
    pub message: String,
    /// Error data
    pub data: T,
}

impl<T, U> IntoResponse for ApiResponse<T, U>
where
    T: Debug + Serialize,
    U: Debug + Serialize,
{
    fn into_response(self) -> Response {
        let (status, payload) = match self {
            ApiResponse::Ok(status, data) => {
                // Check that the status is success
                if !status.is_success() {
                    panic!("Http code '{status}' is invalid for a success reponse");
                }

                (
                    status,
                    ApiData {
                        data: Some(data),
                        error: None,
                    },
                )
            }
            ApiResponse::Err(status, error) => (
                status,
                ApiData {
                    data: None,
                    error: Some(ApiError {
                        message: format!("{:?}", error),
                        data: error,
                    }),
                },
            ),
        };

        let body_str = json!(payload).to_string();
        let body: Body = body_str.into();
        (status, axum::body::boxed(body)).into_response()
    }
}

// ---------------------------------------------------------------
// SERVER MANAGEMENT
// ---------------------------------------------------------------

/// Server status data
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerStatus {
    pub info: ServerStatusInfo,
}

/// Server status info
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerStatusInfo {
    /// Server port
    pub port: u16,
}

// ---------------------------------------------------------------
// AUTH
// ---------------------------------------------------------------

/// Signup request payload
#[derive(Debug, Deserialize)]
pub struct SignupPayload {
    /// Email
    pub email: String,
    /// Name
    pub name: String,
    /// Password
    pub password: String,
}

/// Login response
#[derive(Debug, Serialize)]
pub struct LoginData {
    /// Token
    pub token: String,
    /// User
    pub user: User,
}

/// User
#[derive(Debug, Serialize)]
pub struct User {
    /// ID
    pub id: String,
    /// Name
    pub name: String,
    /// Email
    pub email: String,
    /// Password
    pub password: String,
}
