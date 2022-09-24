use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone, ToSchema)]

pub struct NewProject {
    pub name: String,
}
