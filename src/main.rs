use axum::{
    extract::{Path, State},
    routing::{ get, post, put, delete },
    Json, Router,
};
use sqlx::{ sqlite::SqlitePool, FromRow };
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone, FromRow)]
struct Todo {
    id: i64,
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&db_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app: Router = Router::new()
        .route("/todos", get(get_todos).post(add_todo))
        .route("/todos/{id}", put(update_todo).delete(delete_todo))
        .with_state(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started at http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn get_todos(State(pool): State<SqlitePool>) -> Json<Vec<Todo>> {
    let todos = sqlx::query_as::<_, Todo>("SELECT id, text FROM todos")
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(todos.clone())
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
