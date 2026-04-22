use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct City {
  pub id: i64,
  pub department_code: String,
  pub insee_code: Option<String>,
  pub zip_code: Option<String>,
  pub name: String,
  pub lat: f64,
  pub lon: f64,
}
