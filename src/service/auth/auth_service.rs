use std::sync::Arc;

use argon2::{Argon2, PasswordVerifier, password_hash};
use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};
use redis::AsyncTypedCommands;
use tower_cookies::{Cookie, Cookies};

const COOKIE_LIFETIME_SEC: i64 = 60 * 60 * 24 * 7;

use crate::{
    dto::request::auth::auth::SignInRequest,
    error::app_error::{AppError, AppResult, ErrorCode},
    infrastucture::{cache::client::Cache, repositories::users_repository::UsersRepository},
};

pub struct CookieData {
    session_id: String,
    username: String,
}

pub enum CacheKey {
    SESSION(String),
}

impl CacheKey {
    fn build_key(&self) -> String {
        match self {
            CacheKey::SESSION(session_id) => format!("AUTH_SESSION_{}", session_id),
        }
    }
}

pub struct AuthService {
    cache: Arc<Cache>,
    users_repository: Arc<UsersRepository>,
}

impl AuthService {
    pub fn new(cache: Arc<Cache>, users_repository: Arc<UsersRepository>) -> Self {
        AuthService {
            cache,
            users_repository,
        }
    }

    pub async fn sign_in(&self, payload: SignInRequest, cookies: Cookies) -> AppResult<()> {
        let user = self
            .users_repository
            .get_user_by_email(&payload.email)
            .await?;
        self.verify_password(&payload.password, &user.password)?;
        self.create_session(cookies, user.id).await?;
        Ok(())
    }

    fn verify_password(&self, password: &str, hash: &str) -> AppResult<()> {
        let parsed_hash = match password_hash::PasswordHash::new(hash) {
            Ok(hash) => hash,
            Err(_) => {
                return Err(AppError::Unauthorized(
                    "Wrong password".to_string(),
                    Some(ErrorCode::SignUpFailed),
                ));
            }
        };
        match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => (),
            Err(_) => {
                return Err(AppError::Unauthorized(
                    "Wrong password".to_string(),
                    Some(ErrorCode::SignUpFailed),
                ));
            }
        };
        Ok(())
    }

    async fn create_session(&self, cookies: Cookies, user_id: i32) -> AppResult<()> {
        let session_id: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        let mut cache_con = self.cache.get_async_conn().await?;
        let cache_key = CacheKey::SESSION(session_id.clone()).build_key();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        let _: () = cache_con
            .hset_multiple(
                &cache_key,
                &[
                    ("session_id", session_id.clone()),
                    ("created_at", timestamp.clone()),
                    ("last_updated", timestamp),
                    ("user_id", user_id.to_string()),
                ],
            )
            .await?;

        let _ = cache_con.expire(&cache_key, COOKIE_LIFETIME_SEC).await?;

        let mut cookie = Cookie::new("x-authenticated", session_id);
        cookie.set_secure(true);
        cookie.set_http_only(true);
        cookie.set_path("/");
        cookies.add(cookie);
        Ok(())
    }
}
