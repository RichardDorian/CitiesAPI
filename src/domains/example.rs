use axum::{Router, routing::get};
use sqlx::PgPool;

pub fn example_routes() -> Router<PgPool> {
  Router::new().route("/example", get(|| async { "Hello, World!" }))
}
