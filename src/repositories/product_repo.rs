use crate::models::product::{CreateProduct, Product};
use sqlx::{Executor, PgPool, Postgres};

pub struct ProductRepository {
    pool: PgPool,
}

impl ProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateProduct) -> Result<Product, sqlx::Error> {
        sqlx::query_as::<_, Product>(
            r#"
                INSERT INTO product (name, price, stock_quantity)
                VALUES ($1, $2, $3)
                RETURNING id, name, price, stock_quantity
                "#,
        )
        .bind(&input.name)
        .bind(input.price)
        .bind(input.stock_quantity)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_by_id_with<'e, E>(
        executor: E,
        id: i64,
    ) -> Result<Option<Product>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as::<_, Product>(r#"SELECT * FROM product WHERE id=$1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }
    pub async fn find_by_id(&self, id: i64) -> Result<Option<Product>, sqlx::Error> {
        Self::find_by_id_with(&self.pool, id).await
    }

    pub async fn decrease_stock_returning_with<'e, E>(
        executor: E,
        id: i64,
        target_stock: i32,
    ) -> Result<Option<Product>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let product = sqlx::query_as::<_, Product>(
            r#"
                UPDATE product
                SET stock_quantity = stock_quantity - $2
                WHERE id=$1
                RETURNING id, name, price, stock_quantity
                "#,
        )
        .bind(id)
        .bind(target_stock)
        .fetch_optional(executor)
        .await?;

        Ok(product)
    }
    pub async fn decrease_stock_returning(
        &self,
        id: i64,
        target_stock: i32,
    ) -> Result<Option<Product>, sqlx::Error> {
        Self::decrease_stock_returning_with(&self.pool, id, target_stock).await
    }
}
