use std::time::Duration;

use reqwest::header::USER_AGENT;
use reqwest_middleware::ClientWithMiddleware;
use tracing::debug;
use crate::dto::GrowthBookResponse;
use crate::env::Environment;
use crate::error::GrowthbookError;
use crate::infra::HttpClient;

#[derive(Clone, Debug)]
pub struct GrowthbookGateway {
    pub url: String,
    pub user_agent: String,
    sdk_key: String,
    pub client: ClientWithMiddleware,
}
impl GrowthbookGateway {
    pub fn new(
        url: &str,
        sdk_key: &str,
        timeout: Duration,
    ) -> Result<Self, GrowthbookError> {
        Ok(Self {
            url: String::from(url),
            user_agent: format!(
                "{}/{}",
                Environment::string_or_default("CARGO_PKG_NAME", "growthbook-rust-sdk"),
                Environment::string_or_default("CARGO_PKG_VERSION", "1.0.0")
            ),
            client: HttpClient::create_http_client("growthbook", timeout)?,
            sdk_key: sdk_key.to_string(),
        })
    }

    pub async fn get_features(
        &self,
        sdk_key: Option<&str>,
    ) -> Result<GrowthBookResponse, GrowthbookError> {
        let sdk = sdk_key.unwrap_or(self.sdk_key.as_str());
        let url = format!("{}/api/features/{}", self.url, sdk);
        let send_result = self.client.get(url).header(USER_AGENT, self.user_agent.clone()).send().await.map_err(GrowthbookError::from)?;
        let status_code_response = send_result.status().as_u16();
        let text_response = send_result.text().await.map_err(GrowthbookError::from)?;
        debug!("growthbook raw response : {} status code: {}", text_response, status_code_response);
        //let response = send_result.json::<GrowthBookResponse>().await.map_err(GrowthbookError::from)?;
        let response = serde_json::from_str(&text_response).map_err(GrowthbookError::from)?;

        Ok(response)
    }
}
