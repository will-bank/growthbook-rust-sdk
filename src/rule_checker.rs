use std::collections::HashMap;

use serde_json::Value;

use crate::extensions::{ConvertToString, ConvertToUsize, FoldVecString};
use crate::hash::Hasher;
use crate::model::{Feature, FeatureRule, FeatureRuleMeta, FlagState};

pub struct RuleChecker;

impl RuleChecker {
    pub fn check(
        feature: &Feature,
        feature_rules: &Vec<FeatureRule>,
        user_attributes: Option<&HashMap<String, Vec<String>>>,
    ) -> (FlagState, Option<String>) {
        let user_attributes_map = match user_attributes {
            None => return (FlagState::new(feature.default_value, None), None),
            Some(attributes) => attributes,
        };

        for feature_rule in feature_rules {
            match &feature_rule.condition {
                Some(feature_condition) => {
                    if check_if_all_conditions_satisfy(user_attributes_map, feature_condition) {
                        if let Some((enabled, conditions)) =
                            check_coverage(user_attributes_map, feature_rule)
                        {
                            return check_experiment(
                                user_attributes_map,
                                feature_rule,
                                enabled,
                                conditions,
                            );
                        }
                    }
                }
                None => {
                    if let Some((enabled, conditions)) =
                        check_coverage(user_attributes_map, feature_rule)
                    {
                        return check_experiment(
                            user_attributes_map,
                            feature_rule,
                            enabled,
                            conditions,
                        );
                    }
                }
            }
        }

        (FlagState::new(feature.default_value, None), None)
    }
}

fn check_if_all_conditions_satisfy(
    user_attributes_map: &HashMap<String, Vec<String>>,
    feature_condition: &HashMap<String, Value>,
) -> bool {
    feature_condition
        .iter()
        .all(|(key, value)| match user_attributes_map.get(key) {
            Some(user_condition) => {
                if value.is_string() {
                    check_string(user_condition, value)
                } else if let Some(gte) = value.get("$gte") {
                    check_gte(user_condition, gte)
                } else if let Some(gt) = value.get("$gt") {
                    check_gt(user_condition, gt)
                } else if let Some(gte) = value.get("$lte") {
                    check_lte(user_condition, gte)
                } else if let Some(gt) = value.get("$lt") {
                    check_lt(user_condition, gt)
                } else if let Some(not) = value.get("$not") {
                    if let Some(elem_match) = not.get("$elemMatch") {
                        if let Some(eq) = elem_match.get("$eq") {
                            !check_elem_match_eq(user_condition, eq)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else if let Some(elem_match) = value.get("$elemMatch") {
                    if let Some(eq) = elem_match.get("$eq") {
                        check_elem_match_eq(user_condition, eq)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            None => false,
        })
}

fn check_coverage(
    user_attributes_map: &HashMap<String, Vec<String>>,
    feature_rule: &FeatureRule,
) -> Option<(Option<bool>, Option<String>)> {
    if let Some(hash_attribute) = &feature_rule.hash_attribute {
        if let Some(user_hash_attribute) = user_attributes_map.get(hash_attribute) {
            if let Some(coverage) = &feature_rule.coverage {
                let hash_code = Hasher::hash_code(&user_hash_attribute.fold_to_string());
                if coverage > &hash_code {
                    return Some((
                        feature_rule.force,
                        Some(feature_rule.conditions_to_string()),
                    ));
                }
            } else {
                return Some((
                    feature_rule.force,
                    Some(feature_rule.conditions_to_string()),
                ));
            }
        }
    } else {
        return Some((
            feature_rule.force,
            Some(feature_rule.conditions_to_string()),
        ));
    }
    None
}

fn check_experiment(
    user_attributes_map: &HashMap<String, Vec<String>>,
    feature_rule: &FeatureRule,
    optional_enabled: Option<bool>,
    conditions: Option<String>,
) -> (FlagState, Option<String>) {
    let enabled = if let Some(enabled) = optional_enabled {
        if !enabled {
            return (FlagState::new(false, None), conditions);
        }
        enabled
    } else {
        false
    };

    let weights = match feature_rule.weights() {
        Some(weights) => weights,
        None => return (FlagState::new(enabled, None), conditions),
    };

    let metas = match &feature_rule.meta {
        Some(metas) => metas,
        None => return (FlagState::new(enabled, None), conditions),
    };

    let hash_attribute = match &feature_rule.hash_attribute {
        Some(hash_attribute) => hash_attribute,
        None => return (FlagState::new(enabled, None), conditions),
    };

    let user_hash_attribute = match user_attributes_map.get(hash_attribute) {
        Some(user_hash_attribute) => user_hash_attribute,
        None => return (FlagState::new(enabled, None), conditions),
    };

    let hash_code = Hasher::hash_code(&user_hash_attribute.fold_to_string());

    let index = match weights.iter().position(|w| w > &hash_code) {
        Some(index) => index,
        None => return (FlagState::new(enabled, None), conditions),
    };

    let variations = match &feature_rule.variations {
        Some(variations) => variations,
        None => return (FlagState::new(enabled, None), conditions),
    };

    let experiment_enabled = match variations.get(index) {
        Some(experiment_enabled) => *experiment_enabled,
        None => return (FlagState::new(enabled, None), conditions),
    };

    let default_meta = FeatureRuleMeta::default();
    let experiment_key = &metas.get(index).unwrap_or(&default_meta).key;

    (
        FlagState::new(experiment_enabled, Some(experiment_key.clone())),
        conditions.map(|c| format!("{c}; experiment key={experiment_key}")),
    )
}

fn check_string(user_condition: &Vec<String>, value: &Value) -> bool {
    if let Ok(string) = value.convert_to_string() {
        user_condition.fold_to_string() == string
    } else {
        false
    }
}

fn check_gt(user_condition: &Vec<String>, gt: &Value) -> bool {
    check_number_condition(user_condition, gt, |user_value, rule_value| {
        user_value > rule_value
    })
}

fn check_gte(user_condition: &Vec<String>, gte: &Value) -> bool {
    check_number_condition(user_condition, gte, |user_value, rule_value| {
        user_value >= rule_value
    })
}

fn check_lt(user_condition: &Vec<String>, lt: &Value) -> bool {
    check_number_condition(user_condition, lt, |user_value, rule_value| {
        user_value < rule_value
    })
}

fn check_lte(user_condition: &Vec<String>, lte: &Value) -> bool {
    check_number_condition(user_condition, lte, |user_value, rule_value| {
        user_value <= rule_value
    })
}

fn check_number_condition(
    user_condition: &Vec<String>,
    value: &Value,
    condition: fn(user_value: usize, rule_value: usize) -> bool,
) -> bool {
    if let Ok(user_value) = user_condition.convert_to_usize() {
        if let Ok(rule_value) = value.convert_to_usize() {
            condition(user_value, rule_value)
        } else {
            false
        }
    } else {
        false
    }
}

fn check_elem_match_eq(user_condition: &[String], value: &Value) -> bool {
    if let Ok(rule_value) = value.convert_to_string() {
        user_condition
            .iter()
            .any(|user_value| user_value == &rule_value)
    } else {
        false
    }
}
