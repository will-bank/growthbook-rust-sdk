use std::collections::HashMap;

use crate::dto::FeatureRuleCondition;
use crate::extensions::{ConvertToUsize, FoldVecString};

pub trait ConditionEnabledCheck {
    fn is_on(&self, user_attributes: Option<&HashMap<String, Vec<String>>>) -> bool;
}

impl ConditionEnabledCheck for HashMap<String, FeatureRuleCondition> {
    fn is_on(&self, user_attributes: Option<&HashMap<String, Vec<String>>>) -> bool {
        if let Some(user_map) = user_attributes {
            self.iter().all(|(key, value)| {
                if let Some(user_attribute) = user_map.get(key) {
                    value.matches(user_attribute)
                } else {
                    false
                }
            })
        } else {
            false
        }
    }
}

impl FeatureRuleCondition {
    fn matches(&self, user_attribute_value: &Vec<String>) -> bool {
        match self {
            FeatureRuleCondition::String(it) => it == &user_attribute_value.fold_to_string(),
            FeatureRuleCondition::Gte(it) => usize_matcher(
                user_attribute_value.fold_to_string(),
                it.gte.clone(),
                |user_attribute_value, gte_value| user_attribute_value >= gte_value,
            ),
            FeatureRuleCondition::Gt(it) => usize_matcher(
                user_attribute_value.fold_to_string(),
                it.gt.clone(),
                |user_attribute_value, gte_value| user_attribute_value > gte_value,
            ),
            FeatureRuleCondition::Lte(it) => usize_matcher(
                user_attribute_value.fold_to_string(),
                it.lte.clone(),
                |user_attribute_value, gte_value| user_attribute_value <= gte_value,
            ),
            FeatureRuleCondition::Lt(it) => usize_matcher(
                user_attribute_value.fold_to_string(),
                it.lt.clone(),
                |user_attribute_value, gte_value| user_attribute_value < gte_value,
            ),
            FeatureRuleCondition::ElemMatchEq(it) => {
                user_attribute_value.contains(&it.elem_match.eq)
            }
            FeatureRuleCondition::NotElemMatchEq(it) => {
                !user_attribute_value.contains(&it.not.elem_match.eq)
            }
            FeatureRuleCondition::Eq(it) => it.eq == user_attribute_value.fold_to_string(),
            FeatureRuleCondition::In(it) => {
                it.array.contains(&user_attribute_value.fold_to_string())
            }
        }
    }
}

fn usize_matcher(
    user_attribute_result: String,
    rule_result: String,
    evaluate: fn(usize, usize) -> bool,
) -> bool {
    if let Ok(user_attribute_value) = user_attribute_result.convert_to_usize() {
        if let Ok(rule_value) = rule_result.convert_to_usize() {
            evaluate(user_attribute_value, rule_value)
        } else {
            false
        }
    } else {
        false
    }
}
