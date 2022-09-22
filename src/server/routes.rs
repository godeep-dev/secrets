//! Server API

use axum::{Extension, Json};
use reqwest::StatusCode;

use crate::shared::{ApiResponse, LoginData, ServerStatus, ServerStatusInfo, SignupPayload};

use super::{api, ServerInfo};

/// GET `/status`
///
/// Returns the server status
pub async fn get_server_status(
    Extension(info): Extension<ServerInfo>,
) -> ApiResponse<ServerStatus, ()> {
    ApiResponse::Ok(
        StatusCode::OK,
        ServerStatus {
            info: ServerStatusInfo { port: info.port },
        },
    )
}

// /// POST `/signup`
// ///
// /// Signups a new user
// pub async fn post_signup(
//     Extension(info): Extension<ServerInfo>,
//     Json(data): Json<SignupPayload>,
// ) -> ApiResponse<LoginData> {
//     let x = api::auth_signup(data);

//     // ApiResponse::Ok() {
//     //     data: ServerData { info },
//     //     error: None,
//     // }
//     // .into()
//     todo!();
// }
