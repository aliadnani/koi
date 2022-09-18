use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};

use super::{model::NewProfile, repo::ProfileRepositoryDyn};

pub struct ProfileService {
    pub profile_repo: ProfileRepositoryDyn,
}

impl ProfileService {
    pub fn new(profile_repo: ProfileRepositoryDyn) -> ProfileService {
        ProfileService { profile_repo }
    }

    pub fn routes(&self) -> Router {
        Router::new()
            .route("/profiles", post(create_profile))
            .route("/profiles/:profile_id", get(get_profile))
            .layer(Extension(self.profile_repo.to_owned()))
    }
}

async fn create_profile(
    Json(new_profile): Json<NewProfile>,
    Extension(repo): Extension<ProfileRepositoryDyn>,
) -> impl IntoResponse {
    let profile = repo.create_profile(&new_profile).await.unwrap();

    (StatusCode::OK, Json(profile)).into_response()
}


async fn get_profile(
    Path(profile_id): Path<String>,
    Extension(repo): Extension<ProfileRepositoryDyn>,
) -> impl IntoResponse {
    let response = match repo.get_profile(&profile_id).await.unwrap() {
        Some(profile) => (StatusCode::OK, Json(profile)).into_response(),
        None => (StatusCode::NOT_FOUND).into_response(),
    };
    response
}
