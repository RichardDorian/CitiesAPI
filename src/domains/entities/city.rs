use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct City {
  pub id: i32,
  pub department_code: String,
  pub insee_code: Option<String>,
  pub zip_code: Option<String>,
  pub name: String,
  pub lat: f64,
  pub lon: f64,
}
