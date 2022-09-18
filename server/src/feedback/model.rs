use std::collections::HashMap;

use chrono::{DateTime, Utc};

pub enum FeedbackCategory {
    Idea,
    Issue,
    Other,
}

pub enum FeedbackStatus {
    Default,
    Archived,
}

pub struct FeedbackMetadata {
    created_at: DateTime<Utc>,
    device: String,
}

/// A new feedback entry - with only required fields from client
pub struct NewFeedback {
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
    // project_id is specified in the path - no need in body
    // pub project_id: String,
}

/// A standard feedback entry
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
