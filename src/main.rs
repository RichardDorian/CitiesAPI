use crate::app::create_router;

pub mod app;
pub mod domains;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let app = create_router();

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
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
