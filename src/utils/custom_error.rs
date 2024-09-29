use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct CustomError {
    pub message: String,
    #[serde(skip)]
    pub status_code: StatusCode,
}

impl CustomError {
    pub fn new(message: impl Into<String>, status_code: StatusCode) -> Self {
        CustomError {
            message: message.into(),
            status_code,
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CustomError {}

impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        (
            self.status_code,
            Json(self)
        ).into_response()
    }
}