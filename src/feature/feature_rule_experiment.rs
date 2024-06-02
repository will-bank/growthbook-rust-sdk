use std::collections::HashMap;

use serde_json::Value;

use crate::dto::FeatureRuleExperiment;
use crate::extensions::FoldVecString;
use crate::hash::Hasher;

impl FeatureRuleExperiment {
    pub fn get_match_value(
        &self,
        flag_name: &str,
        user_attributes: Option<&HashMap<String, Vec<String>>>,
    ) -> Option<(Value, String)> {
        if let Some(attributes) = &user_attributes.clone() {
            if let Some(attribute) = attributes.get(&self.hash_attribute) {
                let user_attribute_string = attribute.fold_to_string();
                let user_coverage_position = Hasher::hash_code(&user_attribute_string);
                if self.coverage.gt(&user_coverage_position) {
                    for (index, percentage) in self.weights().iter().enumerate() {
                        let user_weight_position =
                            Hasher::hash_code(&format!("{flag_name}:{user_attribute_string}"));
                        if percentage.gt(&user_weight_position) {
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
}
