mod models;
mod routes;
mod middlewares;

use axum::Router;
use sqlx::sqlite::SqlitePool;
use tower_sessions::SessionManagerLayer;
use tower_sessions_sqlx_store::{SqliteStore};

use routes::todos::todos_routes;

use crate::routes::{auth::auth_routes, users::users_routes};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&db_url).await?;

    let session_store = SqliteStore::new(pool.clone());
    // Create tower_sessions table
    session_store.migrate().await?;
    // Run custom sqlx migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    let session_layer = SessionManagerLayer::new(session_store);

    let app = Router::new()
        .merge(todos_routes(pool.clone()))
        .merge(auth_routes(pool.clone()))
        .merge(users_routes(pool.clone()))
        .layer(session_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started at http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
