use poem_openapi::Tags;

pub mod auth;
pub mod user;

#[derive(Tags)]
enum ApiTags {
    /// Operations about auth
    Auth,
    User,
}

