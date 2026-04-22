use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use sqlx::{PgPool};

use crate::domains::entities::city::City;

pub fn city_routes() -> Router<PgPool> {
  Router::new().route("/cities", get(get_all_cities))
}

async fn get_all_cities(State(db): State<PgPool>) -> impl IntoResponse {
  match sqlx::query_as::<_, City>(
    r#"
    SELECT id, department_code, insee_code, zip_code, name, lat, lon
    FROM city
    ORDER BY id
    "#,
  )
  .fetch_all(&db)
  .await {
    Ok(cities) => (StatusCode::OK, Json(cities)).into_response(),
    Err(err) => (
      StatusCode::INTERNAL_SERVER_ERROR,
      format!("Database error while fetching cities: {err}"),
    ).into_response(),
  }
}
