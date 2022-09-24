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
    pub user_profile: UserProfile,
    pub projects: Vec<Project>,
}

impl UserProfile {
    pub fn with_projects(&self, projects: Vec<Project>) -> UserProfileWithProjects {
        UserProfileWithProjects {
            user_profile: self.clone(),
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
