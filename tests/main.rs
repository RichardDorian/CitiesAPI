use axum::http::StatusCode;
use serde_json::json;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

mod support;

use support::{CityPayload, CityResponse};

async fn health_status(url: &str) -> Option<StatusCode> {
  let client = reqwest::Client::builder()
    .timeout(Duration::from_millis(300))
    .build()
    .expect("failed to build reqwest client");

  match client.get(url).send().await {
    Ok(res) => Some(res.status()),
    Err(_) => None,
  }
}

#[tokio::test]
async fn test_healthcheck_is_down_when_server_not_launched() {
  let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
    .await
    .expect("failed to bind ephemeral port");
  let addr = listener.local_addr().expect("listener has no local addr");

  let accept_task = tokio::spawn(async move {
    let _ = listener.accept().await;
  });

  let url = format!("http://{addr}/_health");
  let status = health_status(&url).await;
  assert!(status.is_none());

  let _ = accept_task.await;
}

#[tokio::test]
async fn test_healthcheck_is_ok_when_server_is_launched() {
  let server = support::spawn_app_lazy_db().await;

  let url = server.url("/_health");
  let status = health_status(&url).await;
  assert_eq!(status, Some(StatusCode::NO_CONTENT));

  server.shutdown().await;
}

#[tokio::test]
async fn test_add_city_response_matches_payload() {
  let Some(server) = support::try_spawn_app().await else {
    return;
  };

  let cities_url = server.url("/city");

  let payload = CityPayload {
    department_code: "80".to_owned(),
    insee_code: Some("75056".to_owned()),
    zip_code: Some("75000".to_owned()),
    name: "Paris".to_owned(),
    lat: 48.8566,
    lon: 2.3522,
  };

  let res = server
    .client
    .post(cities_url)
    .json(&payload)
    .send()
    .await
    .expect("POST /city request failed");

  assert_eq!(res.status(), StatusCode::CREATED);

  let city: CityResponse = res
    .json()
    .await
    .expect("response body is not valid CityResponse JSON");

  assert!(city.id > 0, "expected DB-generated id > 0");
  assert_eq!(city.department_code, payload.department_code);
  assert_eq!(city.insee_code, payload.insee_code);
  assert_eq!(city.zip_code, payload.zip_code);
  assert_eq!(city.name, payload.name);

  // Use a small epsilon for float round-trips through Postgres + JSON.
  support::assert_f64_close(city.lat, payload.lat);
  support::assert_f64_close(city.lon, payload.lon);

  server.shutdown().await;
}

#[tokio::test]
async fn test_add_city_fails_when_missing_mandatory_fields() {
  let Some(server) = support::try_spawn_app().await else {
    return;
  };

  let cities_url = server.url("/city");

  // Missing required field: `name`.
  let invalid_payload = json!({
    "department_code": "80",
    "insee_code": "75056",
    "zip_code": "75000",
    "lat": 48.8566,
    "lon": 2.3522
  });

  let res = server
    .client
    .post(cities_url)
    .json(&invalid_payload)
    .send()
    .await
    .expect("POST /cities request failed");

  assert!(
    res.status().is_client_error(),
    "expected a 4xx response when payload is missing some mandatory fields (name) invalid, got {}",
    res.status()
  );
  assert_ne!(res.status(), StatusCode::CREATED);

  server.shutdown().await;
}

#[tokio::test]
async fn test_get_cities_returns_inserted_city() {
  let Some(server) = support::try_spawn_app().await else {
    return;
  };

  let cities_url = server.url("/city");

  let unique_suffix = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("system time before unix epoch")
    .as_nanos();

  let payload = CityPayload {
    department_code: "80".to_owned(),
    insee_code: Some("75056".to_owned()),
    zip_code: Some("75000".to_owned()),
    name: format!("Paris-{unique_suffix}"),
    lat: 48.8566,
    lon: 2.3522,
  };

  let create_res = server
    .client
    .post(&cities_url)
    .json(&payload)
    .send()
    .await
    .expect("POST /city request failed");

  assert_eq!(create_res.status(), StatusCode::CREATED);
  let created: CityResponse = create_res
    .json()
    .await
    .expect("create response body is not valid CityResponse JSON");

  let list_res = server
    .client
    .get(&cities_url)
    .send()
    .await
    .expect("GET /city request failed");

  assert_eq!(list_res.status(), StatusCode::OK);
  let cities: Vec<CityResponse> = list_res
    .json()
    .await
    .expect("list response body is not valid JSON array");

  let found = cities.into_iter().find(|c| c.id == created.id);
  let found = found.expect("inserted city id not found in GET /city response");

  assert_eq!(found.department_code, payload.department_code);
  assert_eq!(found.insee_code, payload.insee_code);
  assert_eq!(found.zip_code, payload.zip_code);
  assert_eq!(found.name, payload.name);
  support::assert_f64_close(found.lat, payload.lat);
  support::assert_f64_close(found.lon, payload.lon);

  server.shutdown().await;
}
