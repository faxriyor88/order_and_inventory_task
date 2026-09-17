use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum AuthError {
    MissingToken,
    InvalidToken,
    InvalidCredentials,
    TokenCreation,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::MissingToken | AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "unauthorized"),
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "invalid credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "internal server"),
        }.into_response()
    }
}