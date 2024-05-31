use std::collections::HashMap;

use tracing::{debug, error, info};

use crate::env::Environment;
use crate::error::GrowthbookError;
use crate::gateway::GrowthbookGateway;
use crate::model::{FlagState, GrowthBookResponse};
use crate::rule_checker::RuleChecker;

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
    ) -> FlagState {
        let feature_result = self.gateway.get_features(sdk_key).await;
        self.check(
            feature_result,
            flag_name,
            default_response,
            false,
            user_attributes,
        )
        .await
    }

    async fn check(
        &self,
        feature_result: Result<GrowthBookResponse, GrowthbookError>,
        flag_name: &str,
        default_response: bool,
        _hot: bool,
        user_attributes: Option<&HashMap<String, Vec<String>>>,
    ) -> FlagState {
        match feature_result {
            Ok(feature_response) => {
                let optional_feature = feature_response
                    .features
                    .iter()
                    .find(|(key, _)| key.as_str() == flag_name);

                if let Some((_, feature)) = optional_feature {
                    if let Some(rules) = &feature.rules {
                        let (flag_state, attributes) =
                            RuleChecker::check(feature, rules, user_attributes);
                        debug!(
                            "Feature {flag_name} enabled={:?} by attribute={:?}",
                            flag_state, attributes,
                        );
                        return flag_state;
                    }

                    debug!(
                        "Feature {flag_name} response with default value: {}",
                        feature.default_value
                    );
                    FlagState::new(feature.default_value, None)
                } else {
                    info!("Feature {flag_name}, returning default value={default_response}");
                    FlagState::new(default_response, None)
                }
            }
            Err(err) => {
                error!(
                    "Failed when try get feature on growthbook named {}. Error: {}",
                    flag_name, err
                );
                FlagState::new(default_response, None)
            }
        }
    }
}
