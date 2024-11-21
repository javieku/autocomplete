use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ElasticsearchConfig {
    pub url: String,
    pub index: String,
    pub timeout: u64,
}
