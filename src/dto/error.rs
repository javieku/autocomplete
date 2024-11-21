use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use elasticsearch::Error as ElasticsearchError;
use serde::Serialize;
use serde_json::Error as SerdeJsonError;

use axum::extract::{rejection::JsonRejection, Json};

#[derive(Debug)]
pub enum AutocompleteError {
    // The request body contained invalid JSON
    JsonRejection(JsonRejection),
    ElasticsearchTimeout(ElasticsearchError),
    ElasticsearchSerialization(ElasticsearchError),
    ElasticsearchGatewayError(ElasticsearchError),
    SerdeSerialization(SerdeJsonError),
}

// Tell axum how `AutocompleteError` should be converted into a response.
impl IntoResponse for AutocompleteError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            AutocompleteError::JsonRejection(rejection) => {
                (rejection.status(), rejection.body_text())
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong".to_owned(),
            ),
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}

impl From<ElasticsearchError> for AutocompleteError {
    fn from(es_error: ElasticsearchError) -> Self {
        if es_error.is_json() {
            AutocompleteError::ElasticsearchSerialization(es_error)
        } else if es_error.is_timeout() {
            AutocompleteError::ElasticsearchTimeout(es_error)
        } else {
            AutocompleteError::ElasticsearchGatewayError(es_error)
        }
    }
}

impl From<serde_json::Error> for AutocompleteError {
    fn from(error: SerdeJsonError) -> Self {
        AutocompleteError::SerdeSerialization(error)
    }
}
