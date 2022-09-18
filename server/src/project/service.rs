use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};

use super::{model::NewProject, repo::ProjectRepositoryDyn};

pub struct ProjectService {
    pub project_repo: ProjectRepositoryDyn,
}

impl ProjectService {
    pub fn new(project_repo: ProjectRepositoryDyn) -> ProjectService {
        ProjectService { project_repo }
    }

    pub fn routes(&self) -> Router {
        Router::new()
            .route("/projects", post(create_project))
            .route("/projects/:project_id", get(get_project))
            .layer(Extension(self.project_repo.to_owned()))
    }
}

async fn create_project(
    Json(new_project): Json<NewProject>,
    Extension(repo): Extension<ProjectRepositoryDyn>,
) -> impl IntoResponse {
    let project = repo.create_project(&new_project).await.unwrap();

    (StatusCode::OK, Json(project)).into_response()
}

async fn get_project(
    Path(project_id): Path<String>,
    Extension(repo): Extension<ProjectRepositoryDyn>,
) -> impl IntoResponse {
    let response = match repo.get_project(&project_id).await.unwrap() {
        Some(project) => (StatusCode::OK, Json(project)).into_response(),
        None => (StatusCode::NOT_FOUND).into_response(),
    };
    response
}
