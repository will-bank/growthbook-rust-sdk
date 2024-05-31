use reqwest::header::CONTENT_TYPE;
use reqwest_middleware::ClientWithMiddleware;

use crate::error::GrowthbookError;
use crate::infra::HttpClient;
use crate::model::GrowthBookResponse;

#[derive(Clone, Debug)]
pub struct GrowthbookGateway {
    pub url: String,
    pub client: ClientWithMiddleware,
}

impl GrowthbookGateway {
    pub fn new(url: &str, request_timeout: u64) -> Result<Self, GrowthbookError> {
        Ok(Self {
            url: String::from(url),
            client: HttpClient::create_http_client("growthbook", request_timeout)
                .map_err(GrowthbookError::from)?,
        })
    }

    pub async fn get_features(&self, sdk_key: &str) -> Result<GrowthBookResponse, GrowthbookError> {
        let url = format!("{}/api/features/{}", self.url, sdk_key);
        let result = self
            .client
            .get(url.clone())
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await
            .map_err(GrowthbookError::from)?;

        result
            .json::<GrowthBookResponse>()
            .await
            .map_err(GrowthbookError::from)
    }
}
