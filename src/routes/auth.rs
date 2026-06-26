use axum::{
    Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::{get, post}
};
use serde::Deserialize;
use sqlx::SqlitePool;
use tower_sessions::Session;

use crate::models::User;


#[derive(Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct RegisterPayload {
    username: String,
    password: String,
    confirm_password: String,
}

pub fn auth_routes(pool: SqlitePool) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .with_state(pool)
}

async fn register(State(pool): State<SqlitePool>, Json(payload): Json<RegisterPayload>) -> impl IntoResponse {
    if payload.password != payload.confirm_password {
        (StatusCode::BAD_REQUEST, "Passwords must match").into_response()
    } else {
        let user = sqlx::query!("INSERT INTO users (username, password) VALUES (?, ?)", payload.username, payload.password)
            .execute(&pool)
            .await
            .unwrap();
        let id = user.last_insert_rowid();
        Json(User {
            id,
            username: payload.username,
            password: payload.password
        }).into_response()
    }
}

async fn login(State(pool): State<SqlitePool>, session: Session, Json(payload): Json<LoginPayload>) -> impl IntoResponse {
    // TODO: Add logic of checking session first
    let result = sqlx::query_as::<_, User>("SELECT * FROM USERS WHERE username = ? and password = ?")
        .bind(&payload.username)
        .bind(&payload.password)
        .fetch_one(&pool)
        .await;
    match result {
        Ok(user) => {
            session.insert("user_id", user.id).await.unwrap();
            (StatusCode::OK, "Successfully logged in").into_response()
        }
        Err(sqlx::Error::RowNotFound) => {
            (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
        }
        Err(err) => {
            println!("Database error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}

async fn logout(session: Session) -> impl IntoResponse {
    session.clear().await;
    (StatusCode::OK, "Logged out").into_response()
}
