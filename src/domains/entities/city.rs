use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct GetCity {
  pub id: i32,
  pub department_code: String,
  pub insee_code: Option<String>,
  pub zip_code: Option<String>,
  pub name: String,
  pub lat: f64,
  pub lon: f64,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct CreateCity {
  pub department_code: String,
  pub insee_code: Option<String>,
  pub zip_code: Option<String>,
  pub name: String,
  pub lat: f64,
  pub lon: f64,
}
