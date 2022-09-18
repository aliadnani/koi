use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(EnumString, Display, Deserialize, Serialize, Clone)]
pub enum FeedbackCategory {
    Idea,
    Issue,
    Other,
}

#[derive(EnumString, Display, Deserialize, Serialize)]
pub enum FeedbackStatus {
    Default,
    Archived,
}

#[derive(Deserialize, Serialize)]
pub struct FeedbackMetadata {
    pub created_at: DateTime<Utc>,
    pub device: String,
}

/// A new feedback entry - with only required fields from client
#[derive(Deserialize, Serialize)]
pub struct NewFeedback {
    /// Id of feedback
    pub id: String,
    /// Main body of feedback
    pub description: String,
    /// Page/URL feedback came from
    pub location: String,
    /// Category of feedback - any of: Idea, Issue, Other
    pub category: FeedbackCategory,
    /// Any additional attributes specifed by client
    pub additional_attributes: HashMap<String, String>,
    // project_id is specified in the path - no need in body
    // pub project_id: String,
}

/// A standard feedback entry
#[derive(Deserialize, Serialize)]
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
    /// The date of which feedback was submitted
    pub created_at: DateTime<Utc>,
}
