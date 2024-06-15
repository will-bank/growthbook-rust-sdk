use serde_json::Value;

use crate::dto::GrowthBookFeatureRuleExperiment;
use crate::extensions::FindGrowthBookAttribute;
use crate::hash::{HashCode, HashCodeVersion};
use crate::model_public::GrowthBookAttribute;
use crate::range::model::Range;

impl GrowthBookFeatureRuleExperiment {
    pub fn get_match_value(
        &self,
        _feature_name: &str,
        option_user_attributes: Option<&Vec<GrowthBookAttribute>>,
    ) -> Option<(Value, String)> {
        if let Some(user_attributes) = option_user_attributes {
            if let Some(user_value) = user_attributes.find_value(&self.hash_attribute) {
                let user_weight = HashCode::hash_code(&user_value.to_string(), &self.seed(), HashCodeVersion::from(self.hash_version)).unwrap_or(-1.0);
                let ranges = self.ranges();
                let index = choose_variation(user_weight, ranges);
                if index >= 0 {
                    let usize_index = index as usize;
                    return Some((self.variations[usize_index].clone(), self.meta[usize_index].key.clone()));
                }
            }
        }

        None
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
