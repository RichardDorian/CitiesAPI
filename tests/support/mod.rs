use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::env;
use std::str::FromStr;
use std::time::Duration;
use tokio::sync::oneshot;

use citiesapi::app::create_router;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CityPayload {
  pub department_code: String,
  pub insee_code: Option<String>,
  pub zip_code: Option<String>,
  pub name: String,
  pub lat: f64,
  pub lon: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CityResponse {
  pub id: i32,
  pub department_code: String,
  pub insee_code: Option<String>,
  pub zip_code: Option<String>,
  pub name: String,
  pub lat: f64,
  pub lon: f64,
}

pub struct TestServer {
  pub addr: std::net::SocketAddr,
  pub client: reqwest::Client,
  shutdown_tx: oneshot::Sender<()>,
  server_handle: tokio::task::JoinHandle<()>,
}

impl TestServer {
  pub fn url(&self, path: &str) -> String {
    let path = if path.is_empty() {
      "/".to_owned()
    } else if path.starts_with('/') {
      path.to_owned()
    } else {
      format!("/{path}")
    };

    format!("http://{}{}", self.addr, path)
  }

  pub async fn shutdown(self) {
    let _ = self.shutdown_tx.send(());
    let _ = self.server_handle.await;
  }
}

fn database_url_from_env() -> String {
  env::var("CITY_API_DB_URL")
    .or_else(|_| env::var("DATABASE_URL"))
    .unwrap_or_else(|_| "postgres://postgres:password@127.0.0.1:5432/city_api".to_owned())
}

fn connect_options(database_url: &str) -> PgConnectOptions {
  PgConnectOptions::from_str(database_url)
    .expect("CITY_API_DB_URL/DATABASE_URL must be a valid Postgres connection string")
}

pub fn lazy_pg_pool() -> PgPool {
  let _ = dotenvy::dotenv();
  let database_url = database_url_from_env();

  PgPoolOptions::new()
    .max_connections(1)
    .connect_lazy_with(connect_options(&database_url))
}

pub async fn try_pg_pool() -> Option<PgPool> {
  let _ = dotenvy::dotenv();
  let database_url = database_url_from_env();
  let options = connect_options(&database_url);

  match PgPoolOptions::new()
    .max_connections(5)
    .acquire_timeout(Duration::from_secs(2))
    .connect_with(options)
    .await
  {
    Ok(db) => Some(db),
    Err(err) => {
      eprintln!("Skipping test: cannot connect to Postgres ({err})");
      None
    }
  }
}

pub async fn spawn_app(pool: PgPool) -> TestServer {
  let app = create_router(pool);

  let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
    .await
    .expect("failed to bind ephemeral port");
  let addr = listener.local_addr().expect("listener has no local addr");

  let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
  let server_handle = tokio::spawn(async move {
    axum::serve(listener, app)
      .with_graceful_shutdown(async {
        let _ = shutdown_rx.await;
      })
      .await
      .expect("server error");
  });

  TestServer {
    addr,
    client: reqwest::Client::new(),
    shutdown_tx,
    server_handle,
  }
}

pub async fn spawn_app_lazy_db() -> TestServer {
  spawn_app(lazy_pg_pool()).await
}

pub async fn try_spawn_app() -> Option<TestServer> {
  let pool = try_pg_pool().await?;
  Some(spawn_app(pool).await)
}

pub fn assert_f64_close(left: f64, right: f64) {
  assert!((left - right).abs() < 1e-9);
}
