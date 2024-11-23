use crate::dto::error::AutocompleteError;
use crate::dto::request::AutocompleteRequest;
use crate::dto::response::AutocompleteResponse;
use crate::server::state::AppState;
use crate::service::autocomplete;
use anyhow::Result;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::Router;
use axum_macros::FromRequest;
use garde::Validate;
use log::{info, trace};

#[axum_macros::debug_handler]
pub async fn autocomplete(
    State(state): State<AppState>,
    JsonWithCustomError(request): JsonWithCustomError<AutocompleteRequest>,
) -> Result<JsonWithCustomError<AutocompleteResponse>, AutocompleteError> {
    if let Err(e) = request.validate() {
        info!("Invalid request: {e}");
        return Err(AutocompleteError::from(e));
    }

    match autocomplete::get_suggestions(&state, request).await {
        Ok(response) => {
            trace!("Successfully found suggestions");
            Ok(JsonWithCustomError(response))
        }
        Err(e) => {
            trace!("There was an error while fetching suggestions {e:?}");
            Err(AutocompleteError::from(e))
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

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AutocompleteError))]
pub struct JsonWithCustomError<T>(T);

impl<T> IntoResponse for JsonWithCustomError<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}
