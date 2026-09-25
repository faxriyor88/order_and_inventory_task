use crate::AppState;
use crate::errors::AppError;
use crate::models::order::{CreateOrder, Order};
use crate::models::order_item::{CreateOrderItem, OrderItem};
use crate::services::order_service::OrderService;
use axum::extract::{Path, State};
use axum::routing::post;
use axum::{Json, Router};
use std::sync::Arc;

pub fn order_routes() -> Router<AppState> {
    Router::new()
        .route("/order", post(create))
        .route("/order/{id}/items", post(add_item))
}

async fn create(
    State(order_service): State<Arc<OrderService>>,
    Json(payload): Json<CreateOrder>,
) -> Result<Json<Order>, AppError> {
    let order = order_service.create(payload).await?;

    Ok(Json(order))
}

async fn add_item(
    State(order_service): State<Arc<OrderService>>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateOrderItem>,
) -> Result<Json<OrderItem>, AppError> {
    let order = order_service.add_item(id, payload).await?;

    Ok(Json(order))
}
