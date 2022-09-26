//! HTTP handlers

use axum::{
    response::{IntoResponse, Response},
    Json,
};

use crate::ServiceStatus;

/// GET `/status`
pub async fn get_status() -> Response {
    Json(ServiceStatus {}).into_response()
}
