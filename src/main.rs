use axum::{
    extract::{Path, State},
    routing::{ get, post, put, delete },
    Json, Router,
};

use serde::{Serialize, Deserialize};
use std::sync::{Arc, RwLock};


#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: u64,
    text: String,
}

type Db = Arc<RwLock<Vec<Todo>>>;

#[tokio::main]
async fn main() {
    let shared_state: Db = Arc::new(RwLock::new(Vec::new()));

    let app: Router = Router::new()
        .route("/todos", get(get_todos).post(add_todo))
        .route("/todos/{id}", put(update_todo).delete(delete_todo))
        .with_state(shared_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server started at http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap()
}

async fn get_todos(State(db): State<Db>) -> Json<Vec<Todo>> {
    let todos = db.read().unwrap();
    Json(todos.clone())
}

async fn add_todo(State(db): State<Db>, Json(todo): Json<Todo>) -> Json<Todo> {
    let mut todos = db.write().unwrap();
    todos.push(todo.clone());
    Json(todo)
}

async fn update_todo(State(db): State<Db>, Path(id): Path<u64>, Json(payload): Json<Todo>) -> &'static str {
    let mut todos = db.write().unwrap();
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.text = payload.text;
        "Updated"
    } else {
        "Not found"
    }
}

async fn delete_todo(State(db): State<Db>, Path(id): Path<u64>) -> &'static str {
    let mut todos = db.write().unwrap();
    todos.retain(|t| t.id != id);
    "Deleted"
}
