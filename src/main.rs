use std::net::SocketAddr;

use clap::Parser;

use crate::app::create_router;
use sqlx::postgres::PgPoolOptions;

pub mod app;
pub mod domains;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(
    short = 'c',
    long = "host",
    env = "CITY_API_ADDR",
    default_value = "127.0.0.1"
  )]
  api_addr: String,

  #[arg(
    short = 'p',
    long = "port",
    env = "CITY_API_PORT",
    default_value_t = 2022
  )]
  api_port: u16,

  #[arg(long = "db", env = "CITY_API_DB_URL")]
  db_url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let args = Args::parse();

  let db_pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&args.db_url)
    .await?;

  let app = create_router(db_pool);

  let socket_addr = format!("{}:{}", args.api_addr, args.api_port)
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
