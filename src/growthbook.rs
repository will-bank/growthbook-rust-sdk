use std::collections::HashMap;

use crate::dto::Feature;
use crate::model::{Flag, FlagCreator};
use serde_json::Value;
use tracing::info;

#[derive(Clone)]
pub struct Growthbook {
    pub features: HashMap<String, Feature>,
}

impl Growthbook {
    pub fn check(
        &self,
        flag_name: &str,
        default_response: Value,
        user_attributes: Option<&HashMap<String, Vec<String>>>,
    ) -> Flag {
        let optional_feature = self
            .features
            .iter()
            .find(|(key, _)| key.as_str() == flag_name);

        if let Some((_, feature)) = optional_feature {
            let (value, experiment_key) = feature.get_value(flag_name, user_attributes);
            value.create_flag(experiment_key)
        } else {
            info!(
                        "[growthbook-sdk] Feature {flag_name} not found, returning default value={default_response}"
                    );
            default_response.create_flag(None)
        }
    }
}
