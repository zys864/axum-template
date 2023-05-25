use axum::http::header;
use axum::Json;
use axum::{
    body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;
pub type HttpResult<T> = std::result::Result<T, ErrorKind>;
#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("not be authorized")]
    Unauthorized,
    #[error("no such user or error password")]
    NoSuchUserOrErrorPassword,
    #[error(transparent)]
    TokenError(#[from] jsonwebtoken::errors::Error),
    #[error("Duplicated email: {}", 0)]
    DuplicatedEmail(String),
    #[error(transparent)]
    FiledValidate(#[from] validator::ValidationErrors),
    #[error(transparent)]
    SqlError(#[from] sqlx::Error),
    #[error(transparent)]
    EncripyError(#[from] argon2::Error),
}
impl IntoResponse for ErrorKind {
    fn into_response(self) -> Response {

        match self {
            ErrorKind::Unauthorized => {
                (StatusCode::UNAUTHORIZED, body::Empty::new()).into_response()
            }
            ErrorKind::NoSuchUserOrErrorPassword => {
                let errors_info = vec![ErrorKind::NoSuchUserOrErrorPassword.to_string()];
                let errors = ErrorResponse::new(errors_info);
                (
                    StatusCode::BAD_REQUEST,
                    [(
                        header::CONTENT_TYPE,
                        header::HeaderValue::from_str("application/json").unwrap(),
                    )],
                    Json(errors),
                )
                    .into_response()
            }
            ErrorKind::TokenError(e) => {
                let errors_info = match e.kind() {
                    jsonwebtoken::errors::ErrorKind::InvalidToken => "InvalidToken",
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => "ExpiredToken",
                    _ => "other token error",
                }
                .to_string();
                let errors = ErrorResponse::new(vec![errors_info]);
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    [(
                        header::CONTENT_TYPE,
                        header::HeaderValue::from_str("application/json").unwrap(),
                    )],
                    Json(errors),
                )
                    .into_response()
            }
            ErrorKind::DuplicatedEmail(s) => {
                let errors_info = vec![format!("Duplicated email: {}", s)];
                let errors = ErrorResponse::new(errors_info);
                (
                    StatusCode::BAD_REQUEST,
                    [(
                        header::CONTENT_TYPE,
                        header::HeaderValue::from_str("application/json").unwrap(),
                    )],
                    Json(errors),
                )
                    .into_response()
            }
            ErrorKind::FiledValidate(e) => {
                let errors_info: Vec<String> =
                    e.to_string().split('\n').map(|x| x.to_string()).collect();
                let errors = ErrorResponse::new(errors_info);
                (
                    StatusCode::BAD_REQUEST,
                    [(
                        header::CONTENT_TYPE,
                        header::HeaderValue::from_str("application/json").unwrap(),
                    )],
                    Json(errors),
                )
                    .into_response()
            }
            ErrorKind::SqlError(_) | ErrorKind::EncripyError(_) => {
                let errors_info = vec!["Internel server error".to_string()];
                let errors = ErrorResponse::new(errors_info);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    [(
                        header::CONTENT_TYPE,
                        header::HeaderValue::from_str("application/json").unwrap(),
                    )],
                    Json(errors),
                )
                    .into_response()
            }
        }
    }
}
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    errors: ErrorResponseBody,
}
#[derive(Debug, Serialize)]
pub struct ErrorResponseBody {
    body: Vec<String>,
}
impl ErrorResponse {
    pub fn new<T: AsRef<[String]>>(errors: T) -> Self {
        Self {
            errors: ErrorResponseBody {
                body: errors.as_ref().to_vec(),
            },
        }
    }
}
