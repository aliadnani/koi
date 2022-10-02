use axum::Router;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, net::SocketAddr, sync::Arc};
use tower_http::{
    catch_panic::CatchPanicLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    feedback::{repo::postgres::FeedbackRepositoryPostgres, service::FeedbackService},
    openapi::ApiDoc,
    project::{repo::postgres::ProjectRepositoryPostgres, service::ProjectService},
    sessions::postgres::SessionRepositoryPostgres,
    users::{repo::postgres::UserRepositoryPostgres, service::UserService},
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

    tracing::info!(
        "Pg connection at: {}",
        env::var("DATABASE_URL").unwrap().as_str()
    );

    // Initialize DB
    let pool: PgPool = PgPoolOptions::new()
        .max_connections(50)
        // TODO: Load via config
        // .connect("postgresql://koi:ca5WYy8P4x9CfyXxjrik@localhost:5432/koi?sslmode=disable")
        .connect(env::var("DATABASE_URL").unwrap().as_str())
        .await
        .expect("Could not get Postgres pool");

    let pool = Arc::new(pool);

    // Initialize repos
    let project_repo = Arc::new(ProjectRepositoryPostgres::new(pool.clone()));
    let user_repo = Arc::new(UserRepositoryPostgres::new(pool.clone()));
    let feedback_repo = Arc::new(FeedbackRepositoryPostgres::new(pool.clone()));
    let session_repo = Arc::new(SessionRepositoryPostgres::new(pool.clone()));

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
