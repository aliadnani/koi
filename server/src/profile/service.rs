use axum::{routing::{post}, Router};

pub struct UserService {}

impl UserService {
    pub fn routes() -> Router {
        Router::new()
            .route("/projects", post(create_project))
    }
}

async fn create_project() -> String {
    format!("New project created!")
}
async fn get_project(Path(project_id): String) -> String {
    format!("New project created!")
}
