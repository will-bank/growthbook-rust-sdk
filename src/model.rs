use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug)]
pub struct FlagState {
    pub enabled: bool,
    pub experiment_key: Option<String>,
}

impl FlagState {
    pub fn new(enabled: bool, experiment_key: Option<String>) -> Self {
        FlagState {
            enabled,
            experiment_key,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrowthBookResponse {
    pub features: HashMap<String, Feature>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    pub default_value: bool,
    pub rules: Option<Vec<FeatureRule>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureRule {
    pub force: Option<bool>,
    pub condition: Option<HashMap<String, serde_json::Value>>,
    pub coverage: Option<f32>,
    pub hash_attribute: Option<String>,
    pub variations: Option<Vec<bool>>,
    weights: Option<Vec<f32>>,
    pub meta: Option<Vec<FeatureRuleMeta>>,
    pub phase: Option<String>,
}

impl FeatureRule {
    pub fn weights(&self) -> Option<Vec<f32>> {
        self.weights.clone().map(|w| {
            let mut acc = 0.0;
            w.iter()
                .map(|weight| {
                    acc += weight;
                    acc
                })
                .collect()
        })
    }

    pub fn conditions_to_string(&self) -> String {
        match &self.condition {
            None => String::new(),
            Some(map) => {
                let conditions = map
                    .clone()
                    .into_keys()
                    .fold(String::new(), |acc, s| format!("{acc},{s}"));
                format!("Conditions: [{conditions}]")
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureRuleMeta {
    pub key: String,
}

impl Default for FeatureRuleMeta {
    fn default() -> Self {
        FeatureRuleMeta {
            key: String::from("unknown"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::model::FeatureRule;

    #[test]
    fn weights_aggregation() -> Result<(), Box<dyn std::error::Error>> {
        let rule = build_feature_rule(Some(vec![0.3334, 0.3333, 0.3333]));
        let weights = rule.weights().unwrap_or_default();
        assert_eq!(weights.len(), 3);
        assert_eq!(weights[0], 0.3334);
        assert_eq!(weights[1], 0.6667);
        assert_eq!(weights[2], 1.0);
        Ok(())
    }

    fn build_feature_rule(weights: Option<Vec<f32>>) -> FeatureRule {
        FeatureRule {
            force: None,
            condition: None,
            coverage: None,
            hash_attribute: None,
            variations: None,
            weights,
            meta: None,
            phase: None,
        }
    }
}
