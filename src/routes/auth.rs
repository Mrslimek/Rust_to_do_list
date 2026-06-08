use axum::{
    http::StatusCode,
    routing::{post, get},
    Json, Router, response::IntoResponse,
};
use serde::Deserialize;
use tower_sessions::Session;


#[derive(Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

pub fn auth_routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
}

async fn login(session: Session, Json(payload): Json<LoginPayload>) -> impl IntoResponse {
    if payload.username == "admin" && payload.password == "secret" {
        session.insert("user_id", 42i64).await.unwrap();

        (StatusCode::OK, "Successfully logged in").into_response()
    } else {
        (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
    }
}

async fn logout(session: Session) -> impl IntoResponse {
    session.clear().await;
    (StatusCode::OK, "Logged out").into_response()
}
