use axum::{response::IntoResponse, routing::get};

use self::state::AppState;

pub mod endpoint;
pub mod helper;
pub mod schema;
pub mod serve;
pub mod state;
pub async fn api_router() -> axum::Router {
    let state = AppState::new().await;
    axum::Router::new().merge(user_router()).with_state(state)
}

fn user_router() -> axum::Router<AppState> {
    axum::Router::new()
        .nest("/api", endpoint::user_info_router())
        .route("/user", get(|| async move { "hello" }))
        .route("/count", get(count_handler))
}
async fn count_handler() -> impl IntoResponse {
    use std::sync::atomic::AtomicUsize;
    static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
    let id = ATOMIC_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    (axum::http::StatusCode::OK, axum::Json(id))
}
