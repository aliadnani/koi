use async_trait::async_trait;
use eyre::Result;
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::users::model::UserProfile;

pub mod sqlite;
pub mod stretto;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, ToSchema)]
pub struct Session {
    pub token: String,
    pub user_profile: UserProfile,
}

impl Session {
    pub fn new(token: String, user_profile: UserProfile) -> Session {
        Session {
            token,
            user_profile,
        }
    }
}

pub type SessionRepositoryDyn = Arc<dyn SessionRepository + Send + Sync>;

/// `SessionRepository` is abstracted to a trait to allow for using a seperate `SessionRepository` in tests
#[async_trait]
pub trait SessionRepository {
    async fn verify_session(&self, token: String) -> Result<Option<String>>;
    async fn create_session(&self, user_email: String) -> Result<String>;
    async fn delete_session(&self, token: String) -> Result<()>;
}
