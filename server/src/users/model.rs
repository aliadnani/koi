use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::project::model::Project;

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct UserProfile {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserProfileWithProjects {
    pub id: String,
    pub name: String,
    pub email: String,
    pub projects: Vec<Project>,
    pub created_at: DateTime<Utc>,
}

impl UserProfile {
    pub fn with_projects(&self, projects: Vec<Project>) -> UserProfileWithProjects {
        UserProfileWithProjects {
            id: self.id.clone(),
            name: self.name.clone(),
            email: self.email.clone(),
            created_at: self.created_at.clone(),
            projects,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewUserProfile {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserProjectAdditionRemoval {
    pub email: String,
}
