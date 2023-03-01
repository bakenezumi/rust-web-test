// use axum::debug_handler;
use crate::AppError;
use application::company::Company;
use application::company::CreateCompany;
use application::AppState;
use axum::{extract::State, http::StatusCode, Json};

// curl localhost:3000/companies
// #[debug_handler]
pub async fn find_companies(State(state): State<AppState>) -> Result<Json<Vec<Company>>, AppError> {
    let dao = state.company_dao.read().await;
    let result = dao.find().await?;
    Ok(Json(result))
}

// curl -X POST -H "Content-Type: application/json" -d '{"name":"test_name", "alphabet":"test_name"}' localhost:3000/companies
// #[debug_handler]
pub async fn create_company(
    state: State<AppState>,
    Json(payload): Json<CreateCompany>,
) -> Result<(StatusCode, Json<Company>), AppError> {
    let dao = state.company_dao.read().await;
    let result = dao.create(payload).await?;
    Ok((StatusCode::CREATED, Json(result)))
}
