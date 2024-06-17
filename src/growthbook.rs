use std::collections::HashMap;

use crate::dto::GrowthBookFeature;
use crate::model_private::FeatureResult;
use crate::model_public::GrowthBookAttribute;

#[derive(Clone)]
pub struct GrowthBook {
    pub forced_variations: Option<HashMap<String, i64>>,
    pub features: HashMap<String, GrowthBookFeature>,
}

impl GrowthBook {
    pub fn check(
        &self,
        flag_name: &str,
        option_user_attributes: &Option<Vec<GrowthBookAttribute>>,
    ) -> FeatureResult {
        if let Some(feature) = self.features.get(flag_name) {
            let user_attributes = &option_user_attributes.clone().unwrap_or_default();
            feature.get_value(flag_name, vec![], user_attributes, &self.forced_variations, self.features.clone())
        } else {
            FeatureResult::unknown_feature()
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::fs;

    use serde::Deserialize;
    use serde_json::Value;

    use crate::dto::GrowthBookFeature;
    use crate::extensions::JsonHelper;
    use crate::growthbook::GrowthBook;
    use crate::model_private::FeatureResult;
    use crate::model_public::GrowthBookAttribute;

    #[tokio::test]
    async fn evaluate_get_bucket_range() -> Result<(), Box<dyn std::error::Error>> {
        let cases = Cases::new();

        for value in cases.feature {
            let feature = EvalFeature::new(value);
            let gb_test = serde_json::from_value::<GrowthBookForTest>(feature.feature.clone()).unwrap_or_else(|_| panic!("Failed to convert to GrowthBookForTest case='{}'", feature.name));
            let gb = GrowthBook {
                forced_variations: feature.forced_variations.clone(),
                features: gb_test.features.unwrap_or_default(),
            };
            let user_attributes = feature
                .attributes
                .clone()
                .map(|attr| GrowthBookAttribute::from(attr).expect("Failed to convert to GrowthBookAttribute"));
            let result = gb.check(feature.feature_name.as_str(), &user_attributes);
            validate_result(feature, result);
        }

        Ok(())
    }

    fn validate_result(
        eval_feature: EvalFeature,
        feature_result: FeatureResult,
    ) {
        let case_name = eval_feature.name;
        let expected_result = eval_feature.result;

        assert_eq!(expected_result.get_value("value", Value::Null), feature_result.value, "Invalid value for '{case_name}'");
        assert_eq!(expected_result.get_bool("on", false), feature_result.on, "Invalid on for '{case_name}'");
        assert_eq!(expected_result.get_bool("off", false), feature_result.off, "Invalid off for '{case_name}'");
        assert_eq!(expected_result.get_string("source", ""), feature_result.source, "Invalid source for '{case_name}'");
    }

    #[derive(Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    struct Cases {
        feature: Vec<Value>,
    }

    #[derive(Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct GrowthBookForTest {
        pub features: Option<HashMap<String, GrowthBookFeature>>,
    }

    #[derive(Clone)]
    pub struct EvalFeature {
        name: String,
        attributes: Option<Value>,
        forced_variations: Option<HashMap<String, i64>>,
        feature: Value,
        feature_name: String,
        result: Value,
    }

    impl EvalFeature {
        fn new(value: Value) -> Self {
            let array = value.as_array().expect("Failed to convert to array");
            let attr = array[1].as_object().expect("Failed to convert to object").get("attributes").unwrap_or(&Value::Null).clone();
            let attributes = if attr.is_null() { None } else { Some(attr) };
            let forced = array[1].as_object().expect("Failed to convert to object").get("forcedVariations").unwrap_or(&Value::Null).clone();
            let forced_variations = forced.as_object().map(|forced_entries| {
                let mut map = HashMap::new();
                for (key, value) in forced_entries {
                    map.insert(key.clone(), value.as_i64().expect("Failed to convert to i64"));
                }
                map
            });
            Self {
                name: array[0].as_str().expect("Failed to convert to str").to_string(),
                attributes,
                forced_variations,
                feature: array[1].clone(),
                feature_name: array[2].as_str().expect("Failed to convert to str").to_string(),
                result: array[3].clone(),
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
