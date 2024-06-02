use std::collections::HashMap;
use std::time::Duration;

use serde_json::Value;
use tracing::{error, info};

use crate::error::GrowthbookError;
use crate::gateway::GrowthbookGateway;
use crate::model::{BooleanFlag, Flag, FlagCreator, ObjectFlag, StringFlag};

pub struct Growthbook {
    gateway: GrowthbookGateway,
}

impl Growthbook {
    pub fn new(
        url: &str,
        timeout_duration: Duration,
        cache_duration: Duration,
    ) -> Result<Self, GrowthbookError> {
        Ok(Self {
            gateway: GrowthbookGateway::new(url, timeout_duration, cache_duration)?,
        })
    }

    pub async fn is_on(
        &self,
        sdk_key: &str,
        flag_name: &str,
        default_response: bool,
        user_attributes: Option<&HashMap<String, Vec<String>>>,
    ) -> Result<BooleanFlag, GrowthbookError> {
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
            Flag::BooleanFlag(it) => Ok(it),
            it => Err(GrowthbookError::invalid_response_value_type(it, "boolean")),
        }
    }

    pub async fn get_string_value(
        &self,
        sdk_key: &str,
        flag_name: &str,
        default_response: &str,
        user_attributes: Option<&HashMap<String, Vec<String>>>,
    ) -> Result<StringFlag, GrowthbookError> {
        let flag = self
            .check(
                sdk_key,
                flag_name,
                Value::String(String::from(default_response)),
                false,
                user_attributes,
            )
            .await;

        match flag {
            Flag::StringFlag(it) => Ok(it),
            it => Err(GrowthbookError::invalid_response_value_type(it, "String")),
        }
    }

    pub async fn get_object_value(
        &self,
        sdk_key: &str,
        flag_name: &str,
        default_response: &Value,
        user_attributes: Option<&HashMap<String, Vec<String>>>,
    ) -> Result<ObjectFlag, GrowthbookError> {
        let flag = self
            .check(
                sdk_key,
                flag_name,
                default_response.clone(),
                false,
                user_attributes,
            )
            .await;

        match flag {
            Flag::ObjectFlag(it) => Ok(it),
            it => Err(GrowthbookError::invalid_response_value_type(it, "Object")),
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
