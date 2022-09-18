use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FeedbackError {
    #[error("Could not post feedback because its designated project, `{0}`, does not exist")]
    ProjectNonExistent(String),
    #[error("Could not get feedback because it `{0}` does not exist")]
    NotFound(String),
    #[error("Unknown error")]
    Unknown,
}

impl IntoResponse for FeedbackError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message_opt) = match self {
            FeedbackError::ProjectNonExistent(s) => (StatusCode::BAD_REQUEST, Some(s)),
            FeedbackError::NotFound(s) => (StatusCode::NOT_FOUND, Some(s)),
            FeedbackError::Unknown => (StatusCode::INTERNAL_SERVER_ERROR, None),
        };

        match error_message_opt {
            Some(err_msg) => (status, Json(json!({ "error": err_msg }))).into_response(),
            None => status.into_response(),
        }
    }
}
