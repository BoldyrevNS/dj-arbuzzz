use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use deadpool::managed::BuildError;
use diesel_async::pooled_connection::deadpool::PoolError;
use redis::RedisError;
use serde_json::json;
use thiserror::Error;
#[derive(Debug)]
pub enum ErrorCode {
    Unknown,
    OTPResendFailed,
    WrongOTP,
    OTPExpired,
    WrongOTPHash,
    UserAlreadyExists,
}

#[derive(serde::Serialize)]
struct AppErrorResponse {
    message: String,
    code: u16,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String, Option<ErrorCode>),

    #[error("Not found: {0}")]
    NotFound(String, Option<ErrorCode>),

    #[error("Validation error: {0}")]
    Validation(String, Option<ErrorCode>),

    #[error("Unauthorized: {0}")]
    Unauthorized(String, Option<ErrorCode>),

    #[error("Bad request: {0}")]
    BadRequest(String, Option<ErrorCode>),

    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
}

pub type AppResult<T> = Result<T, AppError>;

fn kek() -> AppResult<()> {
    if 1 > 0 {
        Ok(())
    } else {
        Err(AppError::Internal(anyhow::anyhow!("Error occurred")))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg, code) = match self {
            AppError::Database(msg, code) => (StatusCode::INTERNAL_SERVER_ERROR, msg, code),
            AppError::NotFound(msg, code) => (StatusCode::NOT_FOUND, msg, code),
            AppError::Validation(msg, code) => (StatusCode::BAD_REQUEST, msg, code),
            AppError::Unauthorized(msg, code) => (StatusCode::UNAUTHORIZED, msg, code),
            AppError::BadRequest(msg, code) => (StatusCode::BAD_REQUEST, msg, code),
            AppError::Internal(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", err),
                None,
            ),
        };
        let code_number = match code {
            Some(ErrorCode::OTPResendFailed) => 1001,
            Some(ErrorCode::WrongOTP) => 1002,
            Some(ErrorCode::WrongOTPHash) => 1003,
            Some(ErrorCode::OTPExpired) => 1004,
            Some(ErrorCode::UserAlreadyExists) => 1101,
            Some(ErrorCode::Unknown) => 1000,
            None => 1000,
        };
        let body = Json(json!({
            "status": status.as_u16(),
            "error": AppErrorResponse {
                message: msg,
                code: code_number,
            },
        }));
        (status, body).into_response()
    }
}

impl From<RedisError> for AppError {
    fn from(value: RedisError) -> Self {
        AppError::Internal(anyhow::anyhow!(value.to_string()))
    }
}

impl From<BuildError> for AppError {
    fn from(value: BuildError) -> Self {
        AppError::Internal(anyhow::anyhow!(value.to_string()))
    }
}

impl From<PoolError> for AppError {
    fn from(value: PoolError) -> Self {
        AppError::Internal(anyhow::anyhow!(value.to_string()))
    }
}
