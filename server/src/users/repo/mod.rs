use std::sync::Arc;

use crate::project::model::Project;
use async_trait::async_trait;
use eyre::Result;

use super::model::{NewUserProfile, UserProfile};

pub mod postgres;

pub type UserRepositoryDyn = Arc<dyn UserRepository + Send + Sync>;

/// `UserRepository` is abstracted to a trait to allow for using a seperate `UserRepository` in tests
#[async_trait]
pub trait UserRepository {
    async fn create_profile(&self, new_profile: NewUserProfile) -> Result<UserProfile>;
    async fn validate_profile_credentials(
        &self,
        email: String,
        password: String,
    ) -> Result<Option<UserProfile>>;
    async fn get_profile(&self, user_id: String) -> Result<Option<UserProfile>>;
    async fn get_projects_of_user(&self, user_id: String) -> Result<Vec<Project>>;
    async fn get_profile_by_email(&self, email: String) -> Result<Option<UserProfile>>;
}
