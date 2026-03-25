use axum::{Router, http::StatusCode};

use crate::domains::example::example_routes;

pub fn create_router() -> Router {
  Router::new()
    .route("/health", axum::routing::get(health_check))
    .merge(example_routes())
    .fallback(fallback)
}

async fn health_check() -> &'static str {
  "OK\n"
}

async fn fallback() -> (StatusCode, String) {
  (StatusCode::NOT_FOUND, "Not Found".to_string())
}
