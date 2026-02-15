#[derive(serde::Deserialize, utoipa::ToSchema, validator::Validate)]
pub struct SignInRequest {
    #[validate(email(message = "Некорректный email"))]
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}
