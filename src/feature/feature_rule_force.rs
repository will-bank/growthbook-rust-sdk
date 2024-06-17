use serde_json::Value;

use crate::condition::use_case::ConditionsMatchesAttributes;
use crate::coverage::model::Coverage;
use crate::dto::GrowthBookFeatureRuleForce;
use crate::extensions::{FindGrowthBookAttribute, JsonHelper};
use crate::hash::{HashCode, HashCodeVersion};
use crate::model_private::Feature;
use crate::model_public::GrowthBookAttribute;
use crate::range::model::Range;

impl GrowthBookFeatureRuleForce {
    pub fn get_match_value(
        &self,
        feature_name: &str,
        option_user_attributes: &Option<Vec<GrowthBookAttribute>>,
    ) -> Option<Feature> {
        if let Some(feature_attributes) = self.conditions() {
            if let Some(user_attributes) = &option_user_attributes {
                if feature_attributes.matches(user_attributes) {
                    if let Some(filters) = &self.filters {
                        if self.is_filtered_out(filters, option_user_attributes) {
                            return None;
                        }
                    }

                    if let Some(range) = self.range() {
                        let fallback_attribute = self.get_fallback_attribute();
                        if let Some(user_value) = user_attributes.find_value(&fallback_attribute) {
                            let seed = self.seed.clone().unwrap_or(feature_name.to_string());
                            return Coverage::check(&user_value, None, Some(range), &seed, self.hash_version, self.force.clone());
                        }
                    } else {
                        return Some(Feature::force(self.force.clone()));
                    }
                }
            }
        } else {
            if let Some(filters) = &self.filters {
                if self.is_filtered_out(filters, option_user_attributes) {
                    return None;
                }
            }

            if let Some(range) = self.range() {
                if let Some(user_attributes) = &option_user_attributes {
                    let fallback_attribute = self.get_fallback_attribute();
                    if let Some(user_value) = user_attributes.find_value(&fallback_attribute) {
                        let seed = self.seed.clone().unwrap_or(feature_name.to_string());
                        return Coverage::check(&user_value, None, Some(range), &seed, self.hash_version, self.force.clone());
                    }
                }
            } else {
                return Some(Feature::force(self.force.clone()));
            }
        }

        None
    }

    fn is_filtered_out(
        &self,
        filters: &Value,
        option_user_attributes: &Option<Vec<GrowthBookAttribute>>,
    ) -> bool {
        for filter in filters.force_array() {
            let fallback_attribute = self.get_fallback_attribute();
            if let Some(user_attributes) = &option_user_attributes {
                if let Some(user_value) = user_attributes.find_value(&fallback_attribute) {
                    if let Some(user_weight) = HashCode::hash_code(
                        &user_value.to_string(),
                        &filter.get_string("seed"),
                        HashCodeVersion::from(filter.get("hashVersion").unwrap_or(&Value::from(2)).as_i64().expect("Failed to convert to i64")),
                    ) {
                        for array in filter.get_array("ranges") {
                            let range = Range {
                                start: array[0].force_f32(),
                                end: array[1].force_f32(),
                            };
                            if range.in_range(&user_weight) {
                                return false;
                            }
                        }
                    }
                } else {
                    return true;
                }
            } else {
                return false;
            }
        }
        return true;
    }
}
