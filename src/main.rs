use crate::auth::jwt::Keys;
use crate::handlers::{protected_routes, public_routes};
use crate::repositories::user_repo::UserRepository;
use crate::services::auth_service::AuthService;
use axum::Router;
use axum::extract::FromRef;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

mod auth;
mod handlers;
mod models;
mod repositories;
mod services;

#[derive(Clone, FromRef)]
pub struct AppState {
    keys: Keys,
    auth_service: Arc<AuthService>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret_key = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .unwrap();

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("migration failed");

    let app_state = AppState {
        keys: Keys::new(jwt_secret_key.as_bytes()),
        auth_service: Arc::new(AuthService::new(
            Keys::new(jwt_secret_key.as_bytes()).encoding,
            UserRepository::new(pool.clone()),
        )),
    };

    let app = Router::new()
        .merge(public_routes())
        .merge(protected_routes())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
