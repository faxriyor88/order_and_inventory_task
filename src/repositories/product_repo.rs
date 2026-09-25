use crate::models::product::{CreateProduct, Product};
use sqlx::PgPool;

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

    pub async fn find_by_id(&self, id: i64) -> Result<Option<Product>, sqlx::Error> {
        sqlx::query_as::<_, Product>(r#"SELECT * FROM product WHERE id=$1"#)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn decreasing_stock(&self, id: i64, target_stock: i32) -> Result<bool, sqlx::Error> {
        let result =
            sqlx::query(r#"UPDATE product SET stock_quantity = stock_quantity - $2 WHERE id=$1"#)
                .bind(id)
                .bind(target_stock)
                .execute(&self.pool)
                .await?;

        Ok(result.rows_affected() == 1)
    }
}
