use std::collections::HashMap;

use serde_json::Value;

use crate::dto::FeatureRuleRollout;
use crate::extensions::FoldVecString;
use crate::feature::condition::ConditionEnabledCheck;
use crate::hash::{HashCode, HashCodeVersion};

impl FeatureRuleRollout {
    pub fn get_match_value(
        &self,
        feature_name: &str,
        user_attributes: Option<&HashMap<String, Vec<String>>>,
    ) -> Option<Value> {
        if let Some(conditions) = &self.condition {
            if conditions.is_on(user_attributes) {
                if let Some(attributes) = user_attributes {
                    if let Some(attribute) = attributes.get(&self.hash_attribute) {
                        return self.check_coverage(attribute, feature_name);
                    }
                }
            }
        } else if let Some(attributes) = user_attributes {
            if let Some(attribute) = attributes.get(&self.hash_attribute) {
                return self.check_coverage(attribute, feature_name);
            }
        }

        None
    }

    fn check_coverage(&self, attribute: &Vec<String>, feature_name: &str) -> Option<Value> {
        if self.coverage.gt(&HashCode::hash_code(
            &attribute.fold_to_string(),
            feature_name,
            HashCodeVersion::V1,
        )) {
            Some(self.force.clone())
        } else {
            None
        }
    }
}
