use crate::models::order::{CreateOrder, Order};
use sqlx::PgPool;

pub struct OrderRepository {
    pool: PgPool,
}

impl OrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateOrder) -> Result<Order, sqlx::Error> {
        sqlx::query_as::<_, Order>(
            r#"
                INSERT INTO orders (comment)
                VALUES ($1)
                RETURNING id, total_price, comment
                "#,
        )
        .bind(input.comment)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<Order>, sqlx::Error> {
        sqlx::query_as::<_, Order>(r#"SELECT * FROM orders WHERE id=$1"#)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn recalculate_total_price(&self, id: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"UPDATE orders
                   SET total_price = (
                       SELECT sum(line_total)
                       FROM order_item
                       WHERE order_id = $1)
                   WHERE id=$1"#,
        )
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() == 1)
    }
}
