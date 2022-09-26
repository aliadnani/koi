use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use eyre::Result;
use nanoid::nanoid;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OptionalExtension;

use crate::common::ALPHANUMERIC;

use super::SessionRepository;

pub struct SessionRepositorySqlite {
    conn: Arc<Pool<SqliteConnectionManager>>,
}

impl SessionRepositorySqlite {
    pub fn new(conn: Arc<Pool<SqliteConnectionManager>>) -> SessionRepositorySqlite {
        SessionRepositorySqlite { conn }
    }
}

#[async_trait]
impl SessionRepository for SessionRepositorySqlite {
    async fn verify_session(&self, token: String) -> Result<Option<String>> {
        let sesion_option: Option<(String, String)> = self
            .conn
            .get()
            .unwrap()
            .query_row(
                "SELECT user_email, expires_at FROM user_sessions WHERE token = ?1",
                [token.clone()],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()
            .unwrap();

        match sesion_option {
            Some((user_email, expires_at)) => {
                let expires_at = DateTime::parse_from_rfc3339(&expires_at)
                    .expect("Timezones in db should be rfc3339!")
                    .with_timezone(&Utc);

                if expires_at.le(&Utc::now()) {
                    self.delete_session(token.clone()).await.unwrap();
                    return Ok(None);
                }
                return Ok(Some(user_email));
            }
            None => return Ok(None),
        }
    }

    async fn create_session(&self, user_email: String) -> Result<String> {
        let token = nanoid!(30, ALPHANUMERIC);

        self.conn
            .get()
            .unwrap()
            .execute(
                "
                INSERT INTO user_sessions (token, user_email, expires_at)
                VALUES (?1, ?2, ?3);",
                [
                    token.clone(),
                    user_email.to_string(),
                    Utc::now()
                        .checked_add_signed(Duration::days(7))
                        .unwrap()
                        .to_rfc3339(),
                ],
            )
            .unwrap();

        Ok(token)
    }
    async fn delete_session(&self, token: String) -> Result<()> {
        self.conn
            .get()
            .unwrap()
            .execute("DELETE FROM user_sessions WHERE token = ?1;", [token])
            .unwrap();

        Ok(())
    }
}
