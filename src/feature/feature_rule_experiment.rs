use std::collections::HashMap;

use serde_json::Value;

use crate::dto::GrowthBookFeatureRuleExperiment;
use crate::extensions::FindGrowthBookAttribute;
use crate::hash::{HashCode, HashCodeVersion};
use crate::model_private::{ExperimentResult, Feature};
use crate::model_public::GrowthBookAttribute;
use crate::range::model::Range;

impl GrowthBookFeatureRuleExperiment {
    pub fn get_match_value(
        &self,
        feature_name: &str,
        option_user_attributes: &Option<Vec<GrowthBookAttribute>>,
        forced_variations: &Option<HashMap<String, i64>>,
    ) -> Option<Feature> {
        if let Some(user_attributes) = option_user_attributes {
            if let Some(feature_attribute) = &self.hash_attribute {
                if let Some(user_value) = user_attributes.find_value(feature_attribute) {
                    if let Some((namespace, range)) = &self.namespace_range() {
                        let user_weight = HashCode::hash_code(&format!("{}__", user_value.to_string()), namespace, HashCodeVersion::from(1)).unwrap_or(-1.0);
                        if !range.in_range(&user_weight) {
                            return None; // TODO: ???
                        }
                    }

                    if let Some(forced_variation) = self.forced_variation(feature_name, user_attributes, forced_variations) {
                        return forced_variation;
                    }

                    let user_weight = HashCode::hash_code(&user_value.to_string(), &self.seed(feature_name), HashCodeVersion::from(self.hash_version)).unwrap_or(-1.0);
                    let ranges = self.ranges();
                    let index = choose_variation(user_weight, ranges);
                    if index >= 0 {
                        let usize_index = index as usize;
                        let value = self.variations[usize_index].clone();
                        let (meta_value, pass_through) = self.get_meta_value(usize_index);
                        if !pass_through {
                            return Some(Feature::experiment(
                                value.clone(),
                                self.model_experiment(),
                                create_experiment_result(
                                    feature_name,
                                    value.clone(),
                                    index,
                                    true,
                                    Some(feature_attribute.clone()),
                                    Some(user_value.to_value()),
                                    Some(user_weight),
                                    meta_value,
                                ),
                            ));
                        }
                    }
                }
            } else {
                let fallback_attribute = self.get_fallback_attribute();
                if let Some(user_value) = user_attributes.find_value(&fallback_attribute) {
                    if let Some((namespace, range)) = &self.namespace_range() {
                        let user_weight = HashCode::hash_code(&format!("{}__", user_value.to_string()), namespace, HashCodeVersion::from(1)).unwrap_or(-1.0);
                        if !range.in_range(&user_weight) {
                            return None; // TODO: ???
                        }
                    }

                    if let Some(forced_variation) = self.forced_variation(feature_name, user_attributes, forced_variations) {
                        return forced_variation;
                    }

                    let user_weight = HashCode::hash_code(&user_value.to_string(), &self.seed(feature_name), HashCodeVersion::from(self.hash_version)).unwrap_or(-1.0);
                    let ranges = self.ranges();
                    let index = choose_variation(user_weight, ranges);
                    if index >= 0 {
                        let usize_index = index as usize;
                        let value = self.variations[usize_index].clone();
                        let (meta_value, pass_through) = self.get_meta_value(usize_index);
                        if !pass_through {
                            return Some(Feature::experiment(
                                value.clone(),
                                self.model_experiment(),
                                create_experiment_result(
                                    feature_name,
                                    value.clone(),
                                    index,
                                    true,
                                    Some(fallback_attribute.clone()),
                                    Some(user_value.to_value()),
                                    Some(user_weight),
                                    meta_value,
                                ),
                            ));
                        }
                    }
                } else {
                    // TODO:
                }
            }
        }

        None
    }

    fn forced_variation(
        &self,
        feature_name: &str,
        user_attributes: &Vec<GrowthBookAttribute>,
        forced_variations: &Option<HashMap<String, i64>>,
    ) -> Option<Option<Feature>> {
        if let Some(forced_variations) = forced_variations {
            if let Some(found_forced_variation) = forced_variations.get(feature_name) {
                let hash_attribute = self.hash_attribute.clone().unwrap_or(self.get_fallback_attribute());
                if let Some(user_value) = user_attributes.find_value(&hash_attribute) {
                    let forced_variation_index = found_forced_variation.clone() as usize;
                    let value = self.variations[forced_variation_index].clone();
                    let (meta_value, pass_through) = self.get_meta_value(forced_variation_index);
                    return Some(Some(Feature::experiment(
                        value.clone(),
                        self.model_experiment(),
                        create_experiment_result(
                            feature_name,
                            value.clone(),
                            found_forced_variation.clone(),
                            true,
                            self.hash_attribute.clone(),
                            Some(user_value.to_value()),
                            None,
                            meta_value,
                        ),
                    )));
                }
            }
        }
        None
    }

    // TODO: need refactory
    fn get_meta_value(
        &self,
        usize_index: usize,
    ) -> (String, bool) {
        self.meta
            .clone()
            .map(|it| {
                if it.is_array() {
                    if let Some(meta_value) = it.as_array().expect("Failed to convert to array").get(usize_index) {
                        let pass_through = if let Some(pass_through_value) = meta_value.get("passthrough") {
                            pass_through_value.as_bool().expect("Failed to convert to bool")
                        } else {
                            false
                        };

                        if let Some(key) = meta_value.get("key") {
                            if key.is_string() {
                                (key.as_str().expect("Failed to convert to str").to_string(), pass_through)
                            } else {
                                (key.to_string(), pass_through)
                            }
                        } else {
                            (format!("{usize_index}"), pass_through)
                        }
                    } else {
                        (format!("{usize_index}"), false)
                    }
                } else {
                    (format!("{usize_index}"), false)
                }
            })
            .unwrap_or((format!("{usize_index}"), false))
    }

    fn get_fallback_attribute(&self) -> String { self.fallback_attribute.clone().unwrap_or(String::from("id")) }
}

fn create_experiment_result(
    feature_name: &str,
    value: Value,
    variation_id: i64,
    hash_used: bool,
    hash_attribute: Option<String>,
    hash_value: Option<Value>,
    bucket: Option<f32>,
    key: String,
) -> ExperimentResult {
    ExperimentResult {
        feature_id: String::from(feature_name),
        value,
        variation_id,
        in_experiment: true,
        hash_used,
        hash_attribute,
        hash_value,
        bucket,
        key,
        sticky_bucket_used: false,
    }
}

fn choose_variation(
    user_weight: f32,
    ranges: Vec<Range>,
) -> i64 {
    for (index, range) in ranges.iter().enumerate() {
        if range.in_range(&user_weight) {
            return index as i64;
        }
    }
    -1
}

#[cfg(test)]
mod test {
    use std::fs;

    use serde::Deserialize;
    use serde_json::Value;

    use crate::feature::feature_rule_experiment::choose_variation;
    use crate::range::model::Range;

    #[tokio::test]
    async fn evaluate_choose_variation() -> Result<(), Box<dyn std::error::Error>> {
        let cases = Cases::new();

        for value in cases.choose_variation {
            let eval_choose_variation = EvalChooseVariation::new(value);
            let index = choose_variation(eval_choose_variation.weight, eval_choose_variation.ranges);
            if eval_choose_variation.index != index {
                panic!(
                    "EvalChooseVariation failed; name='{}' expected_index={} index={index}",
                    eval_choose_variation.name, eval_choose_variation.index
                )
            }
        }

        Ok(())
    }

    #[derive(Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    struct Cases {
        choose_variation: Vec<Value>,
    }

    pub struct EvalChooseVariation {
        name: String,
        weight: f32,
        ranges: Vec<Range>,
        index: i64,
    }

    impl EvalChooseVariation {
        fn new(value: Value) -> Self {
            let array = value.as_array().expect("Failed to convert to array");
            Self {
                name: array[0].as_str().expect("Failed to convert to str").to_string(),
                weight: array[1].as_f64().expect("Failed to convert to f64") as f32,
                ranges: array[2]
                    .as_array()
                    .expect("Failed to convert to array")
                    .iter()
                    .map(|it| {
                        let array = it.as_array().expect("Failed to convert to array [2]");
                        Range {
                            start: array[0].as_f64().expect("Failed to convert to f64") as f32,
                            end: array[1].as_f64().expect("Failed to convert to f64") as f32,
                        }
                    })
                    .collect(),
                index: array[3].as_i64().expect("Failed to convert to i64"),
            }
        }
    }

    impl Cases {
        pub fn new() -> Self {
            let contents = fs::read_to_string("./tests/all_cases.json").expect("Should have been able to read the file");

            serde_json::from_str(&contents).expect("Failed to create cases")
        }
    }
}
