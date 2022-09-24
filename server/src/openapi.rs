use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

use crate::sessions;
use crate::users;

#[derive(OpenApi)]
#[openapi(
    paths(
        users::service::log_in,
        users::service::register,
        users::service::get_profile
    ),
    components(
        schemas(
            sessions::Session,
            users::model::UserProfile,
            users::model::NewUserProfile
        )
    ),
    security(
        ("Session Token" = [])
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Profile", description = "API for managing your own profile")
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
