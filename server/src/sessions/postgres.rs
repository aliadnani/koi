use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use eyre::Result;
use nanoid::nanoid;
use sqlx::{query, PgPool};

use crate::common::ALPHANUMERIC;

use super::SessionRepository;

pub struct SessionRepositoryPostgres {
    conn: Arc<PgPool>,
}

impl SessionRepositoryPostgres {
    pub fn new(conn: Arc<PgPool>) -> SessionRepositoryPostgres {
        SessionRepositoryPostgres { conn }
    }
}

#[async_trait]
impl SessionRepository for SessionRepositoryPostgres {
    async fn verify_session(&self, token: String) -> Result<Option<String>> {
        let mut conn = self.conn.acquire().await.unwrap();

        let sesion_option: Option<(String, DateTime<Utc>)> = query!(
            "SELECT user_email, expires_at FROM user_sessions WHERE token = $1",
            &token
        )
        .fetch_optional(&mut conn)
        .await?
        .map(|r| (r.user_email, r.expires_at));

        match sesion_option {
            Some((user_email, expires_at)) => {
                if expires_at.le(&Utc::now()) {
                    return Ok(None);
                }
                return Ok(Some(user_email));
            }
            None => return Ok(None),
        }
    }

    async fn create_session(&self, user_email: String) -> Result<String> {
        let token = nanoid!(30, ALPHANUMERIC);

        let mut conn = self.conn.acquire().await.unwrap();

        let _created = query!(
            "
            INSERT INTO user_sessions (token, user_email, expires_at)
            VALUES ($1, $2, $3);",
            token.clone(),
            user_email.to_string(),
            Utc::now().checked_add_signed(Duration::days(7))
        )
        .execute(&mut conn)
        .await?;

        Ok(token)
    }
    async fn delete_session(&self, token: String) -> Result<()> {
        let mut conn = self.conn.acquire().await.unwrap();

        let _deleted = query!("DELETE FROM user_sessions WHERE token = $1;", token.clone())
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}
