use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use eyre::Result;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OptionalExtension;

use crate::common::new_nanoid;

use super::model::{NewProject, Project};

pub type ProjectRepositoryDyn = Arc<dyn ProjectRepository + Send + Sync>;

/// `ProjectRepository` is abstracted to a trait to allow for using a seperate `ProjectRepository` in tests
#[async_trait]
pub trait ProjectRepository {
    async fn create_project(&self, new_project: &NewProject) -> Result<Project>;
    async fn get_project(&self, id: &String) -> Result<Option<Project>>;
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

    async fn get_project(&self, project_id: &String) -> Result<Option<Project>> {
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
}
