use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub price: Option<Decimal>,
    pub stock_quantity: Option<i32>,
}

#[derive(Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub price: Option<Decimal>,
    pub stock_quantity: Option<i32>
}