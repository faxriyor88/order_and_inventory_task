use crate::AppState;
use axum::Router;

pub mod auth;
pub mod order;
pub mod product;

pub fn public_routes() -> Router<AppState> {
    Router::new().merge(auth::auth_routes())
}

pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .merge(product::product_routes())
        .merge(order::order_routes())
}
