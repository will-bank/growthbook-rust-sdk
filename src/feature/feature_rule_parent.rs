use crate::condition::use_case::ConditionsMatchesAttributes;
use crate::dto::GrowthBookFeatureRuleParentData;
use crate::model_private::Feature;
use crate::model_public::{GrowthBookAttribute, GrowthBookAttributeValue};

impl GrowthBookFeatureRuleParentData {
    pub fn is_met(
        &self,
        feature: Feature,
    ) -> bool {
        if let Some(feature_attributes) = self.conditions() {
            feature_attributes.matches(&vec![GrowthBookAttribute::new(String::from("value"), GrowthBookAttributeValue::from(feature.value))])
        } else {
            true
        }
    }
}
