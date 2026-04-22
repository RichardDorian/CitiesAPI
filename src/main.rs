use std::env;
use std::net::SocketAddr;

use crate::app::create_router;
use sqlx::postgres::PgPoolOptions;

pub mod app;
pub mod domains;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let database_url = env::var("DATABASE_URL")
    .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/city_api".to_owned());

  let db_pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await?;

  let app = create_router(db_pool);

  let addr = env::var("CITY_API_ADDR").unwrap_or_else(|_| "127.0.0.1".to_owned());
  let port = env::var("CITY_API_PORT")
    .unwrap_or_else(|_| "2022".to_owned())
    .parse::<u16>()
    .expect("CITY_API_PORT must be a valid port number.");

  let socket_addr = format!("{addr}:{port}")
    .parse::<SocketAddr>()
    .expect("Invalid socket address.");

  let listener = tokio::net::TcpListener::bind(socket_addr).await?;
  axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await?;

  Ok(())
}

pub async fn shutdown_signal() {
  tokio::signal::ctrl_c()
    .await
    .expect("Failed to install CTRL+C signal handler");
}
