use std::sync::Arc;

use argon2::{Argon2, PasswordVerifier, password_hash};
use jsonwebtoken::get_current_timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    dto::{
        request::auth::auth::{RefreshTokenRequest, SignInRequest},
        response::auth::auth::{RefreshTokenResponse, SignInResponse},
    },
    error::app_error::{AppError, AppResult, ErrorCode},
    infrastucture::repositories::users_repository::UsersRepository,
    service::token_service::{Token, TokenService, TokenType},
};

const ACCESS_TOKEN_EXPIRATION: u64 = 60 * 60;
const REFRESH_TOKEN_EXPIRATION: u64 = 60 * 60 * 24 * 7;

#[derive(Serialize, Deserialize)]
struct AccessTokenData {
    user_id: i32,
    exp: u64,
    token_type: TokenType,
}

impl Token for AccessTokenData {
    fn exp(&self) -> u64 {
        self.exp
    }
    fn token_type(&self) -> TokenType {
        self.token_type.clone()
    }
}

#[derive(Serialize, Deserialize)]
struct RefreshTokenData {
    user_id: i32,
    exp: u64,
    token_type: TokenType,
}

impl Token for RefreshTokenData {
    fn exp(&self) -> u64 {
        self.exp
    }
    fn token_type(&self) -> TokenType {
        self.token_type.clone()
    }
}

pub struct AuthService {
    token_service: Arc<TokenService>,
    users_repository: Arc<UsersRepository>,
}

impl AuthService {
    pub fn new(token_service: Arc<TokenService>, users_repository: Arc<UsersRepository>) -> Self {
        AuthService {
            token_service,
            users_repository,
        }
    }

    pub async fn sign_in(&self, payload: SignInRequest) -> AppResult<SignInResponse> {
        let user = self
            .users_repository
            .get_user_by_email(&payload.email)
            .await?;
        self.verify_password(&payload.password, &user.password)?;
        let (access_token, refresh_token) = self.create_tokens(user.id)?;
        Ok(SignInResponse {
            access_token,
            refresh_token,
        })
    }

    pub async fn refresh(&self, payload: RefreshTokenRequest) -> AppResult<RefreshTokenResponse> {
        let claims = match self
            .token_service
            .get_claims_from_jwt::<RefreshTokenData>(&payload.refresh_token, TokenType::Refresh)
        {
            Ok(claims) => claims,
            Err(e) => {
                return Err(AppError::Unauthorized(
                    "Invalid token".to_string(),
                    Some(ErrorCode::JWTInvalid),
                ));
            }
        };
        if self
            .token_service
            .is_token_expired::<RefreshTokenData>(&payload.refresh_token, TokenType::Refresh)?
        {
            return Err(AppError::Unauthorized(
                "Token expired".to_string(),
                Some(ErrorCode::JWTExpired),
            ));
        }
        let (access_token, refresh_token) = self.create_tokens(claims.user_id)?;

        Ok(RefreshTokenResponse {
            access_token,
            refresh_token,
        })
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

    fn create_tokens(&self, user_id: i32) -> AppResult<(String, String)> {
        let access_token = self.token_service.create_jwt(AccessTokenData {
            user_id,
            exp: get_current_timestamp() + ACCESS_TOKEN_EXPIRATION,
            token_type: TokenType::Access,
        })?;

        let refresh_token = self.token_service.create_jwt(RefreshTokenData {
            user_id,
            exp: get_current_timestamp() + REFRESH_TOKEN_EXPIRATION,
            token_type: TokenType::Refresh,
        })?;
        Ok((access_token, refresh_token))
    }
}
