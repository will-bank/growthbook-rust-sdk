use crate::condition::use_case::ConditionsMatchesAttributes;
use crate::coverage::model::Coverage;
use crate::dto::GrowthBookFeatureRuleRollout;
use crate::extensions::FindGrowthBookAttribute;
use crate::model_private::Feature;
use crate::model_public::GrowthBookAttribute;

impl GrowthBookFeatureRuleRollout {
    pub fn get_match_value(
        &self,
        feature_name: &str,
        option_user_attributes: &Option<Vec<GrowthBookAttribute>>,
    ) -> Option<Feature> {
        if let Some(feature_attributes) = &self.conditions() {
            if let Some(user_attributes) = option_user_attributes {
                if feature_attributes.matches(&user_attributes) {
                    if let Some(hash_attribute) = &self.hash_attribute {
                        if let Some(user_value) = user_attributes.find_value(hash_attribute) {
                            return Coverage::check(&user_value, Some(self.coverage), self.range(), feature_name, self.hash_version, self.force.clone());
                        } else {
                            let fallback_attribute = self.get_fallback_attribute();
                            if let Some(user_value) = user_attributes.find_value(&fallback_attribute) {
                                return Coverage::check(&user_value, Some(self.coverage), self.range(), feature_name, self.hash_version, self.force.clone());
                            }
                        }
                    } else {
                        let fallback_attribute = self.get_fallback_attribute();
                        if let Some(user_value) = user_attributes.find_value(&fallback_attribute) {
                            return Coverage::check(&user_value, Some(self.coverage), self.range(), feature_name, self.hash_version, self.force.clone());
                        }
                    }
                }
            }
        } else if let Some(user_attributes) = option_user_attributes {
            if let Some(hash_attribute) = &self.hash_attribute {
                if let Some(user_value) = user_attributes.find_value(hash_attribute) {
                    return Coverage::check(&user_value, Some(self.coverage), self.range(), feature_name, self.hash_version, self.force.clone());
                } else {
                    let fallback_attribute = self.get_fallback_attribute();
                    if let Some(user_value) = user_attributes.find_value(&fallback_attribute) {
                        return Coverage::check(&user_value, Some(self.coverage), self.range(), feature_name, self.hash_version, self.force.clone());
                    }
                }
            } else {
                let fallback_attribute = self.get_fallback_attribute();
                if let Some(user_value) = user_attributes.find_value(&fallback_attribute) {
                    return Coverage::check(&user_value, Some(self.coverage), self.range(), feature_name, self.hash_version, self.force.clone());
                }
            }
        }

        None
    }
}
