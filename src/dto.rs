use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GrowthBookResponse {
    pub features: HashMap<String, Feature>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    pub default_value: Value,
    pub rules: Option<Vec<FeatureRule>>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeatureRuleMeta {
    pub key: String,
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum FeatureRule {
    Experiment(FeatureRuleExperiment),
    Rollout(FeatureRuleRollout),
    Force(FeatureRuleForce), // needs to be the last one
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum FeatureRuleCondition {
    String(String),
    Gte(FeatureRuleConditionGte),
    Gt(FeatureRuleConditionGt),
    Lte(FeatureRuleConditionLte),
    Lt(FeatureRuleConditionLt),
    ElemMatchEq(FeatureRuleConditionElemMatchEq),
    NotElemMatchEq(FeatureRuleConditionNotElemMatchEq),
    Eq(FeatureRuleConditionEq),
    In(FeatureRuleConditionIn),
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeatureRuleForce {
    pub force: Value,
    pub condition: Option<HashMap<String, FeatureRuleCondition>>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeatureRuleRollout {
    pub force: Value,
    pub coverage: f32,
    pub condition: Option<HashMap<String, FeatureRuleCondition>>,
    pub hash_attribute: String,
}

pub struct FeatureRuleExperimentRange {
    pub start: f32,
    pub end: f32,
}

impl FeatureRuleExperimentRange {
    fn new(start: f32, end: f32) -> Self {
        Self { start, end }
    }

    pub fn in_range(&self, value: &f32) -> bool {
        value >= &self.start && value <= &self.end
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeatureRuleExperiment {
    key: Option<String>,
    pub coverage: Option<f32>,
    seed: Option<String>,
    pub hash_version: Option<i64>,
    pub hash_attribute: String,
    pub variations: Vec<Value>,
    weights: Vec<f32>,
    pub meta: Vec<FeatureRuleMeta>,
}

impl FeatureRuleExperiment {
    pub fn seed(&self) -> String {
        self.seed
            .clone()
            .unwrap_or(self.key.clone().unwrap_or(String::from("default")))
    }

    pub fn weights(&self) -> Vec<FeatureRuleExperimentRange> {
        let clamped_coverage = self.clamped_coverage();
        let adjusted_weights = self.adjusted_weights();

        let mut acc = 0.0;
        adjusted_weights
            .iter()
            .map(|weight| {
                let start = acc;
                acc += weight;
                let end = start + clamped_coverage * weight;
                FeatureRuleExperimentRange::new(start, end)
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

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeatureRuleConditionGte {
    #[serde(alias = "$gte")]
    pub gte: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeatureRuleConditionGt {
    #[serde(alias = "$gt")]
    pub gt: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeatureRuleConditionLte {
    #[serde(alias = "$lte")]
    pub lte: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeatureRuleConditionLt {
    #[serde(alias = "$lt")]
    pub lt: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeatureRuleConditionElemMatchEq {
    #[serde(alias = "$elemMatch")]
    pub elem_match: FeatureRuleConditionEq,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeatureRuleConditionNotElemMatchEq {
    #[serde(alias = "$not")]
    pub not: FeatureRuleConditionElemMatchEq,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeatureRuleConditionEq {
    #[serde(alias = "$eq")]
    pub eq: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeatureRuleConditionIn {
    #[serde(alias = "$in")]
    pub array: Vec<String>,
}
