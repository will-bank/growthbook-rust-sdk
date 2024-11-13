use std::time::Duration;

use reqwest::header::USER_AGENT;
use reqwest_middleware::ClientWithMiddleware;

use crate::dto::GrowthBookResponse;
use crate::env::Environment;
use crate::error::{GrowthbookError, GrowthbookErrorCode};
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
            client: HttpClient::create_http_client("growthbook", timeout).map_err(GrowthbookError::from)?,
            sdk_key: sdk_key.to_string(),
        })
    }

    pub async fn get_features(
        &self,
        sdk_key: Option<&str>,
    ) -> Result<GrowthBookResponse, GrowthbookError> {
        let key = sdk_key.unwrap_or(self.sdk_key.as_str());
        let url = url::Url::parse(self.url.as_str())
            .map_err(|_| GrowthbookError::new(GrowthbookErrorCode::GenericError, "cannot parse url"))?;
        let url = url.join(&format!("/api/features/{}", key))
            .map_err(|_| GrowthbookError::new(GrowthbookErrorCode::GenericError, "cannot join url with api path"))?;

        let url = format!("{}", url);
        let send_result = self.client.get(url).header(USER_AGENT, self.user_agent.clone()).send().await.map_err(GrowthbookError::from)?;

        let response = send_result.json::<GrowthBookResponse>().await.map_err(GrowthbookError::from)?;

        Ok(response)
    }
}
