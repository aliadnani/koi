use std::{str::FromStr, sync::Arc};

use async_trait::async_trait;
use chrono::Utc;
use eyre::Result;
use sqlx::{query, PgPool};

use crate::{
    common::new_nanoid,
    feedback::model::{Feedback, FeedbackCategory, FeedbackMetadata, FeedbackStatus, NewFeedback},
};

use super::FeedbackRepository;

pub struct FeedbackRepositoryPostgres {
    conn: Arc<PgPool>,
}

impl FeedbackRepositoryPostgres {
    pub fn new(conn: Arc<PgPool>) -> FeedbackRepositoryPostgres {
        FeedbackRepositoryPostgres { conn }
    }
}

#[async_trait]
impl FeedbackRepository for FeedbackRepositoryPostgres {
    async fn create_feedback(&self, new_feedback: &NewFeedback) -> Result<Feedback> {
        let mut conn = self.conn.acquire().await.unwrap();

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

        let _created = query!(
            "
            INSERT INTO feedback (id, description, location, status, category, metadata, additional_attributes, project_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8);",
                &feedback.id,
                &feedback.description,
                &feedback.location,
                &feedback.status.to_string(),
                &feedback.category.to_string(),
                serde_json::to_value(&feedback.metadata).unwrap(),
                serde_json::to_value(&feedback.additional_attributes).unwrap(),
                &feedback.project_id,
        ).execute(&mut conn);

        Ok(feedback)
    }

    async fn get_feedback(&self, feedback_id: String) -> Result<Option<Feedback>> {
        let mut conn = self.conn.acquire().await.unwrap();

        let feedback = query!(

            r#"
            SELECT id, description, location, status, category, metadata, additional_attributes, project_id
            FROM feedback
            WHERE id = $1;
            "#, &feedback_id

        )
        .fetch_optional(&mut conn)
        .await?
        .map(|r| Feedback {
            id: r.id,
            description: r.description,
            location: r.location,
            status: FeedbackStatus::from_str(&r.status).unwrap(),
            category: FeedbackCategory::from_str(&r.category).unwrap(),
            metadata: serde_json::from_value(r.metadata).unwrap(),
            additional_attributes: serde_json::from_value(r.additional_attributes).unwrap(),
            project_id: r.project_id,
        });

        Ok(feedback)
    }

    async fn list_feedback_for_project(&self, project_id: String) -> Result<Vec<Feedback>> {
        let mut conn = self.conn.acquire().await.unwrap();

        let feedback = query!(
            r#"
            SELECT id, description, location, status, category, metadata, additional_attributes, project_id
            FROM feedback
            WHERE project_id = $1;
            "#, &project_id
        )
        .fetch_all(&mut conn)
        .await?
        .iter()
        .map(|r| Feedback {
            id: r.id.clone(),
            description: r.description.clone(),
            location: r.location.clone(),
            status: FeedbackStatus::from_str(&r.status).unwrap(),
            category: FeedbackCategory::from_str(&r.category).unwrap(),
            metadata: serde_json::from_value(r.metadata.clone()).unwrap(),
            additional_attributes: serde_json::from_value(r.additional_attributes.clone()).unwrap(),
            project_id: r.project_id.clone(),
        })
        .collect();

        Ok(feedback)
    }
}
