use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};

use crate::{
    http::auth::{AuthBasic, AuthBearer},
    sessions::{Session, SessionRepositoryDyn},
};

use super::{model::NewUserProfile, repo::UserRepositoryDyn};

pub struct UserService {
    pub user_repo: UserRepositoryDyn,
    pub session_repo: SessionRepositoryDyn,
}

impl UserService {
    pub fn new(user_repo: UserRepositoryDyn, session_repo: SessionRepositoryDyn) -> UserService {
        UserService {
            user_repo,
            session_repo,
        }
    }

    pub fn routes(&self) -> Router {
        Router::new()
            .route("/profile", get(get_profile).post(register))
            .route("/register", post(register))
            .route("/login", post(log_in))
            .layer(Extension(self.user_repo.to_owned()))
            .layer(Extension(self.session_repo.to_owned()))
    }
}

#[utoipa::path(
    post,
    path = "/register",
    request_body = NewUserProfile,
    responses(
        (status = 200, description = "Account registered succesfully", body = Session),
    ),
    tag = "Profile API",
    security(
        ()
    )
)]
// TODO: Check duplicate email constraint
async fn register(
    Json(new_profile): Json<NewUserProfile>,
    Extension(user_repo): Extension<UserRepositoryDyn>,
    Extension(session_repo): Extension<SessionRepositoryDyn>,
) -> impl IntoResponse {
    let profile = user_repo.create_profile(new_profile).await.unwrap();
    let token = session_repo
        .create_session(profile.clone().email)
        .await
        .unwrap();

    let session = Session::new(token, profile);

    (StatusCode::OK, Json(session)).into_response()
}

#[utoipa::path(
    post,
    path = "/login",
    responses(
        (status = 200, description = "Logged in successfully", body = Session)
    ),
    security(
        ("Username & Password" = [])
    ),
    tag = "Profile API"
)]
async fn log_in(AuthBasic(token): AuthBasic) -> impl IntoResponse {
    (StatusCode::OK, Json(token)).into_response()
}

#[utoipa::path(
    get,
    path = "/profile",
    responses(
        (status = 200, description = "Profile retrived successfully", body = UserProfileWithProjects)
    ),
    security(
        ("Session Token" = [])
    ),
    tag = "Profile API"
)]
async fn get_profile(
    AuthBearer(user_profile): AuthBearer,
    Extension(user_repo): Extension<UserRepositoryDyn>,
) -> impl IntoResponse {
    let projects = user_repo
        .get_projects_of_user(user_profile.id.clone())
        .await
        .unwrap();

    (StatusCode::OK, Json(user_profile.with_projects(projects))).into_response()
}
