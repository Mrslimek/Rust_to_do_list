use axum::{ extract::Path, routing::get, Router };


#[tokio::main]
async fn main() {
    let app: Router = Router::new()
        .route("/hello/{name}", get(|Path(name): Path<String>| async move {
            format!("Hello, {}!", name)
        }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server started at http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap()
}
