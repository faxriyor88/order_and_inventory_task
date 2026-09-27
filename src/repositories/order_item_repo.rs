use crate::models::order_item::{InsertOrderItem, OrderItem};
use sqlx::{Executor, PgPool, Postgres};

pub struct OrderItemRepository {
    pool: PgPool,
}

impl OrderItemRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_with<'e, E>(
        executor: E,
        input: InsertOrderItem,
    ) -> Result<OrderItem, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
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
        .fetch_one(executor)
        .await
    }
    pub async fn create(&self, input: InsertOrderItem) -> Result<OrderItem, sqlx::Error> {
        Self::create_with(&self.pool, input).await
    }

    pub async fn list_by_order_id(&self, order_id: i64) -> Result<Vec<OrderItem>, sqlx::Error> {
        sqlx::query_as::<_, OrderItem>(r#"SELECT * FROM order_item WHERE order_id=$1"#)
            .bind(order_id)
            .fetch_all(&self.pool)
            .await
    }
}