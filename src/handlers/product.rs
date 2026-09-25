use crate::AppState;
use crate::errors::AppError;
use crate::models::product::{CreateProduct, Product};
use crate::services::product_service::ProductService;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use std::sync::Arc;

pub fn product_routes() -> Router<AppState> {
    Router::new().route("/product", post(create))
}

async fn create(
    State(product_service): State<Arc<ProductService>>,
    Json(payload): Json<CreateProduct>,
) -> Result<Json<Product>, AppError> {
    let product = product_service.create(payload).await?;

    Ok(Json(product))
}
