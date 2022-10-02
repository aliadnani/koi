use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use eyre::Result;
use sqlx::{query, query_as, PgPool};

use crate::{
    common::new_nanoid,
    project::{
        error::ProjectError,
        model::{NewProject, Project},
    },
    users::model::UserProfile,
};

use super::ProjectRepository;

pub struct ProjectRepositoryPostgres {
    conn: Arc<PgPool>,
}

impl ProjectRepositoryPostgres {
    pub fn new(conn: Arc<PgPool>) -> ProjectRepositoryPostgres {
        ProjectRepositoryPostgres { conn }
    }
}

#[async_trait]
impl ProjectRepository for ProjectRepositoryPostgres {
    async fn create_project(&self, new_project: &NewProject) -> Result<Project> {
        let mut conn = self.conn.acquire().await?;

        let project = Project {
            id: new_nanoid(),
            name: new_project.name.clone(),
            created_at: Utc::now(),
        };

        query!(
            "INSERT INTO projects (id, name, created_at) VALUES ($1, $2, $3);",
            &project.id,
            &project.name,
            &project.created_at
        )
        .execute(&mut conn)
        .await?;

        Ok(project)
    }

    async fn get_project(&self, project_id: String) -> Result<Option<Project>> {
        let mut conn = self.conn.acquire().await?;

        let project = query_as!(
            Project,
            "SELECT id, name, created_at FROM projects WHERE id = $1",
            project_id
        )
        .fetch_optional(&mut conn)
        .await?;

        Ok(project)
    }

    // TODO: Check project_id & user_id must exist constraint
    async fn add_user_to_project(
        &self,
        project_id: String,
        user_email: String,
    ) -> Result<(), ProjectError> {
        let mut conn = self.conn.acquire().await.unwrap();

        let user_id = match query!("SELECT id FROM users WHERE email = $1", &user_email)
            .fetch_optional(&mut conn)
            .await
            .map_err(|_| ProjectError::Unknown)?
        {
            Some(r) => r.id,
            None => return Err(ProjectError::UserNonExistent.into()),
        };

        let _project_exists = match query!("SELECT id FROM projects WHERE id = $1", &project_id)
            .fetch_optional(&mut conn)
            .await
            .map_err(|_| ProjectError::Unknown)?
        {
            Some(r) => r.id,
            None => return Err(ProjectError::UserNonExistent.into()),
        };

        match query!(
            "INSERT INTO users_projects (user_id, project_id) VALUES ($1, $2);",
            &user_id,
            &project_id
        )
        .execute(&mut conn)
        .await
        .map_err(|_| ProjectError::Unknown)?
        .rows_affected()
        {
            0 => return Err(ProjectError::UserAlreadyPartOfProject.into()),
            _ => return Ok(()),
        };
    }

    async fn remove_user_from_project(
        &self,
        project_id: String,
        user_email: String,
    ) -> Result<(), ProjectError> {
        let mut conn = self.conn.acquire().await.unwrap();

        let user_id = match query!("SELECT id FROM users WHERE email = $1", &user_email)
            .fetch_optional(&mut conn)
            .await
            .map_err(|_| ProjectError::Unknown)?
        {
            Some(r) => r.id,
            None => return Err(ProjectError::UserNonExistent.into()),
        };

        let _project_exists = match query!("SELECT id FROM projects WHERE id = $1", &project_id)
            .fetch_optional(&mut conn)
            .await
            .map_err(|_| ProjectError::Unknown)?
        {
            Some(r) => r.id,
            None => return Err(ProjectError::UserNonExistent.into()),
        };

        match query!(
            "DELETE FROM users_projects WHERE user_id = $1 AND project_id = $2;",
            &user_id,
            &project_id
        )
        .execute(&mut conn)
        .await
        .map_err(|_| ProjectError::Unknown)?
        .rows_affected()
        {
            0 => return Err(ProjectError::UserNotPartOfProject.into()),
            _ => return Ok(()),
        };
    }

    async fn list_users_of_project(&self, project_id: String) -> Result<Vec<UserProfile>> {
        let mut conn = self.conn.acquire().await.unwrap();

        let profiles = query_as!(
            UserProfile,
            "
                SELECT u.id, u.name, u.email, u.created_at
                FROM users u
                JOIN users_projects up ON u.id = up.user_id
                WHERE up.project_id = $1
                ",
            &project_id
        )
        .fetch_all(&mut conn)
        .await
        .map_err(|_| ProjectError::Unknown)?;

        Ok(profiles)
    }

    async fn check_if_user_is_part_of_project(
        &self,
        project_id: String,
        email: String,
    ) -> Result<bool> {
        let mut conn = self.conn.acquire().await.unwrap();
        match query!(
            "
                SELECT u.id
                FROM users u
                JOIN users_projects up ON u.id = up.user_id
                WHERE up.project_id = $1 AND u.email = $2;
                ",
            project_id,
            email
        )
        .fetch_optional(&mut conn)
        .await
        .map_err(|_| ProjectError::Unknown)?
        {
            Some(_) => return Ok(true),
            None => return Ok(false),
        };
    }
}
