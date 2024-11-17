use once_cell::sync::Lazy;

use crate::configuration::env::get_env_source;

pub const ENV_PREFIX: &str = "APP";
pub static CONFIG: Lazy<crate::configuration::app_config::AppConfig> = Lazy::new(|| {
    crate::configuration::app_config::AppConfig::read(get_env_source(ENV_PREFIX)).unwrap()
});
