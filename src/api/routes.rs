use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use super::{AppState, handlers};
use axum::Router;

pub fn create_router(state: AppState) -> Router {
    let state = Arc::new(state);
    let (router, api) = OpenApiRouter::new()
        .nest("/api/v1", handlers::auth::router(state.clone()))
        .split_for_parts();
    let router = router.merge(SwaggerUi::new("/api-docs").url("/api-docs/openapi.json", api));
    router
}
