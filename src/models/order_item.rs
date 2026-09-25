use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct OrderItem {
    pub id: i64,
    pub order_id: i64,
    pub product_id: i64,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub line_total: Decimal,
}

#[derive(Deserialize)]
pub struct CreateOrderItem {
    pub product_id: i64,
    pub quantity: i32,
}

pub struct InsertOrderItem {
    pub order_id: i64,
    pub product_id: i64,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub line_total: Decimal,
}
