use std::time::Duration;

use crate::error::GrowthbookError;
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Extension};
use reqwest_tracing::{OtelName, TracingMiddleware};

pub struct HttpClient;

impl HttpClient {
    pub fn create_http_client(
        name: &str,
        request_timeout: u64,
    ) -> Result<ClientWithMiddleware, GrowthbookError> {
        let client = ClientBuilder::new(
            Client::builder()
                .timeout(Duration::from_millis(request_timeout))
                .build()
                .map_err(GrowthbookError::from)?,
        )
        .with_init(Extension(OtelName(String::from(name).into())))
        .with(TracingMiddleware::default())
        .build();
        Ok(client)
    }
}
