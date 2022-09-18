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
        "#,
        ),
        // Profiles
        M::up(
            r#"
        CREATE TABLE profiles (
            id TEXT PRIMARY KEY UNIQUE,
            name TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL,
            created_at TEXT NOT NULL
        );
        "#,
        ),
        // Profiles & Projects association
        M::up(
            r#"
        CREATE TABLE profiles_projects (
            profile_id TEXT NOT NULL,
            project_id TEXT NOT NULL
        );
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
            created_at TEXT NOT NULL
        );
        "#,
        ),
    ])
}
