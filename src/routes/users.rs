use axum::{
    routing::get,
    Router, response::IntoResponse,
};
use sqlx::SqlitePool;
use tower_sessions::Session;


pub fn users_routes(pool: SqlitePool) -> Router {
    Router::new()
        .route("/users/me", get(current_user))
        .with_state(pool)
}

async fn current_user(_session: Session) -> impl IntoResponse {

}
