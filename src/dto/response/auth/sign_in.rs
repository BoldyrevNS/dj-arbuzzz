use serde::Serialize;

#[derive(Serialize)]
pub struct SignInResponse {
    pub token: String,
}
