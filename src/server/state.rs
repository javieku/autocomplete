use crate::client::client::ElasticsearchClientExt;
use crate::configuration::app_config::AppConfig;
use elasticsearch::Elasticsearch;
use std::fmt::Error;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub elasticsearch: Arc<Elasticsearch>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, Error> {
        let elasticsearch = Arc::new(Elasticsearch::build_from_config(&config));
        Ok(Self {
            config: Arc::new(config),
            elasticsearch,
        })
    }
}
