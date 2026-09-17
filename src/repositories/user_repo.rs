use crate::models::user::User;
use sqlx::PgPool;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT id,username,password_hash FROM users WHERE username=$1")
            .bind(username)
            .fetch_optional(&self.pool)
            .await
    }
}
