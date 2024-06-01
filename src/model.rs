use serde_json::Value;

pub enum Flag {
    BooleanFlag(BooleanFlag),
}

pub trait FlagCreator {
    fn create_flag(&self, experiment_key: Option<String>) -> Flag;
}

impl FlagCreator for Value {
    fn create_flag(&self, experiment_key: Option<String>) -> Flag {
        Flag::BooleanFlag(BooleanFlag::new(self, experiment_key))
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
