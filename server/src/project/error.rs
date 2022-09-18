use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("Could not add user to project because user does not exist")]
    UserNonExistent,
    #[error("Could not add user to project because user is already part of project")]
    UserAlreadyPartOfProject,
    #[error("Could not from project because user is already not part of project")]
    UserNotPartOfProject,
    #[error("Could not add user to project because project does not exist")]
    ProjectNonExistent,
    #[error("Unknown error")]
    Unknown,
}
