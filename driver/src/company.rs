use crate::AppError;
use application::company::Company;
use application::company::CreateCompany;
use application::AppState;
use axum::debug_handler;
use axum::{extract::State, http::StatusCode, Json};
use futures_util::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(remote = "CreateCompany")]
pub struct CreateCompanyDef {
    pub name: String,
    pub alphabet: String,
}

#[derive(Deserialize)]
pub struct CreateCompanyWrapper(#[serde(with = "CreateCompanyDef")] CreateCompany);

#[derive(Serialize)]
#[serde(remote = "Company")]
pub struct CompanyDef {
    pub id: i64,
    pub name: String,
    pub alphabet: String,
}

#[derive(Serialize)]
pub struct CompanyWrapper(#[serde(with = "CompanyDef")] Company);

// curl localhost:3000/companies
// #[debug_handler]
pub async fn find_companies(
    State(state): State<AppState>,
) -> Result<Json<Vec<CompanyWrapper>>, AppError> {
    let dao = state.company_dao.read().await;
    let result = dao
        .find()
        .await?
        .into_iter()
        .map(|x| CompanyWrapper(x))
        .collect();
    Ok(Json(result))
}

// curl localhost:3000/companies-stream
// #[debug_handler]
pub async fn find_companies_stream(
    State(state): State<AppState>,
) -> Result<Json<Vec<CompanyWrapper>>, AppError> {
    let dao = state.company_dao.read().await;
    let result = dao
        .find_iter()
        .await?
        .map(|r| r.map(|x| CompanyWrapper(x)))
        .try_collect()
        .await?;
    Ok(Json(result))
}

// curl -X POST -H "Content-Type: application/json" -d '{"name":"test_name", "alphabet":"test_name"}' localhost:3000/companies
#[debug_handler]
pub async fn create_company(
    state: State<AppState>,
    Json(payload): Json<CreateCompanyWrapper>,
) -> Result<(StatusCode, Json<CompanyWrapper>), AppError> {
    let dao = state.company_dao.read().await;
    let result = dao.create(payload.0).await?;
    Ok((StatusCode::CREATED, Json(CompanyWrapper(result))))
}
