use serde_json::Value;
use tracing::info;

use crate::dto::{GrowthBookFeature, GrowthBookFeatureRule};
use crate::model_public::GrowthBookAttribute;

impl GrowthBookFeature {
    pub fn get_value(
        &self,
        feature_name: &str,
        user_attributes: Option<&Vec<GrowthBookAttribute>>,
    ) -> (Value, Option<String>) {
        if let Some(rules) = &self.rules {
            for rule in rules {
                match rule {
                    GrowthBookFeatureRule::Force(it) => {
                        if let Some(value) = it.get_match_value(user_attributes) {
                            info!(
                                "Feature {feature_name} value={} for forced rule",
                                self.default_value
                            );
                            return (value, None);
                        }
                    }
                    GrowthBookFeatureRule::Rollout(it) => {
                        if let Some(value) = it.get_match_value(feature_name, user_attributes) {
                            info!(
                                "Feature {feature_name} value={} for rollout",
                                self.default_value
                            );
                            return (value, None);
                        }
                    }
                    GrowthBookFeatureRule::Experiment(it) => {
                        if let Some((value, experiment_key)) =
                            it.get_match_value(feature_name, user_attributes)
                        {
                            info!(
                                "Feature {feature_name} value={} for experiment",
                                self.default_value
                            );
                            return (value, Some(experiment_key));
                        }
                    }
                }
            }
        }

        info!("Feature {feature_name} value={}", self.default_value);
        (self.default_value.clone(), None)
    }
}
