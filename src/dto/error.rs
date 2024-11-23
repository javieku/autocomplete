use axum::extract::rejection::JsonDataError;
use axum::extract::{rejection::JsonRejection, Json};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use elasticsearch::Error as ElasticsearchError;
use garde::Report;
use log::{error, trace};
use serde::Serialize;
use serde_json::Error as SerdeJsonError;

#[derive(Debug)]
pub enum AutocompleteError {
    GardeValidationError(Report),
    JsonExtractorRejection(JsonRejection),
    JsonDataError(JsonDataError),
    ElasticsearchTimeout(ElasticsearchError),
    ElasticsearchSerialization(ElasticsearchError),
    ElasticsearchGatewayError(ElasticsearchError),
    SerdeSerialization(SerdeJsonError),
    UnknownError(anyhow::Error),
}

impl IntoResponse for AutocompleteError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            AutocompleteError::JsonExtractorRejection(rejection) => {
                (rejection.status(), rejection.body_text())
            }
            AutocompleteError::JsonDataError(rejection) => {
                (rejection.status(), rejection.body_text())
            }
            AutocompleteError::ElasticsearchTimeout(es_error) => {
                (StatusCode::GATEWAY_TIMEOUT, es_error.to_string())
            }
            AutocompleteError::ElasticsearchGatewayError(es_error) => (
                es_error.status_code().unwrap_or(StatusCode::BAD_GATEWAY),
                es_error.to_string(),
            ),
            AutocompleteError::ElasticsearchSerialization(es_error) => (
                es_error.status_code().unwrap_or(StatusCode::BAD_GATEWAY),
                es_error.to_string(),
            ),
            AutocompleteError::GardeValidationError(report) => {
                (StatusCode::BAD_REQUEST, report.to_string())
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong".to_owned(),
            ),
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}

impl From<garde::Report> for AutocompleteError {
    fn from(report: garde::Report) -> Self {
        AutocompleteError::GardeValidationError(report)
    }
}

impl From<ElasticsearchError> for AutocompleteError {
    fn from(es_error: ElasticsearchError) -> Self {
        error!("ElasticsearchError: {}", es_error);
        if es_error.is_json() {
            AutocompleteError::ElasticsearchSerialization(es_error)
        } else if es_error.is_timeout() {
            AutocompleteError::ElasticsearchTimeout(es_error)
        } else {
            AutocompleteError::ElasticsearchGatewayError(es_error)
        }
    }
}

impl From<JsonRejection> for AutocompleteError {
    fn from(rejection: JsonRejection) -> Self {
        AutocompleteError::JsonExtractorRejection(rejection)
    }
}

impl From<JsonDataError> for AutocompleteError {
    fn from(rejection: JsonDataError) -> Self {
        AutocompleteError::JsonDataError(rejection)
    }
}

impl From<anyhow::Error> for AutocompleteError {
    fn from(error: anyhow::Error) -> Self {
        error!(
            "Root_cause: {} \r\n source: {:?} \r\n error: {:?}",
            error.root_cause(),
            error.source(),
            error.to_string()
        );

        trace!("Backtrace: {}", error.backtrace());

        match error.downcast::<ElasticsearchError>() {
            Ok(es_error) => AutocompleteError::from(es_error),
            Err(e) => match e.downcast::<serde_json::Error>() {
                Ok(serde_error) => AutocompleteError::from(serde_error),
                Err(e) => AutocompleteError::UnknownError(e),
            },
        }
    }
}

impl From<serde_json::Error> for AutocompleteError {
    fn from(error: SerdeJsonError) -> Self {
        error!("SerdeJsonError: {}", error);
        AutocompleteError::SerdeSerialization(error)
    }
}
