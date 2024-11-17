use crate::dto::request::AutocompleteRequest;
use crate::dto::response::AutocompleteResponse;
use crate::server::state::AppState;
use crate::service::autocomplete;
use axum::extract::State;
use axum::routing::post;
use axum::Json;
use axum::Router;

#[axum_macros::debug_handler]
pub async fn autocomplete(
    State(state): State<AppState>,
    Json(request): Json<AutocompleteRequest>,
) -> Result<Json<AutocompleteResponse>, String> {
    match autocomplete::get_suggestions(state, request).await {
        Ok(response) => {
            print!("Successfully found suggestions");
            Ok(Json(response))
        }
        Err(e) => {
            print!("There was an error while fetching suggestions {e:?}");
            Err(e.to_string())
        }
    }
}

pub fn add_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router.route("/api/v1/autocomplete", post(self::autocomplete))
}

pub fn create_router_app(state: AppState) -> Router {
    let router = Router::new();
    let router = self::add_routers(router);
    router.with_state(state)
}
