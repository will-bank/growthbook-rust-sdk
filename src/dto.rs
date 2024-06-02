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

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeatureRuleExperiment {
    pub coverage: f32,
    pub hash_attribute: String,
    pub variations: Vec<Value>,
    weights: Vec<f32>,
    pub meta: Vec<FeatureRuleMeta>,
}

impl FeatureRuleExperiment {
    pub fn weights(&self) -> Vec<f32> {
        let mut acc = 0.0;
        self.weights
            .iter()
            .map(|weight| {
                acc += weight;
                acc
            })
            .collect()
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
