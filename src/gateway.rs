use cached::proc_macro::cached;
use once_cell::sync::Lazy;
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
    is_test: bool,
}

impl GrowthbookGateway {
    pub fn new(url: &str) -> Result<Self, GrowthbookError> {
        Ok(Self {
            url: String::from(url),
            is_test: Environment::boolean_or_default("is-test-environment", false),
        })
    }

    pub async fn get_features(&self, sdk_key: &str) -> Result<GrowthBookResponse, GrowthbookError> {
        if self.is_test {
            cached_fetch_features_for_test(self.url.clone(), sdk_key.to_string()).await
        } else {
            cached_fetch_features(self.url.clone(), sdk_key.to_string()).await
        }
    }
}

struct GatewayClient {
    pub user_agent: String,
    pub client: ClientWithMiddleware,
}

static GATEWAY: Lazy<GatewayClient> = Lazy::new(|| GatewayClient {
    user_agent: format!(
        "{}/{}",
        Environment::string_or_default("CARGO_PKG_NAME", "growthbook-rust-sdk"),
        Environment::string_or_default("CARGO_PKG_VERSION", "1.0.0")
    ),
    client: HttpClient::create_http_client(
        "growthbook",
        Environment::u64_or_default("GROWTHBOOK_TIMEOUT_IN_MILLIS", 1000),
    )
    .expect("Failed to create growthbook gateway client"),
});

#[cached(time = 360, result = true)]
async fn cached_fetch_features(
    url: String,
    sdk_key: String,
) -> Result<GrowthBookResponse, GrowthbookError> {
    try_fetch_features(url, sdk_key).await?
}

#[cached(time = 1, result = true)]
async fn cached_fetch_features_for_test(
    url: String,
    sdk_key: String,
) -> Result<GrowthBookResponse, GrowthbookError> {
    try_fetch_features(url, sdk_key).await?
}

async fn try_fetch_features(
    url: String,
    sdk_key: String,
) -> Result<Result<GrowthBookResponse, GrowthbookError>, GrowthbookError> {
    let uri = format!("{url}/api/features/{sdk_key}");

    let result = GATEWAY
        .client
        .get(uri)
        .header(USER_AGENT, GATEWAY.user_agent.clone())
        .send()
        .await
        .map_err(GrowthbookError::from)?;

    Ok(match result.status() {
        StatusCode::OK => result
            .json::<GrowthBookResponse>()
            .await
            .map_err(GrowthbookError::from),
        _ => Err(GrowthbookError::from(result)),
    })
}
