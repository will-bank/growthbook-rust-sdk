use serde_json::Value;

use crate::condition::use_case::ConditionsMatchesAttributes;
use crate::dto::GrowthBookFeatureRuleForce;
use crate::model_public::GrowthBookAttribute;

impl GrowthBookFeatureRuleForce {
    pub fn get_match_value(
        &self,
        option_user_attributes: Option<&Vec<GrowthBookAttribute>>,
    ) -> Option<Value> {
        if let Some(feature_attributes) = self.conditions() {
            if let Some(user_attributes) = &option_user_attributes {
                if feature_attributes.matches(user_attributes) {
                    return Some(self.force.clone());
                }
            }
        } else {
            return Some(self.force.clone());
        }

        None
    }
}
