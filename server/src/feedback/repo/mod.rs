use std::sync::Arc;

use async_trait::async_trait;
use eyre::Result;

use super::model::{Feedback, NewFeedback};

pub mod postgres;
pub mod sqlite;

pub type FeedbackRepositoryDyn = Arc<dyn FeedbackRepository + Send + Sync>;

/// `FeedbackRepository` is abstracted to a trait to allow for using a seperate `FeedbackRepository` in tests
#[async_trait]
pub trait FeedbackRepository {
    async fn create_feedback(&self, new_feedback: &NewFeedback) -> Result<Feedback>;

    async fn get_feedback(&self, id: String) -> Result<Option<Feedback>>;

    async fn list_feedback_for_project(&self, project_id: String) -> Result<Vec<Feedback>>;
}
