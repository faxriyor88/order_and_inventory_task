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
        .route("/orders", post(create).get(list_orders))
        .route("/orders/{id}/items", post(add_item).get(list_order_items))
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

async fn list_orders(
    State(order_service): State<Arc<OrderService>>,
) -> Result<Json<Vec<Order>>, AppError> {
    let list = order_service.list().await?;

    Ok(Json(list))
}

async fn list_order_items(
    State(order_service): State<Arc<OrderService>>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<OrderItem>>, AppError> {
    let list = order_service.list_items(id).await?;

    Ok(Json(list))
}