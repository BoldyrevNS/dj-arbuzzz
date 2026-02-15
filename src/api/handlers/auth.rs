use std::sync::Arc;

use axum::extract::State;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState,
    dto::{
        request::auth::auth::{RefreshTokenRequest, SignInRequest},
        response::{
            ApiResponse, ApiResult, ValidatedJSON,
            auth::auth::{RefreshTokenResponse, SignInResponse},
        },
    },
};

pub fn auth_router(app_state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(sign_in))
        .routes(routes!(refresh_token))
        .with_state(app_state)
}

#[utoipa::path(
        post,
        path = "/sign-in",
        tag = "Auth",
        request_body = SignInRequest,
        responses(
            (status = 200, description = "Success auth", body = SignInResponse),
            (status = 400, description = "Bad Request"),
            (status = 500, description = "Internal Server Error")
        )
    )]
async fn sign_in(
    State(state): State<Arc<AppState>>,
    ValidatedJSON(payload): ValidatedJSON<SignInRequest>,
) -> ApiResult<SignInResponse> {
    let res = state.services.auth_service.sign_in(payload).await?;
    Ok(ApiResponse::OK(res))
}

#[utoipa::path(
        post,
        path = "/refresh-token",
        tag = "Auth",
        request_body = RefreshTokenRequest,
        responses(
            (status = 200, description = "Success refresh token", body = RefreshTokenResponse),
            (status = 400, description = "Bad Request"),
            (status = 500, description = "Internal Server Error")
        )
    )]
async fn refresh_token(
    State(state): State<Arc<AppState>>,
    ValidatedJSON(payload): ValidatedJSON<RefreshTokenRequest>,
) -> ApiResult<RefreshTokenResponse> {
    let res = state.services.auth_service.refresh(payload).await?;
    Ok(ApiResponse::OK(res))
}
