use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use eyre::Result;
use sqlx::{query, query_as, PgPool};

use crate::{
    common::new_nanoid,
    project::model::Project,
    users::model::{NewUserProfile, UserProfile},
};

use super::UserRepository;

pub struct UserRepositoryPostgres {
    conn: Arc<PgPool>,
}

impl UserRepositoryPostgres {
    pub fn new(conn: Arc<PgPool>) -> UserRepositoryPostgres {
        UserRepositoryPostgres { conn }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryPostgres {
    async fn create_profile(&self, new_profile: NewUserProfile) -> Result<UserProfile> {
        let mut conn = self.conn.acquire().await.unwrap();

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

        let _created = query!(
            "
            INSERT INTO users (id, name, email, password_hash, created_at)
            VALUES ($1, $2, $3, $4, $5);",
            &profile.id,
            &profile.name,
            &profile.email,
            &password_hash,
            &profile.created_at
        )
        .execute(&mut conn)
        .await?;

        Ok(profile)
    }
    async fn validate_profile_credentials(
        &self,
        email: String,
        password: String,
    ) -> Result<Option<UserProfile>> {
        let mut conn = self.conn.acquire().await.unwrap();

        let profile_and_hash = query!(
            "SELECT id, name, email, created_at, password_hash FROM users WHERE email = $1",
            &email
        )
        .fetch_optional(&mut conn)
        .await?
        .map(|r| {
            (
                UserProfile {
                    id: r.id,
                    name: r.name,
                    email: r.email,
                    created_at: r.created_at,
                },
                r.password_hash,
            )
        });

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
        let mut conn = self.conn.acquire().await.unwrap();
        let profile = query_as!(
            UserProfile,
            "SELECT id, name, email, created_at FROM users WHERE email = $1",
            &user_id
        )
        .fetch_optional(&mut conn)
        .await?;

        Ok(profile)
    }

    async fn get_projects_of_user(&self, user_id: String) -> Result<Vec<Project>> {
        let mut conn = self.conn.acquire().await.unwrap();

        let projects = query_as!(
            Project,
            "
                SELECT p.id, p.name, p.created_at
                FROM projects p
                JOIN users_projects up ON p.id = up.project_id
                WHERE up.user_id = $1
                ",
            &user_id
        )
        .fetch_all(&mut conn)
        .await?;

        Ok(projects)
    }

    async fn get_profile_by_email(&self, email: String) -> Result<Option<UserProfile>> {
        let mut conn = self.conn.acquire().await.unwrap();

        let profile = query_as!(
            UserProfile,
            "SELECT id, name, email, created_at FROM users WHERE email = $1",
            &email
        )
        .fetch_optional(&mut conn)
        .await?;

        Ok(profile)
    }
}
