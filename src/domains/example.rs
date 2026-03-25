use axum::{Router, routing::get};

pub fn example_routes() -> Router {
  Router::new().route("/example", get(|| async { "Hello, World!" }))
}
