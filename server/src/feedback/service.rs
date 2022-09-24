use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing::post, Extension, Json, Router,
};

use super::{model::NewFeedback, repo::FeedbackRepositoryDyn};

pub struct FeedbackService {
    pub feedback_repo: FeedbackRepositoryDyn,
}

impl FeedbackService {
    pub fn new(feedback_repo: FeedbackRepositoryDyn) -> FeedbackService {
        FeedbackService { feedback_repo }
    }

    pub fn routes(&self) -> Router {
        Router::new()
            .route("/feedback", post(create_feedback))
            .layer(Extension(self.feedback_repo.to_owned()))
    }
}

// TODO: Check project_id constraint
async fn create_feedback(
    Json(new_feedback): Json<NewFeedback>,
    Extension(repo): Extension<FeedbackRepositoryDyn>,
) -> impl IntoResponse {
    let feedback = repo.create_feedback(&new_feedback).await.unwrap();

    (StatusCode::OK, Json(feedback)).into_response()
}

async fn get_feedback(
    Path(feedback_id): Path<String>,
    Extension(repo): Extension<FeedbackRepositoryDyn>,
) -> impl IntoResponse {
    match repo.get_feedback(feedback_id).await.unwrap() {
        Some(feedback) => (StatusCode::OK, Json(feedback)).into_response(),
        None => (StatusCode::NOT_FOUND).into_response(),
    }
}
