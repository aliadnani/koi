use crate::project::model::Project;

pub struct Profile {
    id: String,
    name: String,
    email: String,
    projects: Vec<Project>,
}
