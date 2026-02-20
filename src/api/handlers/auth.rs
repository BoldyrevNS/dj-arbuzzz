use std::sync::Arc;

use axum::extract::State;
use tower_cookies::Cookies;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState,
    dto::{
        request::auth::auth::SignInRequest,
        response::{ApiResponse, ApiResult, ValidatedJSON},
    },
};

pub fn auth_router(app_state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(sign_in))
        .with_state(app_state)
}

#[utoipa::path(
        post,
        path = "/sign-in",
        tag = "Auth",
        request_body = SignInRequest,
        responses(
            (status = 200, description = "Success auth"),
            (status = 400, description = "Bad Request"),
            (status = 500, description = "Internal Server Error")
        )
    )]
async fn sign_in(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    ValidatedJSON(payload): ValidatedJSON<SignInRequest>,
) -> ApiResult<()> {
    let res = state
        .services
        .auth_service
        .sign_in(payload, cookies)
        .await?;
    Ok(ApiResponse::OK(res))
}
