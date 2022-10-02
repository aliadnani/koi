
-- migrate:up
 
CREATE TABLE
  projects (
    id TEXT PRIMARY KEY UNIQUE,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
  );

CREATE INDEX projects_idx ON projects (id);

CREATE TABLE
  users (
    id TEXT PRIMARY KEY UNIQUE,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
  );

CREATE INDEX users_idx ON users (id, email);

CREATE TABLE
  users_projects (
    user_id TEXT NOT NULL,
    project_id TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (project_id) REFERENCES projects (id),
    UNIQUE (user_id, project_id)
  );

CREATE INDEX users_projects_idx ON users_projects (user_id, project_id);

CREATE TABLE
  feedback (
    id TEXT PRIMARY KEY UNIQUE,
    description TEXT NOT NULL,
    location TEXT NOT NULL,
    status TEXT NOT NULL,
    category TEXT NOT NULL,
    metadata JSONB NOT NULL,
    additional_attributes JSONB NOT NULL,
    project_id TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects (id)
  );

CREATE INDEX feedback_idx ON feedback (id, status, category, project_id);

CREATE TABLE
  user_sessions (
    token TEXT PRIMARY KEY UNIQUE,
    user_email TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL
  );

-- migrate:down