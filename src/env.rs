use std::env;

pub struct Environment;

impl Environment {
    pub fn string_or_default(env_name: &str, default: &str) -> String {
        env::var(env_name).unwrap_or(String::from(default))
    }
}
