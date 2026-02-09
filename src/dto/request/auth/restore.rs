#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct StartRestoreRequest {
    pub email: String,
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct VerifyOTPRequest {
    pub email: String,
    pub hash: String,
    pub otp: u32,
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct CompleteRestoreRequest {
    pub email: String,
    pub password: String,
    pub hash: String,
}
