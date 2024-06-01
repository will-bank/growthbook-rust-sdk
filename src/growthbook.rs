use std::collections::HashMap;

use serde_json::Value;
use tracing::{error, info};

use crate::env::Environment;
use crate::error::GrowthbookError;
use crate::gateway::GrowthbookGateway;
use crate::model::{BooleanFlag, Flag, FlagCreator};

pub struct Growthbook {
    gateway: GrowthbookGateway,
}

impl Growthbook {
    pub fn new() -> Result<Self, GrowthbookError> {
        let url = Environment::string("GROWTHBOOK_URL")?;
        let request_timeout =
            Environment::u64_or_default("GROWTHBOOK_REQUEST_TIMEOUT_IN_MILLIS", 500);

        Ok(Self {
            gateway: GrowthbookGateway::new(&url, request_timeout)?,
        })
    }

    pub async fn is_on(
        &self,
        sdk_key: &str,
        flag_name: &str,
        default_response: bool,
        user_attributes: Option<&HashMap<String, Vec<String>>>,
    ) -> BooleanFlag {
        let flag = self
            .check(
                sdk_key,
                flag_name,
                Value::Bool(default_response),
                false,
                user_attributes,
            )
            .await;

        match flag {
            Flag::BooleanFlag(it) => it,
        }
    }

    async fn check(
        &self,
        sdk_key: &str,
        flag_name: &str,
        default_response: Value,
        _hot: bool,
        user_attributes: Option<&HashMap<String, Vec<String>>>,
    ) -> Flag {
        let feature_result = self.gateway.get_features(sdk_key).await;
        match feature_result {
            Ok(feature_response) => {
                let optional_feature = feature_response
                    .features
                    .iter()
                    .find(|(key, _)| key.as_str() == flag_name);

                if let Some((_, feature)) = optional_feature {
                    let (value, experiment_key) = feature.get_value(flag_name, user_attributes);
                    value.create_flag(experiment_key)
                } else {
                    info!(
                        "Feature {flag_name} not found, returning default value={default_response}"
                    );
                    default_response.create_flag(None)
                }
            }
            Err(err) => {
                error!(
                    "Failed when try get feature on growthbook named {}. Error: {}",
                    flag_name, err
                );
                default_response.create_flag(None)
            }
        }
    }
}
