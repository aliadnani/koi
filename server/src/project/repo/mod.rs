use std::sync::Arc;

use async_trait::async_trait;
use eyre::Result;

use crate::users::model::UserProfile;

use super::{
    error::ProjectError,
    model::{NewProject, Project},
};

pub type ProjectRepositoryDyn = Arc<dyn ProjectRepository + Send + Sync>;

pub mod postgres;

/// `ProjectRepository` is abstracted to a trait to allow for using a seperate `ProjectRepository` in tests
#[async_trait]
pub trait ProjectRepository {
    async fn create_project(&self, new_project: &NewProject) -> Result<Project>;
    async fn get_project(&self, id: String) -> Result<Option<Project>>;
    async fn add_user_to_project(
        &self,
        project_id: String,
        user_email: String,
    ) -> Result<(), ProjectError>;
    async fn remove_user_from_project(
        &self,
        project_id: String,
        user_email: String,
    ) -> Result<(), ProjectError>;
    async fn list_users_of_project(&self, project_id: String) -> Result<Vec<UserProfile>>;
    async fn check_if_user_is_part_of_project(
        &self,
        project_id: String,
        email: String,
    ) -> Result<bool>;
}
