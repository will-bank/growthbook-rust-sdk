use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use reqwest::header::USER_AGENT;
use reqwest_middleware::ClientWithMiddleware;
use tokio::sync::Mutex;

use crate::dto::GrowthBookResponse;
use crate::env::Environment;
use crate::error::GrowthbookError;
use crate::infra::HttpClient;

#[derive(Clone, Debug)]
pub struct GrowthbookGateway {
    pub url: String,
    pub user_agent: String,
    pub cache_duration: Duration,
    pub client: ClientWithMiddleware,
}

#[derive(Clone)]
struct CacheData {
    ttl: DateTime<Utc>,
    data: GrowthBookResponse,
}

impl Default for CacheData {
    fn default() -> Self {
        CacheData {
            ttl: Utc::now() - Duration::from_secs(10),
            data: Default::default(),
        }
    }
}

lazy_static! {
    static ref CACHE: Arc<Mutex<CacheData>> = Arc::new(Mutex::default());
}

impl GrowthbookGateway {
    pub fn new(
        url: &str,
        timeout_duration: Duration,
        cache_duration: Duration,
    ) -> Result<Self, GrowthbookError> {
        Ok(Self {
            url: String::from(url),
            cache_duration,
            user_agent: format!(
                "{}/{}",
                Environment::string_or_default("CARGO_PKG_NAME", "growthbook-rust-sdk"),
                Environment::string_or_default("CARGO_PKG_VERSION", "1.0.0")
            ),
            client: HttpClient::create_http_client("growthbook", timeout_duration)
                .map_err(GrowthbookError::from)?,
        })
    }

    pub async fn get_features(&self, sdk_key: &str) -> Result<GrowthBookResponse, GrowthbookError> {
        let mut cache = CACHE.lock().await;
        if cache.ttl.gt(&Utc::now()) {
            return Ok(cache.data.clone());
        }

        let url = format!("{}/api/features/{}", self.url, sdk_key);
        let send_result = self
            .client
            .get(url.clone())
            .header(USER_AGENT, self.user_agent.clone())
            .send()
            .await
            .map_err(GrowthbookError::from)?;

        let response = send_result
            .json::<GrowthBookResponse>()
            .await
            .map_err(GrowthbookError::from)?;

        cache.data = response.clone();
        cache.ttl = Utc::now() + self.cache_duration;

        Ok(response)
    }
}
