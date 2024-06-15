use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

use crate::model_public::{GrowthBookAttribute, GrowthBookAttributeValue};
use crate::range::model::Range;

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GrowthBookResponse {
    pub features: HashMap<String, GrowthBookFeature>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GrowthBookFeature {
    pub default_value: Value,
    pub rules: Option<Vec<GrowthBookFeatureRule>>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GrowthBookFeatureRuleMeta {
    pub key: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum GrowthBookFeatureRule {
    Experiment(GrowthBookFeatureRuleExperiment),
    Rollout(GrowthBookFeatureRuleRollout),
    Force(GrowthBookFeatureRuleForce), // needs to be the last one
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GrowthBookFeatureRuleForce {
    pub force: Value,
    condition: Option<HashMap<String, Value>>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GrowthBookFeatureRuleRollout {
    pub force: Value,
    pub coverage: f32,
    condition: Option<HashMap<String, Value>>,
    pub hash_attribute: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GrowthBookFeatureRuleExperiment {
    key: Option<String>,
    pub coverage: Option<f32>,
    seed: Option<String>,
    pub hash_version: Option<i64>,
    pub hash_attribute: String,
    pub variations: Vec<Value>,
    weights: Vec<f32>,
    pub meta: Vec<GrowthBookFeatureRuleMeta>,
}

impl GrowthBookFeatureRuleRollout {
    pub fn conditions(&self) -> Option<Vec<GrowthBookAttribute>> { option_map_to_attributes(self.condition.clone()) }
}

impl GrowthBookFeatureRuleForce {
    pub fn conditions(&self) -> Option<Vec<GrowthBookAttribute>> { option_map_to_attributes(self.condition.clone()) }
}

impl GrowthBookFeatureRuleExperiment {
    pub fn seed(&self) -> String { self.seed.clone().unwrap_or(self.key.clone().unwrap_or(String::from("default"))) }

    pub fn weights(&self) -> Vec<Range> { Range::get_bucket_range(self.weights.len() as i64, &self.coverage, Some(self.weights.clone())) }
}

pub fn option_map_to_attributes(option_map: Option<HashMap<String, Value>>) -> Option<Vec<GrowthBookAttribute>> {
    option_map.map(|conditions| conditions.iter().map(|(k, v)| GrowthBookAttribute::new(k.clone(), GrowthBookAttributeValue::from(v.clone()))).collect())
}
