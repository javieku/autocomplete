use std::str::FromStr;

use config::{ConfigError, Environment};
use serde::Deserialize;

use crate::configuration::elasticsearch::ElasticsearchConfig;
use crate::configuration::server::ServerConfig;
use crate::utils::dir::get_project_root;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub profile: Profile,
    pub server: ServerConfig,
    pub elasticsearch: ElasticsearchConfig,
}

impl AppConfig {
    pub fn read(env_src: Environment) -> Result<Self, ConfigError> {
        let config_dir = get_settings_dir()?;
        let profile = std::env::var("APP_PROFILE")
            .map(|env| Profile::from_str(&env).map_err(|e| ConfigError::Message(e.to_string())))
            .unwrap_or_else(|_e| Ok(Profile::Dev))?;
        let profile_filename = format!("{profile}.toml");
        let config = config::Config::builder()
            .add_source(config::File::from(config_dir.join("base.toml")))
            .add_source(config::File::from(config_dir.join(profile_filename)))
            .add_source(env_src)
            .build()?;
        println!("Successfully read config profile: {profile}.");
        config.try_deserialize()
    }
}

pub fn get_settings_dir() -> Result<std::path::PathBuf, ConfigError> {
    Ok(get_project_root()
        .map_err(|e| ConfigError::Message(e.to_string()))?
        .join("settings"))
}

pub fn get_static_dir() -> Result<std::path::PathBuf, ConfigError> {
    Ok(get_project_root()
        .map_err(|e| ConfigError::Message(e.to_string()))?
        .join("static"))
}

#[derive(
    Debug,
    strum_macros::Display,
    strum_macros::EnumString,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
)]
pub enum Profile {
    #[serde(rename = "test")]
    #[strum(serialize = "test")]
    Test,
    #[serde(rename = "dev")]
    #[strum(serialize = "dev")]
    Dev,
    #[serde(rename = "prod")]
    #[strum(serialize = "prod")]
    Prod,
}

#[cfg(test)]
mod tests {
    use crate::configuration::env::get_env_source;

    pub use super::*;

    #[test]
    pub fn test_read_app_config() {
        let _config = AppConfig::read(get_env_source("TEST_APP")).unwrap();
    }

    #[test]
    pub fn test_profile_to_string() {
        let profile: Profile = Profile::try_from("dev").unwrap();
        assert_eq!(profile, Profile::Dev)
    }
}
