use serde::Deserialize;
use serde_json::Value;

use crate::error::GrowthbookError;

pub enum Feature {
    Boolean(BooleanFeature),
    String(StringFeature),
    Object(ObjectFeature),
    Invalid(),
}

pub trait FeatureCreator {
    fn create(&self, experiment_key: Option<String>) -> Feature;
}

pub struct BooleanFeature {
    pub enabled: bool,
    pub experiment_key: Option<String>,
}

pub struct StringFeature {
    pub value: String,
    pub experiment_key: Option<String>,
}

pub struct ObjectFeature {
    value: Value,
    pub experiment_key: Option<String>,
}

impl FeatureCreator for Value {
    fn create(&self, experiment_key: Option<String>) -> Feature {
        if self.is_boolean() {
            Feature::Boolean(BooleanFeature::new(self, experiment_key))
        } else if self.is_string() {
            Feature::String(StringFeature::new(self, experiment_key))
        } else if self.is_object() {
            Feature::Object(ObjectFeature::new(self, experiment_key))
        } else {
            Feature::Invalid()
        }
    }
}

impl BooleanFeature {
    fn new(value: &Value, experiment_key: Option<String>) -> Self {
        Self {
            enabled: value.as_bool().unwrap_or_default(),
            experiment_key,
        }
    }
}

impl StringFeature {
    fn new(value: &Value, experiment_key: Option<String>) -> Self {
        Self {
            value: String::from(value.as_str().unwrap_or_default()),
            experiment_key,
        }
    }
}

impl ObjectFeature {
    fn new(value: &Value, experiment_key: Option<String>) -> Self {
        Self {
            value: value.clone(),
            experiment_key,
        }
    }

    pub fn value<T>(&self) -> Result<T, GrowthbookError>
    where
        for<'a> T: Deserialize<'a>,
    {
        serde_json::from_value(self.value.clone()).map_err(GrowthbookError::from)
    }
}
