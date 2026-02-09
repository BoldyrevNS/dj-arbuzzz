use std::sync::Arc;

use utoipa_axum::{router::OpenApiRouter, routes};

use crate::AppState;

pub mod sign_up;
pub fn router(app_state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new().nest("/auth/sign_up", sign_up_router(app_state.clone()))
}

fn sign_up_router(app_state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(sign_up::start))
        .routes(routes!(sign_up::verify))
        .routes(routes!(sign_up::resend))
        .routes(routes!(sign_up::complete))
        .with_state(app_state)
}
