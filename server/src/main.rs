use axum::Router;
use r2d2_sqlite::SqliteConnectionManager;
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    profile::{repo::ProfileRepositorySqlite, service::ProfileService},
    project::{repo::ProjectRepositorySqlite, service::ProjectService},
};

mod common;
mod db;
mod feedback;
mod profile;
mod project;

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
    let manager = SqliteConnectionManager::memory();
    let pool = Arc::new(r2d2::Pool::new(manager).unwrap());

    pool.get()
        .unwrap()
        .pragma_update(None, "journal_mode", &"WAL")
        .expect("Failed to set journal mode to WAL");

    db::sqlite::migrations()
        .to_latest(&mut pool.get().unwrap())
        .expect("Failed to run migrations");

    // Initialize repos
    let project_repo = Arc::new(ProjectRepositorySqlite::new(pool.clone()));
    let profile_repo = Arc::new(ProfileRepositorySqlite::new(pool.clone()));

    // Initialize services
    let project_service = ProjectService::new(project_repo);
    let profile_service = ProfileService::new(profile_repo);

    let app = Router::new()
        .merge(project_service.routes())
        .merge(profile_service.routes())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 6122));
    tracing::debug!("Server started on {}!", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
