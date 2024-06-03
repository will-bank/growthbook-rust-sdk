use crate::env::Environment;
use crate::error::GrowthbookError;
use crate::gateway::GrowthbookGateway;
use crate::growthbook::Growthbook;
use crate::model::{BooleanFlag, Flag, ObjectFlag, StringFlag};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info};

#[derive(Clone)]
pub struct GrowthBookClient {
    pub gb: Arc<RwLock<Growthbook>>,
}

async fn updated_features_task(
    growthbook_gateway: GrowthbookGateway,
    config: Arc<RwLock<Growthbook>>,
    interval: Duration,
) {
    loop {
        match growthbook_gateway.get_features(None).await {
            Ok(new_config) => {
                let mut writable_config =
                    config.write().expect("problem to create mutex for gb data");
                let updated_features = Growthbook {
                    features: new_config.features,
                };
                *writable_config = updated_features;
                info!("[growthbook-sdk] features from growthbook was updated.");
            }
            Err(e) => {
                error!(
                    "[growthbook-sdk] Failed to fetch features from server: {:?}",
                    e
                );
            }
        }
        sleep(interval).await;
    }
}

impl GrowthBookClient {
    pub async fn new(
        api_url: &str,
        sdk_key: &str,
        update_interval: Option<Duration>,
        http_timeout: Option<Duration>,
    ) -> Result<Self, GrowthbookError> {
        let default_interval = update_interval.unwrap_or_else(|| {
            let seconds = Environment::u64_or_default("GB_UPDATE_INTERVAL", 60);
            Duration::from_secs(seconds)
        });
        let default_timeout = http_timeout.unwrap_or_else(|| {
            let seconds = Environment::u64_or_default("GB_HTTP_CLIENT_TIMEOUT", 10);
            Duration::from_secs(seconds)
        });
        let gb_gateway = GrowthbookGateway::new(api_url, sdk_key, default_timeout)?;
        let resp = gb_gateway.get_features(None).await?;
        let growthbook_writable = Arc::new(RwLock::new(Growthbook {
            features: resp.features,
        }));
        let gb_rw_clone = Arc::clone(&growthbook_writable);

        tokio::spawn(async move {
            updated_features_task(gb_gateway, gb_rw_clone, default_interval).await;
        });

        Ok(GrowthBookClient {
            gb: growthbook_writable,
        })
    }

    pub fn is_on(
        &self,
        feature_name: &str,
        default_response: bool,
        user_attributes: Option<&HashMap<String, Vec<String>>>,
    ) -> Result<BooleanFlag, GrowthbookError> {
        let flag =
            self.read_gb()
                .check(feature_name, Value::Bool(default_response), user_attributes);

        match flag {
            Flag::Boolean(it) => Ok(it),
            it => Err(GrowthbookError::invalid_response_value_type(it, "boolean")),
        }
    }

    pub fn get_string_value(
        &self,
        feature_name: &str,
        default_response: &str,
        user_attributes: Option<&HashMap<String, Vec<String>>>,
    ) -> Result<StringFlag, GrowthbookError> {
        let flag = self.read_gb().check(
            feature_name,
            Value::String(String::from(default_response)),
            user_attributes,
        );

        match flag {
            Flag::String(it) => Ok(it),
            it => Err(GrowthbookError::invalid_response_value_type(it, "String")),
        }
    }

    pub fn get_object_value(
        &self,
        feature_name: &str,
        default_response: &Value,
        user_attributes: Option<&HashMap<String, Vec<String>>>,
    ) -> Result<ObjectFlag, GrowthbookError> {
        let flag = self
            .read_gb()
            .check(feature_name, default_response.clone(), user_attributes);

        match flag {
            Flag::Object(it) => Ok(it),
            it => Err(GrowthbookError::invalid_response_value_type(it, "Object")),
        }
    }

    pub fn total_features(&self) -> usize {
        let gb_data = self.read_gb();
        gb_data.features.len()
    }

    fn read_gb(&self) -> Growthbook {
        match self.gb.read() {
            Ok(rw_read_guard) => (*rw_read_guard).clone(),
            Err(e) => {
                error!(
                    "{}",
                    format!(
                        "[growthbook-sdk] problem to reading gb mutex data returning empty {:?}",
                        e
                    )
                );
                Growthbook {
                    features: HashMap::new(),
                }
            }
        }
    }
}
