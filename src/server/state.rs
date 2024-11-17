use std::fmt::Error;
use std::sync::Arc;

//use crate::client::http::HttpClient;
use crate::configuration::app_config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    // pub http: HttpClient,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, Error> {
        //let http = HttpClient::build_from_config(&config)?;
        Ok(Self {
            config: Arc::new(config),
            //     http,
        })
    }
}
