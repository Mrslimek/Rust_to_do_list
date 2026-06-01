use axum::{ routing::get, Router };


#[tokio::main]
async fn main() {
    let app: Router = Router::new()
        .route("/ping", get(|| async { "pong" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server started at http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap()
}
