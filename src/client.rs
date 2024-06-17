use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use tokio::time::sleep;
use tracing::error;

use crate::env::Environment;
use crate::error::GrowthbookError;
use crate::gateway::GrowthbookGateway;
use crate::growthbook::GrowthBook;
use crate::model_private::FeatureResult;
use crate::model_public::GrowthBookAttribute;

#[derive(Clone)]
pub struct GrowthBookClient {
    pub gb: Arc<RwLock<GrowthBook>>,
}

async fn updated_features_task(
    growthbook_gateway: GrowthbookGateway,
    config: Arc<RwLock<GrowthBook>>,
    interval: Duration,
) {
    loop {
        match growthbook_gateway.get_features(None).await {
            Ok(new_config) => {
                let mut writable_config = config.write().expect("problem to create mutex for gb data");
                let updated_features = GrowthBook {
                    forced_variations: new_config.forced_variations,
                    features: new_config.features,
                };
                *writable_config = updated_features;
            },
            Err(e) => {
                error!("[growthbook-sdk] Failed to fetch features from server: {:?}", e);
            },
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
        let growthbook_writable = Arc::new(RwLock::new(GrowthBook {
            forced_variations: resp.forced_variations,
            features: resp.features,
        }));
        let gb_rw_clone = Arc::clone(&growthbook_writable);

        tokio::spawn(async move {
            updated_features_task(gb_gateway, gb_rw_clone, default_interval).await;
        });

        Ok(GrowthBookClient { gb: growthbook_writable })
    }

    pub fn is_on(
        &self,
        feature_name: &str,
        user_attributes: Option<Vec<GrowthBookAttribute>>,
    ) -> bool {
        self.read_gb().check(feature_name, &user_attributes).on
    }

    pub fn is_off(
        &self,
        feature_name: &str,
        user_attributes: Option<Vec<GrowthBookAttribute>>,
    ) -> bool {
        self.read_gb().check(feature_name, &user_attributes).off
    }

    pub fn feature_result(
        &self,
        feature_name: &str,
        user_attributes: Option<Vec<GrowthBookAttribute>>,
    ) -> FeatureResult {
        self.read_gb().check(feature_name, &user_attributes)
    }

    pub fn total_features(&self) -> usize {
        let gb_data = self.read_gb();
        gb_data.features.len()
    }

    fn read_gb(&self) -> GrowthBook {
        match self.gb.read() {
            Ok(rw_read_guard) => (*rw_read_guard).clone(),
            Err(e) => {
                error!("{}", format!("[growthbook-sdk] problem to reading gb mutex data returning empty {:?}", e));
                GrowthBook {
                    forced_variations: None,
                    features: HashMap::new(),
                }
            },
        }
    }
}
