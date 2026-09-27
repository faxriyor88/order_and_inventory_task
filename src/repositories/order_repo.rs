use crate::models::order::{CreateOrder, Order};
use sqlx::{Executor, PgPool, Postgres};

pub struct OrderRepository {
    pool: PgPool,
}

impl OrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_with<'e, E>(executor: E, input: CreateOrder) -> Result<Order, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as::<_, Order>(
            r#"
                INSERT INTO orders (comment)
                VALUES ($1)
                RETURNING id, total_price, comment
                "#,
        )
        .bind(input.comment)
        .fetch_one(executor)
        .await
    }
    pub async fn create(&self, input: CreateOrder) -> Result<Order, sqlx::Error> {
        Self::create_with(&self.pool, input).await
    }

    pub async fn find_by_id_with<'e, E>(executor: E, id: i64) -> Result<Option<Order>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as::<_, Order>(r#"SELECT * FROM orders WHERE id=$1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }
    pub async fn find_by_id(&self, id: i64) -> Result<Option<Order>, sqlx::Error> {
        Self::find_by_id_with(&self.pool, id).await
    }

    pub async fn list(&self) -> Result<Vec<Order>, sqlx::Error> {
        sqlx::query_as::<_, Order>(r#"SELECT * FROM orders"#)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn recalculate_total_price_with<'e, E>(executor: E, id: i64) -> Result<bool, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let result = sqlx::query(
            r#"UPDATE orders
                   SET total_price = (
                       SELECT sum(line_total)
                       FROM order_item
                       WHERE order_id = $1)
                   WHERE id=$1"#,
        )
        .bind(id)
        .execute(executor)
        .await?;

        Ok(result.rows_affected() == 1)
    }
    pub async fn recalculate_total_price(&self, id: i64) -> Result<bool, sqlx::Error> {
        Self::recalculate_total_price_with(&self.pool, id).await
    }
}
