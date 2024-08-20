use crate::config::error::{Error, Result};
use std::env;
use std::sync::OnceLock;

pub fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::MissingEnv(name))
}

pub fn core_config() -> &'static CoreConfig {
    static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        CoreConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct CoreConfig {
    pub API_KEY: String,
}

impl CoreConfig {
    fn load_from_env() -> Result<CoreConfig> {
        Ok(CoreConfig {
            API_KEY: get_env("API_KEY")?,
        })
    }
}
