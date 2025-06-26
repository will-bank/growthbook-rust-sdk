#[cfg(feature = "mocker")]
use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener};
use std::time::Duration;

use crate::client::GrowthBookClient;
use crate::error::GrowthbookError;
use rand::Rng;
use serde_json::{Value, json};
use uuid::Uuid;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[cfg(feature = "mocker")]
pub struct GrowthBookTest {
    mock_server: MockServer,
    client: GrowthBookClient,
    sdk_key: Uuid,
    features: HashMap<String, Value>,
}

#[cfg(feature = "mocker")]
impl GrowthBookTest {
    pub async fn new() -> Result<Self, GrowthbookError> {
        let mock_server = Self::create_mock_server().await;
        let sdk_key = Uuid::now_v7();
        let features = HashMap::new();

        // Configura o mock inicial
        let body = json!({
            "status": 200,
            "features": features,
            "dateUpdated": "2024-05-29T18:43:22.153Z"
        });

        Mock::given(method("GET"))
            .and(path(format!("/api/features/{}", sdk_key)))
            .respond_with(ResponseTemplate::new(200).set_body_string(body.to_string()))
            .expect(1..) // accept many requests
            .mount(&mock_server)
            .await;

        let client = GrowthBookClient::new(
            &mock_server.uri(),
            &sdk_key.to_string(),
            Some(Duration::from_millis(100)), // short interval for tests and make the things "atomic"
            None,
        )
        .await?;

        // wait time for the first time to get the features
        tokio::time::sleep(Duration::from_millis(150)).await;

        Ok(GrowthBookTest {
            mock_server,
            client,
            sdk_key,
            features,
        })
    }

    pub async fn add_simple_flag(
        &mut self,
        name: &str,
        default_value: bool,
    ) -> &mut Self {
        let feature = json!({
            "defaultValue": default_value
        });
        self.features.insert(name.to_string(), feature);
        self.update_mock().await;
        self
    }

    pub async fn add_fixed_value_flag<T: serde::Serialize>(
        &mut self,
        name: &str,
        value: T,
    ) -> &mut Self {
        let feature = json!({
            "defaultValue": value
        });
        self.features.insert(name.to_string(), feature);
        self.update_mock().await;
        self
    }

    pub async fn add_conditional_flag(
        &mut self,
        name: &str,
        default_value: bool,
        condition: Value,
        force_value: bool,
    ) -> &mut Self {
        let feature = json!({
            "defaultValue": default_value,
            "rules": [
                {
                    "condition": condition,
                    "force": force_value
                }
            ]
        });
        self.features.insert(name.to_string(), feature);
        self.update_mock().await;
        self
    }

    pub async fn add_rollout_flag(
        &mut self,
        name: &str,
        default_value: bool,
        coverage: f64,
        hash_attribute: &str,
        force_value: bool,
    ) -> &mut Self {
        let feature = json!({
            "defaultValue": default_value,
            "rules": [
                {
                    "force": force_value,
                    "coverage": coverage,
                    "hashAttribute": hash_attribute
                }
            ]
        });
        self.features.insert(name.to_string(), feature);
        self.update_mock().await;
        self
    }

    pub async fn add_experiment_flag<T: serde::Serialize>(
        &mut self,
        name: &str,
        default_value: T,
        hash_attribute: &str,
        variations: Vec<T>,
        weights: Vec<f64>,
        coverage: f64,
    ) -> &mut Self {
        let meta: Vec<_> = (0..variations.len()).map(|i| json!({ "key": i.to_string() })).collect();

        let feature = json!({
            "defaultValue": default_value,
            "rules": [
                {
                    "coverage": coverage,
                    "hashAttribute": hash_attribute,
                    "seed": "test-seed",
                    "hashVersion": 2,
                    "variations": variations,
                    "weights": weights,
                    "key": format!("{}-experiment", name),
                    "meta": meta,
                    "phase": "1"
                }
            ]
        });
        self.features.insert(name.to_string(), feature);
        self.update_mock().await;
        self
    }

    pub async fn remove_flag(
        &mut self,
        name: &str,
    ) -> &mut Self {
        self.features.remove(name);
        self.update_mock().await;
        self
    }

    pub async fn clear_flags(&mut self) -> &mut Self {
        self.features.clear();
        self.update_mock().await;
        self
    }

    pub fn client(&self) -> &GrowthBookClient {
        &self.client
    }

    pub fn mock_uri(&self) -> String {
        self.mock_server.uri()
    }

    async fn update_mock(&self) {
        let body = json!({
            "status": 200,
            "features": self.features,
            "dateUpdated": "2024-05-29T18:43:22.153Z"
        });

        self.mock_server.reset().await;

        Mock::given(method("GET"))
            .and(path(format!("/api/features/{}", self.sdk_key)))
            .respond_with(ResponseTemplate::new(200).set_body_string(body.to_string()))
            .expect(1..)
            .mount(&self.mock_server)
            .await;

        tokio::time::sleep(Duration::from_millis(150)).await;
    }

    async fn create_mock_server() -> MockServer {
        for _ in 1..10 {
            let port = rand::rng().random_range(51000..54000);
            let addr = SocketAddr::from(([0, 0, 0, 0], port));
            if let Ok(listener) = TcpListener::bind(addr) {
                return MockServer::builder().listener(listener).start().await;
            }
        }
        panic!("failed to create mock server for growth book mocker");
    }
}
