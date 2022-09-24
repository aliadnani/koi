use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use eyre::Result;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OptionalExtension;

use crate::{common::new_nanoid, project::model::Project};

use super::model::{NewUserProfile, UserProfile};

pub type UserRepositoryDyn = Arc<dyn UserRepository + Send + Sync>;

/// `UserRepository` is abstracted to a trait to allow for using a seperate `UserRepository` in tests
#[async_trait]
pub trait UserRepository {
    async fn create_profile(&self, new_profile: NewUserProfile) -> Result<UserProfile>;
    async fn validate_profile_credentials(
        &self,
        email: String,
        password: String,
    ) -> Result<Option<UserProfile>>;
    async fn get_profile(&self, user_id: String) -> Result<Option<UserProfile>>;
    async fn get_projects_of_user(&self, user_id: String) -> Result<Vec<Project>>;
    async fn get_profile_by_email(&self, email: String) -> Result<Option<UserProfile>>;
}

pub struct UserRepositorySqlite {
    conn: Arc<Pool<SqliteConnectionManager>>,
}

impl UserRepositorySqlite {
    pub fn new(conn: Arc<Pool<SqliteConnectionManager>>) -> UserRepositorySqlite {
        UserRepositorySqlite { conn }
    }
}

#[async_trait]
impl UserRepository for UserRepositorySqlite {
    async fn create_profile(&self, new_profile: NewUserProfile) -> Result<UserProfile> {
        let password_salt = nanoid::nanoid!(20);

        let password_hash = argon2::hash_encoded(
            new_profile.password.as_bytes(),
            password_salt.as_bytes(),
            &argon2::Config::default(),
        )
        .unwrap();

        let profile = UserProfile {
            id: new_nanoid(),
            name: new_profile.name.clone(),
            created_at: Utc::now(),
            email: new_profile.email.clone(),
        };

        let _created = self.conn.get()?.execute(
            "
            INSERT INTO users (id, name, email, password_hash, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5);",
            (
                &profile.id,
                &profile.name,
                &profile.email,
                &password_hash,
                &profile.created_at.to_rfc3339(),
            ),
        )?;

        Ok(profile)
    }
    async fn validate_profile_credentials(
        &self,
        email: String,
        password: String,
    ) -> Result<Option<UserProfile>> {
        let profile_and_hash: Option<(UserProfile, String)> = self
            .conn
            .get()?
            .query_row(
                "SELECT id, name, email, created_at, password_hash FROM users WHERE email = ?1",
                [&email],
                |row| {
                    Ok((
                        UserProfile {
                            id: row.get(0)?,
                            name: row.get(1)?,
                            email: row.get(2)?,
                            created_at: {
                                let date: String = row.get(3)?;
                                DateTime::parse_from_rfc3339(&date)
                                    .expect("Timezones in db should be rfc3339!")
                                    .with_timezone(&Utc)
                            },
                        },
                        row.get(4)?,
                    ))
                },
            )
            .optional()
            .unwrap();

        match profile_and_hash {
            Some((user_profile, password_hash)) => {
                match argon2::verify_encoded(&password_hash, &password.as_bytes()).unwrap() {
                    true => Ok(Some(user_profile)),
                    false => Ok(None),
                }
            }
            None => Ok(None),
        }
    }

    async fn get_profile(&self, user_id: String) -> Result<Option<UserProfile>> {
        let profile = self
            .conn
            .get()?
            .query_row(
                "SELECT id, name, email, created_at FROM users WHERE id = ?1",
                [user_id],
                |row| {
                    Ok(UserProfile {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        email: row.get(2)?,
                        created_at: {
                            let date: String = row.get(3)?;
                            DateTime::parse_from_rfc3339(&date)
                                .expect("Timezones in db should be rfc3339!")
                                .with_timezone(&Utc)
                        },
                    })
                },
            )
            .optional()?;

        Ok(profile)
    }

    async fn get_projects_of_user(&self, user_id: String) -> Result<Vec<Project>> {
        let projects: Vec<Project> = self
            .conn
            .get()?
            .prepare_cached(
                "
                SELECT p.id, p.name, p.created_at
                FROM projects p
                JOIN users_projects up ON p.id = up.project_id
                WHERE up.user_id = ?1
                ",
            )
            .unwrap()
            .query_map([user_id], |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    created_at: {
                        let date: String = row.get(2)?;
                        DateTime::parse_from_rfc3339(&date)
                            .expect("Timezones in db should be rfc3339!")
                            .with_timezone(&Utc)
                    },
                })
            })
            .unwrap()
            .map(|f| f.unwrap())
            .collect();

        Ok(projects)
    }

    async fn get_profile_by_email(&self, email: String) -> Result<Option<UserProfile>> {
        let profile = self
            .conn
            .get()?
            .query_row(
                "SELECT id, name, email, created_at FROM users WHERE email = ?1",
                [email],
                |row| {
                    Ok(UserProfile {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        email: row.get(2)?,
                        created_at: {
                            let date: String = row.get(3)?;
                            DateTime::parse_from_rfc3339(&date)
                                .expect("Timezones in db should be rfc3339!")
                                .with_timezone(&Utc)
                        },
                    })
                },
            )
            .optional()?;

        Ok(profile)
    }
}
