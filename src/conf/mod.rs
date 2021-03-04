use config::{Config, Environment, File};
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("{0}")]
    ReadConfig(config::ConfigError),
}

impl From<config::ConfigError> for ConfigError {
    fn from(e: config::ConfigError) -> Self {
        ConfigError::ReadConfig(e)
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings;

pub fn init_config(config_files: Vec<String>) -> Result<Settings, ConfigError> {
    let mut s = Config::new();

    let _ = dotenv::dotenv();

    let env_var = std::env::var("CONFIG_PATHS");
    let mut config_files: Vec<&str> = match &env_var {
        Ok(val) => val.split(',').map(|e| e.trim()).collect(),
        Err(_) => config_files.iter().map(AsRef::as_ref).collect(),
    };

    config_files.reverse();
    for config_file in config_files {
        s.merge(File::with_name(config_file))?;
    }
    s.merge(Environment::with_prefix("APP"))?;

    let result = s.get::<Settings>("")?;
    Ok(result)
}
