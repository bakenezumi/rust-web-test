use axum::{
  http::StatusCode,
  Json,
};
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;
use crate::company::company_dao::CompanyDao;

pub async fn find_companies(pool: MySqlPool) -> Json<Vec<Company>> {
  let company_dao = company_dao_impl::CompanyDaoImpl{ pool };
  let companies = company_dao.select_companies();
  Json(companies.await)
}

mod company_dao {
  use async_trait::async_trait;
  use crate::company::Company;

  #[async_trait]
  pub trait CompanyDao {
    async fn select_companies(&self) -> Vec<Company>;  
  }
}

mod company_dao_impl {
  use async_trait::async_trait;
  use sqlx::mysql::MySqlPool;
  use crate::company::Company;
  use crate::company::company_dao::CompanyDao;

  pub struct CompanyDaoImpl {
    pub pool: MySqlPool
  }

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
