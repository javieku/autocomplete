use crate::client::client::ElasticsearchClientExt;
use crate::dto::request::AutocompleteRequest;
use crate::dto::response::AutocompleteResponse;
use crate::server::state::AppState;
use anyhow::Result;

pub async fn get_suggestions(
    state: &AppState,
    request: AutocompleteRequest,
) -> Result<AutocompleteResponse> {
    state.elasticsearch.get_suggestions(&request).await
}
