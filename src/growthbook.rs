use std::collections::HashMap;

use serde_json::Value;
use tracing::info;

use crate::dto::GrowthBookFeature;
use crate::model_private::{Feature, FeatureCreator};
use crate::model_public::GrowthBookAttribute;

#[derive(Clone)]
pub struct Growthbook {
    pub features: HashMap<String, GrowthBookFeature>,
}

impl Growthbook {
    pub fn check(
        &self,
        flag_name: &str,
        default_response: Value,
        user_attributes: Option<&Vec<GrowthBookAttribute>>,
    ) -> Feature {
        let optional_feature = self
            .features
            .iter()
            .find(|(key, _)| key.as_str() == flag_name);

        if let Some((_, feature)) = optional_feature {
            let (value, experiment_key) = feature.get_value(flag_name, user_attributes);
            value.create(experiment_key)
        } else {
            info!(
                        "[growthbook-sdk] Feature {flag_name} not found, returning default value={default_response}"
                    );
            default_response.create(None)
        }
    }
}
