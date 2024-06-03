use reqwest::header::USER_AGENT;
use reqwest::{Client, StatusCode};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

use crate::dto::GrowthBookResponse;
use crate::env::Environment;
use crate::error::GrowthbookError;

#[derive(Clone, Debug)]
pub struct GBGateway {
    pub url: String,
    pub client: ClientWithMiddleware,
    is_test: bool,
}

impl GBGateway {
    pub fn new(url: &str) -> Result<Self, GrowthbookError> {
        Ok(Self {
            url: String::from(url),
            client: ClientBuilder::new(Client::new()).build(),
            is_test: Environment::boolean_or_default("is-test-environment", false),
        })
    }

    pub async fn get_features(&self, sdk_key: &str) -> Result<GrowthBookResponse, GrowthbookError> {
        let uri = format!("{0}/api/features/{sdk_key}", self.url);

        let result = self
            .client
            .get(uri)
            .header(USER_AGENT, "rustzinho/sdk")
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

