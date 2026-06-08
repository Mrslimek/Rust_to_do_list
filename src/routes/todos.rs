use axum::response::IntoResponse;
use axum::routing::{post, put, delete, get};
use axum::{
    extract::{Path, State},
    Json, Router
};
use sqlx::SqlitePool;

use crate::middlewares::auth_guard;
use crate::models::Todo;

pub fn todos_routes(pool: SqlitePool) -> Router {
    Router::new()
    .route("/todos", get(get_todos).post(add_todo))
    .route("/todos/{id}", put(update_todo).delete(delete_todo))
    .layer(axum::middleware::from_fn(auth_guard))
    .with_state(pool)
}

async fn get_todos(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let todos = sqlx::query_as::<_, Todo>("SELECT id, text FROM todos")
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(todos).into_response()
}

async fn add_todo(State(pool): State<SqlitePool>, Json(payload): Json<Todo>) -> Json<Todo> {
    let result = sqlx::query!("INSERT INTO todos (text) VALUES (?)", payload.text)
        .execute(&pool)
        .await
        .unwrap();
    let id = result.last_insert_rowid();
    Json(Todo {
        id,
        text: payload.text
    })
}

async fn update_todo(State(pool): State<SqlitePool>, Path(id): Path<i64>, Json(payload): Json<Todo>) -> &'static str {
    let result = sqlx::query!("UPDATE todos SET text = ? WHERE id = ?", payload.text, id)
        .execute(&pool)
        .await
        .unwrap();

    if result.rows_affected() > 0 {
        "Updated"
    } else {
        "Not found"
    }
}

async fn delete_todo(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> &'static str {
    let result = sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(&pool)
        .await
        .unwrap();
    if result.rows_affected() > 0 {
        "Deleted"
    } else {
        "Not found"
    }
}
