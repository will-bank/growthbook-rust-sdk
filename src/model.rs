use serde_json::Value;

pub enum Flag {
    BooleanFlag(BooleanFlag),
    StringFlag(StringFlag),
    InvalidFlag(),
}

pub trait FlagCreator {
    fn create_flag(&self, experiment_key: Option<String>) -> Flag;
}

impl FlagCreator for Value {
    fn create_flag(&self, experiment_key: Option<String>) -> Flag {
        if self.is_boolean() {
            Flag::BooleanFlag(BooleanFlag::new(self, experiment_key))
        } else if self.is_string() {
            Flag::StringFlag(StringFlag::new(self, experiment_key))
        } else {
            Flag::InvalidFlag()
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
