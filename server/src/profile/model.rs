use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::project::model::Project;

#[derive(Debug, Serialize, Clone)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub email: String,
    pub projects: Vec<Project>,
    pub created_at: DateTime<Utc>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct NewProfile {
    pub name: String,
    pub email: String,
}
