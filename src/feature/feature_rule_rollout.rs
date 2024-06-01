use std::collections::HashMap;

use serde_json::Value;

use crate::dto::FeatureRuleRollout;
use crate::extensions::FoldVecString;
use crate::feature::condition::ConditionEnabledCheck;
use crate::hash::Hasher;

impl FeatureRuleRollout {
    pub fn get_match_value(
        &self,
        user_attributes: Option<&HashMap<String, Vec<String>>>,
    ) -> Option<Value> {
        if let Some(conditions) = &self.condition {
            if conditions.is_on(user_attributes) {
                if let Some(attributes) = user_attributes {
                    if let Some(attribute) = attributes.get(&self.hash_attribute) {
                        return self.check_coverage(attribute);
                    }
                }
            }
        } else if let Some(attributes) = user_attributes {
            if let Some(attribute) = attributes.get(&self.hash_attribute) {
                return self.check_coverage(attribute);
            }
        }

        None
    }

    fn check_coverage(&self, attribute: &Vec<String>) -> Option<Value> {
        if self
            .coverage
            .gt(&Hasher::hash_code(&attribute.fold_to_string()))
        {
            Some(self.force.clone())
        } else {
            None
        }
    }
}
