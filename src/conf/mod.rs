use config::{Config, Environment, File};
use serde::Deserialize;
use thiserror::Error;
use tracing::{debug, trace};

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
pub struct Settings {
    bla: Option<String>,
}

pub fn init_config(config_files: Vec<String>) -> Result<Settings, ConfigError> {
    trace!("Initializing app config");

    let mut s = Config::new();

    let _ = dotenv::dotenv();

    let env_var = std::env::var("CONFIG_PATHS");
    let mut config_files: Vec<&str> = match &env_var {
        Ok(val) => {
            debug!("Config paths are overridden with the env variable CONFIG_PATHS");
            val.split(',').map(|e| e.trim()).collect()
        }
        Err(_) => config_files.iter().map(AsRef::as_ref).collect(),
    };

    config_files.reverse();
    trace!("Loading config files {:#?}", &config_files);
    for config_file in config_files {
        s.merge(File::with_name(config_file))?;
    }
    debug!("Loading config from environment variables prefixed with APP");
    s.merge(Environment::with_prefix("APP"))?;

    let result = s.try_into::<Settings>().unwrap();
    trace!("Successfully loaded config");

    Ok(result)
}
