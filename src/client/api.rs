//! Client API

use anyhow::bail;

use crate::shared::{ApiData, ServerStatus};

use super::Client;

impl Client {
    /// Queries the server status
    pub async fn server_status(&self) -> anyhow::Result<ServerStatus> {
        let client = reqwest::Client::new();
        let url = self.url.join("./status")?;
        let res = client.get(url).send().await?;

        let status = res.status();
        let body = res.json::<ApiData<ServerStatus, ()>>().await?;

        if !status.is_success() {
            // NB: Is the HTTP status is not 200, unwrapping is fine
            let api_error = body.error.unwrap();
            bail!(api_error.message);
        } else {
            // NB: Is the HTTP status is 200, unwrapping is fine
            Ok(body.data.unwrap())
        }
    }
}
