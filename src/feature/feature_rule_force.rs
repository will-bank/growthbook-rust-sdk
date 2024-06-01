use std::collections::HashMap;

use serde_json::Value;

use crate::dto::FeatureRuleForce;
use crate::feature::condition::ConditionEnabledCheck;

impl FeatureRuleForce {
    pub fn get_match_value(
        &self,
        user_attributes: Option<&HashMap<String, Vec<String>>>,
    ) -> Option<Value> {
        if let Some(conditions) = &self.condition {
            if conditions.is_on(user_attributes) {
                return Some(self.force.clone());
            }
        }

        None
    }
}
