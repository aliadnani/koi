use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

use crate::feedback;
use crate::project;
use crate::sessions;
use crate::users;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Users
        users::service::log_in,
        users::service::register,
        users::service::get_profile,
        // Projects
        project::service::create_project,
        project::service::get_project,
        project::service::get_project_feedback,
        project::service::add_user_to_project,
        project::service::remove_user_from_project,
        project::service::list_users_of_project,
        // Feedback
        feedback::service::create_feedback,

    ),
    components(
        schemas(
            // Session
            sessions::Session,
            // Users
            users::model::UserProfile,
            users::model::NewUserProfile,
            users::model::UserProfileWithProjects,
            users::model::UserProjectAdditionRemoval,
            // Projects
            project::model::Project,
            project::model::NewProject,
            // Feedback
            feedback::model::NewFeedback,
            feedback::model::Feedback,
            feedback::model::FeedbackCategory,
            feedback::model::FeedbackMetadata,
            feedback::model::FeedbackStatus,
        )
    ),
    security(
        ("Session Token" = [])
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Profile API", description = "For profile self-service"),
        (name = "Project API", description = "For project-related operations"),
        (name = "Feedback API", description = "Mainly for posting new feedback"),
    )
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "Username & Password",
                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Basic).build()),
            );
            components.add_security_scheme(
                "Session Token",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("Opaque")
                        .build(),
                ),
            );
        }
    }
}
