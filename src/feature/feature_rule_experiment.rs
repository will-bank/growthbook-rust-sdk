use std::collections::HashMap;

use serde_json::Value;

use crate::dto::{FeatureRuleExperiment, FeatureRuleExperimentRange};
use crate::extensions::FoldVecString;
use crate::hash::{HashCode, HashCodeVersion};

impl FeatureRuleExperiment {
    pub fn get_match_value(
        &self,
        _feature_name: &str,
        user_attributes: Option<&HashMap<String, Vec<String>>>,
    ) -> Option<(Value, String)> {
        if let Some(attributes) = &user_attributes.clone() {
            if let Some(attribute) = attributes.get(&self.hash_attribute) {
                let user_attribute_string = attribute.fold_to_string();
                let weights = self.weights();
                if self.is_valid_experiment(&weights) {
                    for (index, percentage) in weights.iter().enumerate() {
                        let user_weight_position = HashCode::hash_code(
                            &user_attribute_string,
                            &self.seed(),
                            HashCodeVersion::from(self.hash_version),
                        );
                        if percentage.in_range(&user_weight_position) {
                            return Some((
                                self.variations[index].clone(),
                                self.meta[index].key.clone(),
                            ));
                        }
                    }
                }
            }
        }

        None
    }

    fn is_valid_experiment(&self, weights: &Vec<FeatureRuleExperimentRange>) -> bool {
        weights.len() == self.variations.len() && weights.len() == self.meta.len()
    }
}
