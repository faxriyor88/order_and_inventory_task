use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Order {
    pub id: i64,
    pub total_price: Option<Decimal>,
    pub comment: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateOrder {
    pub comment: Option<String>,
}
