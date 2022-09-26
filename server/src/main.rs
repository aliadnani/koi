use axum::Router;
use r2d2_sqlite::SqliteConnectionManager;
use std::{net::SocketAddr, sync::Arc};
use stretto::AsyncCache;
use tower_http::{
    catch_panic::CatchPanicLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    feedback::{repo::FeedbackRepositorySqlite, service::FeedbackService},
    openapi::ApiDoc,
    project::{repo::ProjectRepositorySqlite, service::ProjectService},
    sessions::stretto::SessionRepositoryStretto,
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
    let pool = Arc::new(r2d2::Pool::new(manager).expect("Could not acquire SQLite pool."));

    // Runs db migrations and sets sqlite config
    db::sqlite::start_up(pool.clone());

    // Initialize session cache
    let cache = Arc::new(
        AsyncCache::<String, String>::new(
            // Max 100_000_000 sessions in cache
            100_000_000,
            // I have no idea what cost is
            // need to look into https://github.com/dgraph-io/ristrettomore
            1_073_741_824,
            tokio::spawn,
        )
        .expect("Could not construct stretto cache"),
    );

    // Initialize repos
    let project_repo = Arc::new(ProjectRepositorySqlite::new(pool.clone()));
    let user_repo = Arc::new(UserRepositorySqlite::new(pool.clone()));
    let feedback_repo = Arc::new(FeedbackRepositorySqlite::new(pool.clone()));
    let session_repo = Arc::new(SessionRepositoryStretto::new(cache.clone()));

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
        // App services
        .merge(project_service.routes())
        .merge(profile_service.routes())
        .merge(feedback_service.routes())
        .merge(SwaggerUi::new("/swagger-ui/*tail").url("/api-doc/openapi.json", ApiDoc::openapi()))
        // Logging
        .layer(TraceLayer::new_for_http())
        // Return 500s on panics
        .layer(CatchPanicLayer::new())
        // CORS
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 6122));
    tracing::debug!("Server started on {}!", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Could not start server.");
}
