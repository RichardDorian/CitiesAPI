use axum::{Router, http::StatusCode};
use sqlx::PgPool;

use crate::domains::city::city_routes;
use crate::domains::example::example_routes;

pub fn create_router(db_pool: PgPool) -> Router {
  Router::<PgPool>::new()
    .route("/_health", axum::routing::get(health_check))
    .merge(example_routes())
    .merge(city_routes())
    .with_state(db_pool)
    .fallback(fallback)
}

async fn health_check() -> StatusCode {
  StatusCode::NO_CONTENT
}

async fn fallback() -> (StatusCode, String) {
  (StatusCode::NOT_FOUND, "Not Found".to_string())
}
