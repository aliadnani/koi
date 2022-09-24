use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use eyre::Result;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OptionalExtension;

use crate::{common::new_nanoid, users::model::UserProfile};

use super::{
    error::ProjectError,
    model::{NewProject, Project},
};

pub type ProjectRepositoryDyn = Arc<dyn ProjectRepository + Send + Sync>;

/// `ProjectRepository` is abstracted to a trait to allow for using a seperate `ProjectRepository` in tests
#[async_trait]
pub trait ProjectRepository {
    async fn create_project(&self, new_project: &NewProject) -> Result<Project>;
    async fn get_project(&self, id: String) -> Result<Option<Project>>;
    async fn add_user_to_project(
        &self,
        project_id: String,
        user_email: String,
    ) -> Result<(), ProjectError>;
    async fn remove_user_from_project(
        &self,
        project_id: String,
        user_email: String,
    ) -> Result<(), ProjectError>;
    async fn list_users_of_project(&self, project_id: String) -> Result<Vec<UserProfile>>;
    async fn check_if_user_is_part_of_project(
        &self,
        project_id: String,
        email: String,
    ) -> Result<bool>;
}

pub struct ProjectRepositorySqlite {
    conn: Arc<Pool<SqliteConnectionManager>>,
}

impl ProjectRepositorySqlite {
    pub fn new(conn: Arc<Pool<SqliteConnectionManager>>) -> ProjectRepositorySqlite {
        ProjectRepositorySqlite { conn }
    }
}

#[async_trait]
impl ProjectRepository for ProjectRepositorySqlite {
    async fn create_project(&self, new_project: &NewProject) -> Result<Project> {
        let project = Project {
            id: new_nanoid(),
            name: new_project.name.clone(),
            created_at: Utc::now(),
        };

        let _created = self.conn.get()?.execute(
            "INSERT INTO projects (id, name, created_at) VALUES (?1, ?2, ?3);",
            (&project.id, &project.name, &project.created_at.to_rfc3339()),
        )?;

        Ok(project)
    }

    async fn get_project(&self, project_id: String) -> Result<Option<Project>> {
        let project = self
            .conn
            .get()?
            .query_row(
                "SELECT id, name, created_at FROM projects WHERE id = ?1",
                [project_id],
                |row| {
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
                },
            )
            .optional()?;

        Ok(project)
    }

    // TODO: Check project_id & user_id must exist constraint
    async fn add_user_to_project(
        &self,
        project_id: String,
        user_email: String,
    ) -> Result<(), ProjectError> {
        let user_id: String = match self
            .conn
            .get()
            .map_err(|_| ProjectError::Unknown)?
            .query_row(
                "SELECT id FROM users WHERE email = ?1",
                [user_email],
                |row| Ok(row.get(0)?),
            )
            .optional()
            .map_err(|_| ProjectError::Unknown)?
        {
            Some(user_id) => user_id,
            None => return Err(ProjectError::UserNonExistent.into()),
        };

        let _project_exists: String = match self
            .conn
            .get()
            .map_err(|_| ProjectError::Unknown)?
            .query_row(
                "SELECT id FROM projects WHERE id = ?1",
                [&project_id],
                |row| Ok(row.get(0)?),
            )
            .optional()
            .map_err(|_| ProjectError::Unknown)?
        {
            Some(project_id) => project_id,
            None => return Err(ProjectError::ProjectNonExistent.into()),
        };

        let _association = match self
            .conn
            .get()
            .map_err(|_| ProjectError::Unknown)?
            .execute(
                "INSERT INTO users_projects (user_id, project_id) VALUES (?1, ?2);",
                (&user_id, &project_id),
            )
            .map_err(|_| ProjectError::Unknown)?
        {
            0 => return Err(ProjectError::UserNotPartOfProject.into()),
            _ => return Ok(()),
        };
    }

    async fn remove_user_from_project(
        &self,
        project_id: String,
        user_email: String,
    ) -> Result<(), ProjectError> {
        let user_id: String = match self
            .conn
            .get()
            .map_err(|_| ProjectError::Unknown)?
            .query_row(
                "SELECT id FROM users WHERE email = ?1",
                [user_email],
                |row| Ok(row.get(0)?),
            )
            .optional()
            .map_err(|_| ProjectError::Unknown)?
        {
            Some(user_id) => user_id,
            None => return Err(ProjectError::UserNonExistent.into()),
        };

        let _project_exists: String = match self
            .conn
            .get()
            .map_err(|_| ProjectError::Unknown)?
            .query_row(
                "SELECT id FROM projects WHERE id = ?1",
                [&project_id],
                |row| Ok(row.get(0)?),
            )
            .optional()
            .map_err(|_| ProjectError::Unknown)?
        {
            Some(project_id) => project_id,
            None => return Err(ProjectError::ProjectNonExistent.into()),
        };

        let _association = match self
            .conn
            .get()
            .map_err(|_| ProjectError::Unknown)?
            .execute(
                "DELETE FROM users_projects WHERE user_id = ?1 AND project_id = ?2;",
                (&user_id, &project_id),
            )
            .map_err(|_| ProjectError::Unknown)?
        {
            0 => return Err(ProjectError::UserNotPartOfProject.into()),
            _ => return Ok(()),
        };
    }

    async fn list_users_of_project(&self, project_id: String) -> Result<Vec<UserProfile>> {
        let profiles: Vec<UserProfile> = self
            .conn
            .get()?
            .prepare_cached(
                "
                SELECT u.id, u.name, u.email, u.created_at
                FROM users u
                JOIN users_projects up ON u.id = up.user_id
                WHERE up.project_id = ?1
                ",
            )
            .unwrap()
            .query_map([project_id], |row| {
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
            })
            .unwrap()
            .map(|f| f.unwrap())
            .collect();

        Ok(profiles)
    }

    async fn check_if_user_is_part_of_project(
        &self,
        project_id: String,
        email: String,
    ) -> Result<bool> {
        let user_is_part_of_project = self
            .conn
            .get()?
            .prepare_cached(
                "
                SELECT u.id
                FROM users u
                JOIN users_projects up ON u.id = up.user_id
                WHERE up.project_id = ?1 AND u.email = ?2;
                ",
            )
            .unwrap()
            .query_row([project_id, email], |_r| Ok(true))
            .optional()
            .unwrap()
            .unwrap_or(false);

        Ok(user_is_part_of_project)
    }
}
