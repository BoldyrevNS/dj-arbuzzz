use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;

use crate::error::app_error::AppError;

pub mod auth;
#[derive(Serialize)]
struct Res<T: Serialize> {
    status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

pub enum ApiResponse<T: Serialize> {
    OK(T),
    CREATED(T),
}

pub type ApiResult<T> = Result<ApiResponse<T>, AppError>;

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let (status, data) = match self {
            Self::OK(data) => (StatusCode::OK, data),
            Self::CREATED(data) => (StatusCode::CREATED, data),
        };
        let body = Json(json!(Res {
            status: status.as_u16(),
            data: Some(data)
        }));
        (status, body).into_response()
    }
}
