use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse
};
use tower_sessions::Session;

pub async fn auth_guard(
    session: Session,
    request: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    if let Ok(Some(_user_id)) = session.get::<i64>("user_id").await {
        next.run(request).await
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}
