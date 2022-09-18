use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use eyre::Result;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OptionalExtension;

use crate::common::new_nanoid;

use super::model::{NewProfile, Profile};

pub type ProfileRepositoryDyn = Arc<dyn ProfileRepository + Send + Sync>;

/// `ProfileRepository` is abstracted to a trait to allow for using a seperate `ProfileRepository` in tests
#[async_trait]
pub trait ProfileRepository {
    async fn create_profile(&self, new_profile: &NewProfile) -> Result<Profile>;
    async fn get_profile(&self, id: &String) -> Result<Option<Profile>>;
}

pub struct ProfileRepositorySqlite {
    conn: Arc<Pool<SqliteConnectionManager>>,
}

impl ProfileRepositorySqlite {
    pub fn new(conn: Arc<Pool<SqliteConnectionManager>>) -> ProfileRepositorySqlite {
        ProfileRepositorySqlite { conn }
    }
}

#[async_trait]
impl ProfileRepository for ProfileRepositorySqlite {
    async fn create_profile(&self, new_profile: &NewProfile) -> Result<Profile> {
        let profile = Profile {
            id: new_nanoid(),
            name: new_profile.name.clone(),
            created_at: Utc::now(),
            email: new_profile.email.clone(),
            projects: vec![],
        };

        let _created = self.conn.get()?.execute(
            "INSERT INTO profiles (id, name, email, created_at) VALUES (?1, ?2, ?3, ?4);",
            (
                &profile.id,
                &profile.name,
                &profile.email,
                &profile.created_at.to_rfc3339(),
            ),
        )?;

        Ok(profile)
    }

    async fn get_profile(&self, profile_id: &String) -> Result<Option<Profile>> {
        let profile = self
            .conn
            .get()?
            .query_row(
                "SELECT id, name, email, created_at FROM profiles WHERE id = ?1",
                [profile_id],
                |row| {
                    Ok(Profile {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        email: row.get(2)?,
                        created_at: {
                            let date: String = row.get(3)?;
                            DateTime::parse_from_rfc3339(&date)
                                .expect("Timezones in db should be rfc3339!")
                                .with_timezone(&Utc)
                        },
                        projects: vec![],
                    })
                },
            )
            .optional()?;

        Ok(profile)
    }
}
