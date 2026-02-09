use serde::Deserialize;
#[derive(Deserialize)]
pub struct SignInStart {
    pub username: String,
    pub password: String,
}
