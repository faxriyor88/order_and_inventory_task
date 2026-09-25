use crate::auth::middleware::AuthError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sqlx::Error;

pub enum AppError {
    Auth(AuthError),
    Product(ProductError),
    Order(OrderError),
    Validation(ValidationError),
    Database(Error),
    Internal,
}

pub enum ProductError {
    NotFound
}

impl From<ProductError> for AppError {
    fn from(err: ProductError) -> Self {
        AppError::Product(err)
    }
}

pub enum OrderError {
    NotFound,
    ProductPriceMissing
}

impl From<OrderError> for AppError {
    fn from(err: OrderError) -> Self {
        AppError::Order(err)
    }
}

pub enum ValidationError {
    EmptyField(&'static str),
    MustBePositive(&'static str),
}

impl From<ValidationError> for AppError {
    fn from(err: ValidationError) -> Self {
        AppError::Validation(err)
    }
}

impl From<Error> for AppError {
    fn from(err: Error) -> Self {
        AppError::Database(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Auth(AuthError::MissingToken) | AppError::Auth(AuthError::InvalidToken) => {
                (StatusCode::UNAUTHORIZED, "unauthorized".to_string())
            }
            AppError::Auth(AuthError::InvalidCredentials) => {
                (StatusCode::UNAUTHORIZED, "invalid credentials".to_string())
            }
            AppError::Auth(AuthError::TokenCreation) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server".to_string(),
            ),
            AppError::Product(ProductError::NotFound) => {
                (StatusCode::BAD_REQUEST, "product not found".to_string())
            },
            AppError::Order(OrderError::NotFound) => {
                (StatusCode::NOT_FOUND, "order not found".to_string())
            },
            AppError::Order(OrderError::ProductPriceMissing) => {
                (StatusCode::NOT_FOUND, "product price missing".to_string())
            },
            AppError::Validation(ValidationError::EmptyField(field)) => (
                StatusCode::BAD_REQUEST,
                format!("Field '{}' cannot be empty", field),
            ),
            AppError::Validation(ValidationError::MustBePositive(field)) => (
                StatusCode::BAD_REQUEST,
                format!("Field '{}' must be positive", field)
            ),
            AppError::Database(err) => {
                eprintln!("Database error: {err:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        }
        .into_response()
    }
}
