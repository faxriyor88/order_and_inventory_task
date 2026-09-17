use crate::AppState;
use crate::auth::middleware::AuthError;
use crate::services::auth_service::AuthService;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct LoginReq {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginRes {
    token: String,
}

pub fn auth_routes() -> Router<AppState> {
    Router::new().route("/login", post(login))
}

async fn login(
    State(svc): State<Arc<AuthService>>,
    Json(payload): Json<LoginReq>,
) -> Result<Json<LoginRes>, AuthError> {
    let token = svc.login(&payload.username, &payload.password).await?;

    Ok(Json(LoginRes { token }))
}
