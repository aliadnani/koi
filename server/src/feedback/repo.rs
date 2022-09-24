use std::{str::FromStr, sync::Arc};

use async_trait::async_trait;
use chrono::Utc;
use eyre::Result;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OptionalExtension;

use crate::common::new_nanoid;

use super::model::{Feedback, FeedbackCategory, FeedbackMetadata, FeedbackStatus, NewFeedback};

pub type FeedbackRepositoryDyn = Arc<dyn FeedbackRepository + Send + Sync>;

/// `FeedbackRepository` is abstracted to a trait to allow for using a seperate `FeedbackRepository` in tests
#[async_trait]
pub trait FeedbackRepository {
    async fn create_feedback(&self, new_feedback: &NewFeedback) -> Result<Feedback>;

    async fn get_feedback(&self, id: String) -> Result<Option<Feedback>>;

    async fn list_feedback_for_project(&self, project_id: String) -> Result<Vec<Feedback>>;
}

pub struct FeedbackRepositorySqlite {
    conn: Arc<Pool<SqliteConnectionManager>>,
}

impl FeedbackRepositorySqlite {
    pub fn new(conn: Arc<Pool<SqliteConnectionManager>>) -> FeedbackRepositorySqlite {
        FeedbackRepositorySqlite { conn }
    }
}

#[async_trait]
impl FeedbackRepository for FeedbackRepositorySqlite {
    async fn create_feedback(&self, new_feedback: &NewFeedback) -> Result<Feedback> {
        let feedback = Feedback {
            id: new_nanoid(),
            description: new_feedback.description.clone(),
            location: new_feedback.location.clone(),
            status: FeedbackStatus::Default,
            category: new_feedback.category.clone(),
            metadata: FeedbackMetadata {
                created_at: Utc::now(),
                device: "???".to_string(),
            },
            additional_attributes: new_feedback.additional_attributes.clone(),
            project_id: new_feedback.project_id.clone(),
        };

        let _created = self.conn.get()?.execute(
            "
            INSERT INTO feedback (id, description, location, status, category, metadata, additional_attributes, project_id)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);",
            (
                &feedback.id,
                &feedback.description,
                &feedback.location,
                &feedback.status.to_string(),
                &feedback.category.to_string(),
                serde_json::to_string(&feedback.metadata).unwrap(),
                serde_json::to_string(&feedback.additional_attributes).unwrap(),
                &feedback.project_id,
            ),
        )?;

        Ok(feedback)
    }

    async fn get_feedback(&self, feedback_id: String) -> Result<Option<Feedback>> {
        let feedback = self
            .conn
            .get()?
            .query_row(
                "
                SELECT id, description, location, status, category, metadata, additional_attributes, project_id
                FROM feedback
                WHERE id = ?1;
                ",
                [feedback_id],
                |row| {
                    Ok(Feedback {
                        id: row.get(0)?,
                        description: row.get(1)?,
                        location: row.get(2)?,
                        status: {
                            let status: String = row.get(3)?;
                            FeedbackStatus::from_str(&status).unwrap()
                        },
                        category: {
                            let category: String = row.get(4)?;
                            FeedbackCategory::from_str(&category).unwrap()
                        },
                        metadata: {
                            let metadata: String = row.get(5)?;
                            serde_json::from_str(&metadata).unwrap()
                        },
                        additional_attributes: {
                            let additional_attributes: String = row.get(6)?;
                            serde_json::from_str(&additional_attributes).unwrap()
                        },
                        project_id: row.get(7)?,
                    })
                },
            )
            .optional()?;

        Ok(feedback)
    }

    async fn list_feedback_for_project(&self, project_id: String) -> Result<Vec<Feedback>> {
        let feedback: Vec<Feedback> = self
            .conn
            .get()?
            .prepare_cached(
        "
            SELECT id, description, location, status, category, metadata, additional_attributes, project_id
            FROM feedback
            WHERE project_id = ?1;
            "
             )
            .unwrap()
            .query_map([project_id], |row| {
                Ok(Feedback {
                    id: row.get(0)?,
                    description: row.get(1)?,
                    location: row.get(2)?,
                    status: {
                        let status: String = row.get(3)?;
                        FeedbackStatus::from_str(&status).unwrap()
                    },
                    category: {
                        let category: String = row.get(4)?;
                        FeedbackCategory::from_str(&category).unwrap()
                    },
                    metadata: {
                        let metadata: String = row.get(5)?;
                        serde_json::from_str(&metadata).unwrap()
                    },
                    additional_attributes: {
                        let additional_attributes: String = row.get(6)?;
                        serde_json::from_str(&additional_attributes).unwrap()
                    },
                    project_id: row.get(7)?,
                })
            })
            .unwrap()
            .map(|f| f.unwrap())
            .collect();

        Ok(feedback)
    }
}
