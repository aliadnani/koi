use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use serde_json::json;

use crate::{
    feedback::repo::FeedbackRepositoryDyn,
    http::auth::AuthBearer,
    sessions::SessionRepositoryDyn,
    users::{model::UserProjectAdditionRemoval, repo::UserRepositoryDyn},
};

use super::{error::ProjectError, model::NewProject, repo::ProjectRepositoryDyn};

pub struct ProjectService {
    pub project_repo: ProjectRepositoryDyn,
    pub feedback_repo: FeedbackRepositoryDyn,
    pub user_repo: UserRepositoryDyn,
    pub session_repo: SessionRepositoryDyn,
}

impl ProjectService {
    pub fn new(
        project_repo: ProjectRepositoryDyn,
        feedback_repo: FeedbackRepositoryDyn,
        user_repo: UserRepositoryDyn,
        session_repo: SessionRepositoryDyn,
    ) -> ProjectService {
        ProjectService {
            project_repo,
            feedback_repo,
            user_repo,
            session_repo,
        }
    }

    pub fn routes(&self) -> Router {
        Router::new()
            .route("/projects", post(create_project))
            .route("/projects/:project_id", get(get_project))
            .route("/projects/:project_id/feedback", get(get_project_feedback))
            .route(
                "/projects/:project_id/users",
                post(add_user_to_project)
                    .delete(remove_user_from_project)
                    .get(list_users_of_project),
            )
            .layer(Extension(self.project_repo.to_owned()))
            .layer(Extension(self.feedback_repo.to_owned()))
            .layer(Extension(self.user_repo.to_owned()))
            .layer(Extension(self.session_repo.to_owned()))
    }
}

async fn create_project(
    Json(new_project): Json<NewProject>,
    Extension(project_repo): Extension<ProjectRepositoryDyn>,
    AuthBearer(user_profile): AuthBearer,
) -> impl IntoResponse {
    let project = project_repo.create_project(&new_project).await.unwrap();

    match project_repo
        .add_user_to_project(&project.id, &user_profile.email)
        .await
    {
        Ok(_) => (),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }

    (StatusCode::OK, Json(project)).into_response()
}

async fn get_project(
    Path(project_id): Path<String>,
    Extension(project_repo): Extension<ProjectRepositoryDyn>,
    AuthBearer(user_profile): AuthBearer,
) -> impl IntoResponse {
    match project_repo
        .check_if_user_is_part_of_project(&project_id, &user_profile.email)
        .await
        .unwrap()
    {
        true => return (StatusCode::FORBIDDEN).into_response(),
        false => (),
    }

    match project_repo.get_project(&project_id).await.unwrap() {
        Some(project) => (StatusCode::OK, Json(project)).into_response(),
        None => (StatusCode::NOT_FOUND).into_response(),
    }
}

async fn get_project_feedback(
    Path(project_id): Path<String>,
    Extension(project_repo): Extension<ProjectRepositoryDyn>,
    Extension(feedback_repo): Extension<FeedbackRepositoryDyn>,
    AuthBearer(user_profile): AuthBearer,
) -> impl IntoResponse {
    match project_repo
        .check_if_user_is_part_of_project(&project_id, &user_profile.email)
        .await
        .unwrap()
    {
        true => (),
        false => return (StatusCode::FORBIDDEN).into_response(),
    }

    let _project_exists = match project_repo.get_project(&project_id).await.unwrap() {
        Some(project) => project,
        None => return (StatusCode::NOT_FOUND).into_response(),
    };

    let feedback = feedback_repo
        .list_feedback_for_project(&project_id)
        .await
        .unwrap();

    (StatusCode::OK, Json(feedback)).into_response()
}

async fn add_user_to_project(
    Path(project_id): Path<String>,
    Json(user_project_addition): Json<UserProjectAdditionRemoval>,
    Extension(project_repo): Extension<ProjectRepositoryDyn>,
    AuthBearer(user_profile): AuthBearer,
) -> impl IntoResponse {
    match project_repo
        .check_if_user_is_part_of_project(&project_id, &user_profile.email)
        .await
        .unwrap()
    {
        true => return (StatusCode::FORBIDDEN).into_response(),
        false => (),
    }

    let feedback = match project_repo
        .add_user_to_project(&project_id, &user_project_addition.email)
        .await
    {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(err) => match err {
            ProjectError::ProjectNonExistent => (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Project does not exist"})),
            )
                .into_response(),
            ProjectError::UserNonExistent => (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "User does not exist"})),
            )
                .into_response(),
            ProjectError::UserAlreadyPartOfProject => {
                (StatusCode::UNPROCESSABLE_ENTITY).into_response()
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        },
    };

    feedback
}

async fn remove_user_from_project(
    Path(project_id): Path<String>,
    Query(user_project_removal): Query<UserProjectAdditionRemoval>,
    Extension(project_repo): Extension<ProjectRepositoryDyn>,
    AuthBearer(user_profile): AuthBearer,
) -> impl IntoResponse {
    match project_repo
        .check_if_user_is_part_of_project(&project_id, &user_profile.email)
        .await
        .unwrap()
    {
        true => return (StatusCode::FORBIDDEN).into_response(),
        false => (),
    }

    let feedback = match project_repo
        .remove_user_from_project(&project_id, &user_project_removal.email)
        .await
    {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(err) => match err {
            ProjectError::ProjectNonExistent => (StatusCode::NOT_FOUND).into_response(),
            ProjectError::UserNonExistent => (StatusCode::NOT_FOUND).into_response(),
            ProjectError::UserNotPartOfProject => {
                (StatusCode::UNPROCESSABLE_ENTITY).into_response()
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        },
    };
    feedback
}

async fn list_users_of_project(
    Path(project_id): Path<String>,
    Extension(project_repo): Extension<ProjectRepositoryDyn>,
) -> impl IntoResponse {
    let _project_exists = match project_repo.get_project(&project_id).await.unwrap() {
        Some(project) => project,
        None => return (StatusCode::NOT_FOUND).into_response(),
    };

    let users = match project_repo.list_users_of_project(&project_id).await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };
    users
}
