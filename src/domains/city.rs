use axum::{
  Json, Router,
  extract::State,
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
};
use sqlx::PgPool;

use crate::domains::entities::city::{CreateCity, GetCity};

pub fn city_routes() -> Router<PgPool> {
  Router::new()
    .route("/city", get(get_all_cities))
    .route("/city", post(create_city))
}

async fn get_all_cities(State(db): State<PgPool>) -> impl IntoResponse {
  match sqlx::query_as::<_, GetCity>(
    r#"
    SELECT id, department_code, insee_code, zip_code, name, lat, lon
    FROM city
    ORDER BY id
    "#,
  )
  .fetch_all(&db)
  .await
  {
    Ok(cities) => (StatusCode::OK, Json(cities)).into_response(),
    Err(err) => (
      StatusCode::INTERNAL_SERVER_ERROR,
      format!("Database error while fetching cities: {err}"),
    )
      .into_response(),
  }
}

async fn create_city(State(db): State<PgPool>, Json(city): Json<CreateCity>) -> impl IntoResponse {
  match sqlx::query_as::<_, GetCity>(
    r#"
    INSERT INTO city (department_code, insee_code, zip_code, name, lat, lon)
    VALUES ($1, $2, $3, $4, $5, $6)
    RETURNING id, department_code, insee_code, zip_code, name, lat, lon
    "#,
  )
  .bind(&city.department_code)
  .bind(&city.insee_code)
  .bind(&city.zip_code)
  .bind(&city.name)
  .bind(city.lat)
  .bind(city.lon)
  .fetch_one(&db)
  .await
  {
    Ok(created_city) => (StatusCode::CREATED, Json(created_city)).into_response(),
    Err(err) => (
      StatusCode::INTERNAL_SERVER_ERROR,
      format!("Database error while creating city: {err}"),
    )
      .into_response(),
  }
}
