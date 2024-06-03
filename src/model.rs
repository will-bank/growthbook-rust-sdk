use serde::Deserialize;
use serde_json::Value;

use crate::error::GrowthbookError;

pub enum Flag {
    Boolean(BooleanFlag),
    String(StringFlag),
    Object(ObjectFlag),
    Invalid(),
}

pub trait FlagCreator {
    fn create_flag(&self, experiment_key: Option<String>) -> Flag;
}

impl FlagCreator for Value {
    fn create_flag(&self, experiment_key: Option<String>) -> Flag {
        if self.is_boolean() {
            Flag::Boolean(BooleanFlag::new(self, experiment_key))
        } else if self.is_string() {
            Flag::String(StringFlag::new(self, experiment_key))
        } else if self.is_object() {
            Flag::Object(ObjectFlag::new(self, experiment_key))
        } else {
            Flag::Invalid()
        }
    }
}

pub struct BooleanFlag {
    pub enabled: bool,
    pub experiment_key: Option<String>,
}

impl BooleanFlag {
    fn new(value: &Value, experiment_key: Option<String>) -> Self {
        Self {
            enabled: value.as_bool().unwrap_or_default(),
            experiment_key,
        }
    }
}

pub struct StringFlag {
    pub value: String,
    pub experiment_key: Option<String>,
}

impl StringFlag {
    fn new(value: &Value, experiment_key: Option<String>) -> Self {
        Self {
            value: String::from(value.as_str().unwrap_or_default()),
            experiment_key,
        }
    }
}

pub struct ObjectFlag {
    value: Value,
    pub experiment_key: Option<String>,
}

impl ObjectFlag {
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
