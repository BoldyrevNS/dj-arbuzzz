#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct SignInResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}
