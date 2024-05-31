use crate::error::GrowthbookError;
use std::env;

pub struct Environment;
impl Environment {
    pub fn string(env_name: &str) -> Result<String, GrowthbookError> {
        env::var(env_name).map_err(GrowthbookError::from)
    }

    pub fn u64_or_default(env_name: &str, default: u64) -> u64 {
        env::var(env_name)
            .ok()
            .map(|env| env.parse::<u64>().unwrap_or(default))
            .unwrap_or(default)
    }
}
