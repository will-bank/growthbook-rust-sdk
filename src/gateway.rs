use reqwest::header::USER_AGENT;
use reqwest::StatusCode;
use reqwest_middleware::ClientWithMiddleware;

use crate::dto::GrowthBookResponse;
use crate::env::Environment;
use crate::error::GrowthbookError;
use crate::infra::HttpClient;

#[derive(Clone, Debug)]
pub struct GrowthbookGateway {
    pub url: String,
    pub user_agent: String,
    pub client: ClientWithMiddleware,
}

impl GrowthbookGateway {
    pub fn new(url: &str, request_timeout: u64) -> Result<Self, GrowthbookError> {
        Ok(Self {
            url: String::from(url),
            user_agent: format!(
                "{}/{}",
                Environment::string_or_default("CARGO_PKG_NAME", "growthbook-rust-sdk"),
                Environment::string_or_default("CARGO_PKG_VERSION", "1.0.0")
            ),
            client: HttpClient::create_http_client("growthbook", request_timeout)
                .map_err(GrowthbookError::from)?,
        })
    }

    pub async fn get_features(&self, sdk_key: &str) -> Result<GrowthBookResponse, GrowthbookError> {
        let url = format!("{}/api/features/{}", self.url, sdk_key);
        let result = self
            .client
            .get(url.clone())
            .header(USER_AGENT, self.user_agent.clone())
            .send()
            .await
            .map_err(GrowthbookError::from)?;

        match result.status() {
            StatusCode::OK => result
                .json::<GrowthBookResponse>()
                .await
                .map_err(GrowthbookError::from),
            _ => Err(GrowthbookError::from(result)),
        }
    }
}
