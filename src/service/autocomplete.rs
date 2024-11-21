use crate::client::client::ElasticsearchClientExt;
use crate::dto::request::AutocompleteRequest;

use crate::dto::{error::AutocompleteError, response::AutocompleteResponse};
use crate::server::state::AppState;

pub async fn get_suggestions(
    state: &AppState,
    request: AutocompleteRequest,
) -> Result<AutocompleteResponse, AutocompleteError> {
    state.elasticsearch.get_suggestions(&request).await
}
