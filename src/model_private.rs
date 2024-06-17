use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::GrowthbookError;
use crate::extensions::JsonHelper;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureResult {
    pub value: Value,
    pub on: bool,
    pub off: bool,
    pub experiment: Option<Experiment>,
    pub experiment_result: Option<ExperimentResult>,
    pub source: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Experiment {
    pub name: Option<String>,
    pub seed: Option<String>,
    pub hash_version: Option<i64>,
    pub hash_attribute: Option<String>,
    pub namespace: Option<Vec<Value>>,
    pub coverage: Option<f32>,
    pub ranges: Option<Vec<Vec<f32>>>,
    pub meta: Option<Value>,
    pub filters: Option<Value>,
    pub variations: Vec<Value>,
    pub weights: Option<Vec<f32>>,
    pub condition: Option<Value>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExperimentResult {
    pub feature_id: String,
    pub value: Value,
    pub variation_id: i64,
    pub in_experiment: bool,
    pub hash_used: bool,
    pub hash_attribute: Option<String>,
    pub hash_value: Option<Value>,
    pub bucket: Option<f32>,
    pub key: String,
    pub sticky_bucket_used: bool,
}

impl FeatureResult {
    pub fn value_as<T>(&self) -> Result<T, GrowthbookError>
    where
        for<'a> T: Deserialize<'a>,
    {
        serde_json::from_value(self.value.clone()).map_err(GrowthbookError::from)
    }

    pub fn force(value: Value) -> Self {
        let is_on = is_on(&value);
        FeatureResult {
            value,
            on: is_on,
            off: !is_on,
            experiment: None,
            experiment_result: None,
            source: String::from("force"),
        }
    }
    pub fn experiment(
        value: Value,
        experiment: Experiment,
        experiment_result: ExperimentResult,
    ) -> Self {
        let is_on = is_on(&value);
        FeatureResult {
            value,
            on: is_on,
            off: !is_on,
            experiment: Some(experiment),
            experiment_result: Some(experiment_result),
            source: String::from("experiment"),
        }
    }

    pub fn from_default_value(option_value: Option<Value>) -> Self {
        let value = option_value.unwrap_or(Value::Null);
        let is_on = is_on(&value);
        Self {
            value,
            on: is_on,
            off: !is_on,
            experiment: None,
            experiment_result: None,
            source: String::from("defaultValue"),
        }
    }

    pub fn prerequisite() -> Self {
        Self {
            value: Value::Null,
            on: false,
            off: true,
            experiment: None,
            experiment_result: None,
            source: String::from("prerequisite"),
        }
    }

    pub fn cyclic_prerequisite() -> Self {
        Self {
            value: Value::Null,
            on: false,
            off: true,
            experiment: None,
            experiment_result: None,
            source: String::from("cyclicPrerequisite"),
        }
    }

    pub fn unknown_feature() -> Self {
        Self {
            value: Value::Null,
            on: false,
            off: true,
            experiment: None,
            experiment_result: None,
            source: String::from("unknownFeature"),
        }
    }
}

fn is_on(value: &Value) -> bool {
    let is_on = if value.is_null() {
        false
    } else if (value.is_number() && value.force_f64(-1.0) != 0.0) || (value.is_string() && value.force_string("any") != "") {
        true
    } else if value.is_boolean() {
        value.as_bool().unwrap_or(false)
    } else if value.is_object() {
        value.as_object().map(|it| !it.is_empty()).unwrap_or(false)
    } else if value.is_array() {
        value.as_array().map(|it| !it.is_empty()).unwrap_or(false)
    } else {
        false
    };
    is_on
}
