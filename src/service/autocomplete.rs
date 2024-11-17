use axum::Error;

use crate::dto::request::AutocompleteRequest;

use crate::dto::response::AutocompleteResponse;
use crate::server::state::AppState;

pub async fn get_suggestions(
    _state: AppState,
    _req: AutocompleteRequest,
) -> Result<AutocompleteResponse, Error> {
    unimplemented!()
}
