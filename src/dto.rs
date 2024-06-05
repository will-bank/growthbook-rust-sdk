use std::collections::HashMap;

use crate::model_public::{GrowthBookAttribute, GrowthBookAttributeValue};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GrowthBookResponse {
    pub features: HashMap<String, GrowthBookFeature>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GrowthBookFeature {
    pub default_value: Value,
    pub rules: Option<Vec<GrowthBookFeatureRule>>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GrowthBookFeatureRuleMeta {
    pub key: String,
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum GrowthBookFeatureRule {
    Experiment(GrowthBookFeatureRuleExperiment),
    Rollout(GrowthBookFeatureRuleRollout),
    Force(GrowthBookFeatureRuleForce), // needs to be the last one
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GrowthBookFeatureRuleForce {
    pub force: Value,
    condition: Option<HashMap<String, Value>>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GrowthBookFeatureRuleRollout {
    pub force: Value,
    pub coverage: f32,
    condition: Option<HashMap<String, Value>>,
    pub hash_attribute: String,
}

#[derive(Deserialize, Clone)]
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

pub struct GrowthBookFeatureRuleExperimentRange {
    pub start: f32,
    pub end: f32,
}

impl GrowthBookFeatureRuleRollout {
    pub fn conditions(&self) -> Option<Vec<GrowthBookAttribute>> {
        option_map_to_attributes(self.condition.clone())
    }
}

impl GrowthBookFeatureRuleForce {
    pub fn conditions(&self) -> Option<Vec<GrowthBookAttribute>> {
        option_map_to_attributes(self.condition.clone())
    }
}

impl GrowthBookFeatureRuleExperimentRange {
    fn new(start: f32, end: f32) -> Self {
        Self { start, end }
    }

    pub fn in_range(&self, value: &f32) -> bool {
        value >= &self.start && value <= &self.end
    }
}

impl GrowthBookFeatureRuleExperiment {
    pub fn seed(&self) -> String {
        self.seed
            .clone()
            .unwrap_or(self.key.clone().unwrap_or(String::from("default")))
    }

    pub fn weights(&self) -> Vec<GrowthBookFeatureRuleExperimentRange> {
        let clamped_coverage = self.clamped_coverage();
        let adjusted_weights = self.adjusted_weights();

        let mut acc = 0.0;
        adjusted_weights
            .iter()
            .map(|weight| {
                let start = acc;
                acc += weight;
                let end = start + clamped_coverage * weight;
                GrowthBookFeatureRuleExperimentRange::new(start, end)
            })
            .collect()
    }

    fn adjusted_weights(&self) -> Vec<f32> {
        let weights_sum: f32 = self.weights.iter().sum();
        if !(0.99..=1.01).contains(&weights_sum) {
            let weight = 1.0 / self.weights.len() as f32;
            let mut vec = vec![];
            for _ in 0..self.weights.len() {
                vec.push(weight);
            }
            vec
        } else {
            self.weights.clone()
        }
    }

    fn clamped_coverage(&self) -> f32 {
        match self.coverage {
            None => 1.0,
            Some(value) => {
                if value > 1.0 {
                    1.0
                } else if value < 0.0 {
                    0.0
                } else {
                    value
                }
            }
        }
    }
}

pub fn option_map_to_attributes(
    option_map: Option<HashMap<String, Value>>,
) -> Option<Vec<GrowthBookAttribute>> {
    if let Some(conditions) = option_map {
        Some(
            conditions
                .iter()
                .map(|(k, v)| {
                    GrowthBookAttribute::new(k.clone(), GrowthBookAttributeValue::from(v.clone()))
                })
                .collect(),
        )
    } else {
        None
    }
}
