use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;

use crate::common::new_nanoid;

use super::model::{NewProject, Project};

pub struct ProjectService {}

impl ProjectService {
    pub fn routes() -> Router {
        Router::new()
            .route("/projects", post(create_project))
            .route("/projects/:project_id", get(get_project))
    }
}

async fn create_project(Json(new_project): Json<NewProject>) -> String {
    let project = Project {
        id: new_nanoid(),
        name: new_project.name,
        created_at: Utc::now(),
    };
    format!("New project created!")
}

async fn get_project(Path(project_id): Path<String>) -> String {
    format!("Project {} retrieved!", project_id)
}
