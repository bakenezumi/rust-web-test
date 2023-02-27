use axum::{
  http::StatusCode,
  Json,
  extract::State
};
use serde::{Deserialize, Serialize};
use crate::AppState;

pub async fn find_companies(State(state): State<AppState>) -> Json<Vec<Company>> {
  let dao = state.company_dao.lock().await;
  let companies = dao.select_companies();
  Json(companies.await)
}

pub mod company_dao {
  use async_trait::async_trait;
  use crate::company::Company;
  use dyn_clone::DynClone;

  #[async_trait]
  pub trait CompanyDao: DynClone + Send {
    async fn select_companies(&self) -> Vec<Company>;
  }

}

pub mod company_dao_impl {
  use async_trait::async_trait;
  use sqlx::mysql::MySqlPool;
  use crate::company::Company;
  use crate::company::company_dao::CompanyDao;

  #[derive(Clone)]
  pub struct CompanyDaoImpl {
    pub pool: MySqlPool
  }

  dyn_clone::clone_trait_object!(CompanyDao);

  #[async_trait]
  impl CompanyDao for CompanyDaoImpl {
    async fn select_companies(&self) -> Vec<Company> {
      let row: (i64, String,) = sqlx::query_as("SELECT id, name from companies")
      .fetch_one(&self.pool).await.expect("error");
  
      vec!(Company {
          id: row.0,
          name: row.1,
      })
    }  
  }
}

pub async fn create_company(
  _: State<AppState>,
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
