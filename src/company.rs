use axum::{
  http::StatusCode,
  Json,
};
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;

pub async fn find_companies(pool: MySqlPool) -> Json<Vec<Company>> {
  let row: (i64, String,) = sqlx::query_as("SELECT id, name from companies")
  .fetch_one(&pool).await.expect("error");

  let companies = vec!(Company {
      id: row.0,
      name: row.1,
  });
  Json(companies)
}

pub async fn create_company(
  Json(payload): Json<CreateCompany>,
) -> (StatusCode, Json<Company>) {
  let company = Company {
      id: 1337,
      name: payload.name,
  };
  (StatusCode::CREATED, Json(company))
}

#[derive(Deserialize)]
pub struct CreateCompany {
  name: String,
}

#[derive(Serialize)]
pub struct Company {
  id: i64,
  name: String,
}
