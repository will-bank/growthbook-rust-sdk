use crate::extensions::{ConvertToUsize, FoldVecString};
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

pub trait ConditionEnabledCheck {
    fn is_on(&self, user_attributes: Option<&HashMap<String, Vec<String>>>) -> bool;
}

impl ConditionEnabledCheck for HashMap<String, Value> {
    fn is_on(&self, user_attributes: Option<&HashMap<String, Vec<String>>>) -> bool {
        if let Some(attributes) = user_attributes {
            evaluate(self, None, attributes)
        } else {
            false
        }
    }
}

fn evaluate(
    map: &HashMap<String, Value>,
    parent_key: Option<String>,
    attributes: &HashMap<String, Vec<String>>,
) -> bool {
    map.iter()
        .all(|(key, value)| is_on(parent_key.clone(), key, value, attributes))
}

fn is_on(
    parent_key: Option<String>,
    key: &str,
    value: &Value,
    user_attributes: &HashMap<String, Vec<String>>,
) -> bool {
    match key {
        "$not" => {
            let next_elem = value_struct_to_hash_map(value);
            !evaluate(
                &next_elem,
                Some(parent_key.unwrap_or(key.to_string())),
                user_attributes,
            )
        }
        "$gt" => evaluate_usize(
            parent_key,
            key,
            value,
            user_attributes,
            |feature_attribute, user_attribute| feature_attribute < user_attribute,
        ),
        "$gte" => evaluate_usize(
            parent_key,
            key,
            value,
            user_attributes,
            |feature_attribute, user_attribute| feature_attribute <= user_attribute,
        ),
        "$lt" => evaluate_usize(
            parent_key,
            key,
            value,
            user_attributes,
            |feature_attribute, user_attribute| feature_attribute > user_attribute,
        ),
        "$lte" => evaluate_usize(
            parent_key,
            key,
            value,
            user_attributes,
            |feature_attribute, user_attribute| feature_attribute >= user_attribute,
        ),
        "$eq" => evaluate_string(
            parent_key,
            key,
            value,
            user_attributes,
            |feature_attribute, user_attribute| feature_attribute == user_attribute,
        ),
        "$regex" => evaluate_string(
            parent_key,
            key,
            value,
            user_attributes,
            |feature_attribute, user_attribute| {
                if let Ok(regex) = Regex::new(feature_attribute) {
                    regex.is_match(user_attribute)
                } else {
                    false
                }
            },
        ),
        "$elemMatch" => {
            let next_elem = value_struct_to_hash_map(value);
            let attribute_key = parent_key.clone().unwrap_or(key.to_string());
            if let Some(user_attribute_values) = user_attributes.get(&attribute_key.clone()) {
                user_attribute_values.iter().any(|value| {
                    let user_attribute_elem =
                        HashMap::from([(attribute_key.clone(), vec![value.clone()])]);
                    evaluate(
                        &next_elem,
                        Some(attribute_key.clone()),
                        &user_attribute_elem,
                    )
                })
            } else {
                false
            }
        }
        &_ => {
            if value.is_object() {
                let next_elem = value_struct_to_hash_map(value);
                evaluate(
                    &next_elem,
                    Some(parent_key.unwrap_or(key.to_string())),
                    user_attributes,
                )
            } else {
                evaluate_string(
                    parent_key,
                    key,
                    value,
                    user_attributes,
                    |feature_attribute, user_attribute| feature_attribute == user_attribute,
                )
            }
        }
    }
}

fn evaluate_usize(
    parent_key: Option<String>,
    key: &str,
    value: &Value,
    user_attributes: &HashMap<String, Vec<String>>,
    evaluate: fn(usize, usize) -> bool,
) -> bool {
    let attribute_key = &parent_key.clone().unwrap_or(String::from(key));
    if let Some(attribute) = user_attributes.get(attribute_key) {
        if let Ok(feature_attribute) = value.convert_to_usize() {
            if let Ok(user_attribute) = attribute.fold_to_string().convert_to_usize() {
                evaluate(feature_attribute, user_attribute)
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

fn evaluate_string(
    parent_key: Option<String>,
    key: &str,
    value: &Value,
    user_attributes: &HashMap<String, Vec<String>>,
    evaluate: fn(&str, &str) -> bool,
) -> bool {
    let attribute_key = &parent_key.unwrap_or(String::from(key));
    if let Some(attribute) = user_attributes.get(attribute_key) {
        if let Some(feature_attribute) = value.as_str() {
            evaluate(feature_attribute, &attribute.fold_to_string())
        } else {
            false
        }
    } else {
        false
    }
}

fn value_struct_to_hash_map(value: &Value) -> HashMap<String, Value> {
    let map = value.as_object().expect("");
    let mut hash_map = HashMap::new();
    for (key, value) in map {
        hash_map.insert(key.clone(), value.clone());
    }
    hash_map
}
