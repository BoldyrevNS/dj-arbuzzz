#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct SignUpStartRequest {
    pub email: String,
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct VerifyOTPRequest {
    pub email: String,
    pub hash: String,
    pub otp: u32,
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct ResendOTPRequest {
    pub email: String,
    pub hash: String,
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct SignUpCompleteRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub hash: String,
}
