use std::time::Duration;

use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Extension};
use reqwest_tracing::{OtelName, TracingMiddleware};

use crate::error::GrowthbookError;

pub struct HttpClient;

impl HttpClient {
    pub fn create_http_client(
        name: &str,
        timeout_duration: Duration,
    ) -> Result<ClientWithMiddleware, GrowthbookError> {
        let client = ClientBuilder::new(
            Client::builder()
                .timeout(timeout_duration)
                .build()
                .map_err(GrowthbookError::from)?,
        )
        .with_init(Extension(OtelName(String::from(name).into())))
        .with(TracingMiddleware::default())
        .build();
        Ok(client)
    }
}
