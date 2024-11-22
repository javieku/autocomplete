use super::response::Suggest;
use crate::{
    configuration::app_config::AppConfig,
    dto::{request::AutocompleteRequest, response::AutocompleteResponse},
};
use anyhow::Result;
use elasticsearch::http::transport::Transport;
use elasticsearch::{Elasticsearch, SearchParts};
use log::trace;

pub trait ElasticsearchClientExt {
    fn build_from_config(config: &AppConfig) -> Self;
    async fn get_suggestions(&self, request: &AutocompleteRequest) -> Result<AutocompleteResponse>;
}

impl ElasticsearchClientExt for Elasticsearch {
    fn build_from_config(config: &AppConfig) -> Self {
        let transport = Transport::single_node(&config.elasticsearch.url);
        if transport.is_ok() {
            return Elasticsearch::new(transport.unwrap());
        } else {
            return Elasticsearch::default();
        }
    }
    async fn get_suggestions(&self, request: &AutocompleteRequest) -> Result<AutocompleteResponse> {
        let request_json = serde_json::Value::from(request);
        trace!("Elasticsearch request to be executed: {:?}", request_json);
        let index_name = SearchParts::Index(&["poi_v1"]);
        let search_response = self.search(index_name).body(request_json).send().await?;

        let response_body = search_response.json::<serde_json::Value>().await?;
        trace!("Elasticsearch Response: {}", response_body);

        let suggest: Suggest = serde_json::from_value(response_body["suggest"].clone())?;

        Ok(AutocompleteResponse::from(suggest))
    }
}
