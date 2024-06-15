use serde_json::Value;

use crate::dto::GrowthBookFeatureRuleExperiment;
use crate::extensions::FindGrowthBookAttribute;
use crate::hash::{HashCode, HashCodeVersion};
use crate::model_public::GrowthBookAttribute;
use crate::range::model::Range;

impl GrowthBookFeatureRuleExperiment {
    pub fn get_match_value(
        &self,
        _feature_name: &str,
        option_user_attributes: Option<&Vec<GrowthBookAttribute>>,
    ) -> Option<(Value, String)> {
        if let Some(user_attributes) = option_user_attributes {
            if let Some(user_value) = user_attributes.find_value(&self.hash_attribute) {
                let weights = self.weights();
                if self.is_valid_experiment(&weights) {
                    for (index, percentage) in weights.iter().enumerate() {
                        let user_weight_position = HashCode::hash_code(&user_value.to_string(), &self.seed(), HashCodeVersion::from(self.hash_version)).unwrap_or(-1.0);
                        if percentage.in_range(&user_weight_position) {
                            return Some((self.variations[index].clone(), self.meta[index].key.clone()));
                        }
                    }
                }
            }
        }

        None
    }

    fn is_valid_experiment(
        &self,
        weights: &[Range],
    ) -> bool {
        weights.len() == self.variations.len() && weights.len() == self.meta.len()
    }
}
