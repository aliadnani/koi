use std::sync::Arc;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite_migration::{Migrations, M};

pub fn migrations() -> Migrations<'static> {
    Migrations::new(vec![
        // Projects
        M::up(
            r#"
        CREATE TABLE projects (
            id TEXT PRIMARY KEY UNIQUE,
            name TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        CREATE INDEX projects_idx ON projects (id);
        "#,
        ),
        // Users
        M::up(
            r#"
        CREATE TABLE users (
            id TEXT PRIMARY KEY UNIQUE,
            name TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT UNIQUE NOT NULL,
            created_at TEXT NOT NULL
        );
        CREATE INDEX users_idx ON users (id, email);
        "#,
        ),
        // Users & Projects association
        M::up(
            r#"
        CREATE TABLE users_projects (
            user_id TEXT NOT NULL,
            project_id TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id),
            FOREIGN KEY (project_id) REFERENCES projects (id),
            UNIQUE(user_id, project_id)
        );
        CREATE INDEX users_projects_idx ON users_projects (user_id, project_id);
        "#,
        ),
        // Feedback
        M::up(
            r#"
        CREATE TABLE feedback (
            id TEXT PRIMARY KEY UNIQUE,
            description TEXT NOT NULL,
            location TEXT NOT NULL,
            status TEXT NOT NULL,
            category TEXT NOT NULL,
            metadata TEXT NOT NULL,
            additional_attributes TEXT NOT NULL,
            project_id TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects (id)
        );
        CREATE INDEX feedback_idx ON feedback (id, status, category, project_id);
        "#,
        ),
        // Sessions
        M::up(
            r#"
        CREATE TABLE user_sessions (
            token TEXT PRIMARY KEY UNIQUE,
            user_email TEXT NOT NULL,
            expires_at TEXT NOT NULL
        );
        "#,
        ),
    ])
}

pub fn start_up(pool: Arc<Pool<SqliteConnectionManager>>) {
    pool.get()
        .expect("Could not get a connection from SQLite pool.")
        .pragma_update(None, "journal_mode", &"WAL")
        .expect("Failed to set journal mode to WAL");

    pool.get()
        .expect("Could not get a connection from SQLite pool.")
        .pragma_update(None, "foreign_keys", &"ON")
        .expect("Failed to enable strict mode");

    pool.get()
        .expect("Could not get a connection from SQLite pool.")
        .pragma_update(None, "strict", &"ON")
        .expect("Failed to enable foreign keys");

    migrations()
        .to_latest(
            &mut pool
                .get()
                .expect("Could not get a connection from SQLite pool."),
        )
        .expect("Failed to run migrations");
}
