use crate::{
    configuration::app_config::AppConfig,
    dto::{error::AutocompleteError, request::AutocompleteRequest, response::AutocompleteResponse},
};
use elasticsearch::http::transport::Transport;

use elasticsearch::{Elasticsearch, SearchParts};
use serde::Deserialize;

pub trait ElasticsearchClientExt {
    fn build_from_config(config: &AppConfig) -> Self;
    async fn get_suggestions(
        &self,
        request: &AutocompleteRequest,
    ) -> Result<AutocompleteResponse, AutocompleteError>;
}

#[derive(Deserialize, Debug)]
struct Source {
    name: String,
}

#[derive(Deserialize, Debug)]
struct ElasticsearchResponse {
    suggest: Suggest,
}

#[derive(Deserialize, Debug)]
struct Suggest {
    #[serde(rename = "poi-suggestions")]
    poi_suggestions: Vec<PoiSuggestion>,
}

#[derive(Deserialize, Debug)]
struct PoiSuggestion {
    options: Vec<OptionEntry>,
}

#[derive(Deserialize, Debug)]
struct OptionEntry {
    _source: Source,
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
    async fn get_suggestions(
        &self,
        request: &AutocompleteRequest,
    ) -> Result<AutocompleteResponse, AutocompleteError> {
        let request_json = serde_json::Value::from(request);
        println!("{:?}", request_json);
        let index_name = SearchParts::Index(&["poi_v1"]);
        let search_response = self.search(index_name).body(request_json).send().await?;

        let response_body = search_response.json::<serde_json::Value>().await?;
        let suggest: Suggest = serde_json::from_value(response_body["suggest"].clone())?;

        let mut result = Vec::new();
        for suggestion in suggest.poi_suggestions {
            for option in suggestion.options {
                println!("Name: {}", option._source.name);
                result.push(option._source.name);
            }
        }

        Ok(AutocompleteResponse::new(result))
    }
}
