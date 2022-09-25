use axum::{Router, http::Method};
use r2d2_sqlite::SqliteConnectionManager;
use std::{net::SocketAddr, sync::Arc};
use tower_http::{catch_panic::CatchPanicLayer, trace::TraceLayer, cors::{CorsLayer, Any}};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    feedback::{repo::FeedbackRepositorySqlite, service::FeedbackService},
    openapi::ApiDoc,
    project::{repo::ProjectRepositorySqlite, service::ProjectService},
    sessions::SessionRepositorySqlite,
    users::{repo::UserRepositorySqlite, service::UserService},
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod common;
mod db;
mod feedback;
mod http;
mod openapi;
mod project;
mod sessions;
mod users;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "koi=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize DB
    let manager = SqliteConnectionManager::file("koi.db");
    // let manager = SqliteConnectionManager::memory();

    let pool = Arc::new(r2d2::Pool::new(manager).expect("Could not acquire SQLite pool."));

    pool.get()
        .expect("Could not get a connection from SQLite pool.")
        .pragma_update(None, "journal_mode", &"WAL")
        .expect("Failed to set journal mode to WAL");

    pool.get()
        .expect("Could not get a connection from SQLite pool.")
        .pragma_update(None, "foreign_keys", &"ON")
        .expect("Failed to enable strict mode");

    pool.get()
        .expect("Could not get a connection from SQLite pool.")
        .pragma_update(None, "strict", &"ON")
        .expect("Failed to enable foreign keys");

    db::sqlite::migrations()
        .to_latest(
            &mut pool
                .get()
                .expect("Could not get a connection from SQLite pool."),
        )
        .expect("Failed to run migrations");

    // Initialize repos
    let project_repo = Arc::new(ProjectRepositorySqlite::new(pool.clone()));
    let user_repo = Arc::new(UserRepositorySqlite::new(pool.clone()));
    let feedback_repo = Arc::new(FeedbackRepositorySqlite::new(pool.clone()));
    let session_repo = Arc::new(SessionRepositorySqlite::new(pool.clone()));

    // Initialize services
    let project_service = ProjectService::new(
        project_repo.clone(),
        feedback_repo.clone(),
        user_repo.clone(),
        session_repo.clone(),
    );
    let profile_service = UserService::new(user_repo.clone(), session_repo.clone());
    let feedback_service = FeedbackService::new(feedback_repo.clone());

    let app = Router::new()
        .merge(project_service.routes())
        .merge(profile_service.routes())
        .merge(feedback_service.routes())
        .layer(TraceLayer::new_for_http())
        .layer(CatchPanicLayer::new())
        .merge(SwaggerUi::new("/swagger-ui/*tail").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .layer(
            // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
            // for more details
            //
            // pay attention that for some request types like posting content-type: application/json
            // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
            // or see this issue https://github.com/tokio-rs/axum/issues/849
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any).allow_headers(Any),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 6122));
    tracing::debug!("Server started on {}!", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Could not start server.");
}
