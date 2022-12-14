use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use utoipa::ToSchema;

#[derive(EnumString, Display, Deserialize, Serialize, Clone, ToSchema)]
pub enum FeedbackCategory {
    Idea,
    Issue,
    Other,
}

#[derive(EnumString, Display, Deserialize, Serialize, ToSchema)]
pub enum FeedbackStatus {
    Default,
    Archived,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct FeedbackMetadata {
    pub created_at: DateTime<Utc>,
    pub device: String,
}

/// A new feedback entry - with only required fields from client
#[derive(Deserialize, Serialize, ToSchema)]
pub struct NewFeedback {
    /// Main body of feedback
    pub description: String,
    /// Page/URL feedback came from
    pub location: String,
    /// Category of feedback - any of: Idea, Issue, Other
    pub category: FeedbackCategory,
    /// Any additional attributes specifed by client
    pub additional_attributes: HashMap<String, String>,
    /// The project the feedback entry belongs to
    pub project_id: String,
}

/// A standard feedback entry
#[derive(Deserialize, Serialize, ToSchema)]
pub struct Feedback {
    /// Id of feedback
    pub id: String,
    /// Main body of feedback
    pub description: String,
    /// Page/URL feedback came from
    pub location: String,
    /// Status of feedback - any of: Default or Archived
    pub status: FeedbackStatus,
    /// Category of feedback - any of: Idea, Issue, Other
    pub category: FeedbackCategory,
    /// Metadata of feedback calculated on creation
    pub metadata: FeedbackMetadata,
    /// Any additional attributes specifed by client
    pub additional_attributes: HashMap<String, String>,
    /// The project the feedback entry belongs to
    pub project_id: String,
}
