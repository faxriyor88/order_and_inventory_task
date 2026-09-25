use crate::models::order_item::{InsertOrderItem, OrderItem};
use sqlx::PgPool;

pub struct OrderItemRepository {
    pool: PgPool,
}

impl OrderItemRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: InsertOrderItem) -> Result<OrderItem, sqlx::Error> {
        sqlx::query_as::<_, OrderItem>(
            r#"
                INSERT INTO order_item (order_id, product_id, quantity, unit_price, line_total)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id, order_id, product_id, quantity, unit_price, line_total
                "#,
        )
        .bind(input.order_id)
        .bind(input.product_id)
        .bind(input.quantity)
        .bind(input.unit_price)
        .bind(input.line_total)
        .fetch_one(&self.pool)
        .await
    }
}