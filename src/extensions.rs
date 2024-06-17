use serde_json::Value;

use crate::model_public::{GrowthBookAttribute, GrowthBookAttributeValue};

pub trait FindGrowthBookAttribute {
    fn find_value(
        &self,
        attribute_key: &str,
    ) -> Option<GrowthBookAttributeValue>;
}

pub trait JsonHelper {
    fn get_value(
        &self,
        name: &str,
    ) -> Value;
    fn get_bool(
        &self,
        name: &str,
    ) -> bool;
    fn get_string(
        &self,
        name: &str,
    ) -> String;
    fn get_array(
        &self,
        name: &str,
    ) -> Vec<Value>;
    fn force_string(&self) -> String;
    fn force_i64(&self) -> i64;
    fn force_f32(&self) -> f32;
    fn force_f64(&self) -> f64;
    fn force_array(&self) -> Vec<Value>;
}

impl JsonHelper for Value {
    fn get_value(
        &self,
        name: &str,
    ) -> Value {
        self.get(name).expect(format!("Failed to get {name}").as_str()).clone()
    }

    fn get_bool(
        &self,
        name: &str,
    ) -> bool {
        self.get_value(name).as_bool().expect("Failed to convert to bool")
    }

    fn get_string(
        &self,
        name: &str,
    ) -> String {
        self.get_value(name).force_string()
    }

    fn get_array(
        &self,
        name: &str,
    ) -> Vec<Value> {
        self.get(name).expect(format!("Failed to get {name}").as_str()).force_array()
    }

    fn force_string(&self) -> String { self.as_str().expect("Failed to convert to str").to_string() }

    fn force_i64(&self) -> i64 { self.as_i64().expect("Failed to convert to i64") }

    fn force_f32(&self) -> f32 { self.force_f64() as f32 }

    fn force_f64(&self) -> f64 { self.as_f64().expect("Failed to convert to f64") }

    fn force_array(&self) -> Vec<Value> { self.as_array().expect("Failed to convert to array").clone() }
}

impl FindGrowthBookAttribute for Vec<GrowthBookAttribute> {
    fn find_value(
        &self,
        attribute_key: &str,
    ) -> Option<GrowthBookAttributeValue> {
        look_for_attribute(0, attribute_key, self).map(|it| it.value)
    }
}

impl FindGrowthBookAttribute for &[GrowthBookAttribute] {
    fn find_value(
        &self,
        attribute_key: &str,
    ) -> Option<GrowthBookAttributeValue> {
        look_for_attribute(0, attribute_key, self).map(|it| it.value)
    }
}

fn look_for_attribute(
    split_index: usize,
    attribute_key: &str,
    user_attributes: &[GrowthBookAttribute],
) -> Option<GrowthBookAttribute> {
    let split = attribute_key.split('.').collect::<Vec<&str>>();
    let key_part = split[split_index];
    let option_attribute = user_attributes.iter().find(|item| item.key == key_part);
    if let Some(found_attribute) = option_attribute {
        if split.len().gt(&(split_index + 1)) {
            match found_attribute.value.clone() {
                GrowthBookAttributeValue::Object(it) => look_for_attribute(split_index + 1, attribute_key, &it),
                GrowthBookAttributeValue::Empty => None,
                _ => Some(found_attribute.clone()),
            }
        } else {
            Some(found_attribute.clone())
        }
    } else {
        None
    }
}
