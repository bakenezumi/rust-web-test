use crate::{AppError, AppState};
// use axum::debug_handler;
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

// curl localhost:3000/companies
// #[debug_handler]
pub async fn find_companies(State(state): State<AppState>) -> Result<Json<Vec<Company>>, AppError> {
    let dao = state.company_dao.read().await;
    let result = dao.find().await?;
    Ok(Json(result))
}

// curl -X POST -H "Content-Type: application/json" -d '{"name":"test_name"}' localhost:3000/companies
// #[debug_handler]
pub async fn create_company(
    state: State<AppState>,
    Json(payload): Json<CreateCompany>,
) -> Result<(StatusCode, Json<Company>), AppError> {
    let dao = state.company_dao.read().await;
    let result = dao.create(payload).await?;
    Ok((StatusCode::CREATED, Json(result)))
}

pub mod company_dao {
    use crate::company::Company;
    use crate::company::CreateCompany;
    use async_trait::async_trait;

    #[async_trait]
    pub trait CompanyDao: Send + Sync {
        async fn find(&self) -> anyhow::Result<Vec<Company>>;
        async fn create(&self, payload: CreateCompany) -> anyhow::Result<Company>;
    }
}

pub mod company_dao_impl {
    use crate::company::company_dao::CompanyDao;
    use crate::company::Company;
    use crate::company::CreateCompany;
    use async_trait::async_trait;
    use sqlx::mysql::MySqlPool;

    pub struct CompanyDaoImpl {
        pub pool: MySqlPool,
    }

    #[async_trait]
    impl CompanyDao for CompanyDaoImpl {
        async fn find(&self) -> anyhow::Result<Vec<Company>> {
            let (id, name) = sqlx::query_as("SELECT id, name from companies")
                .fetch_one(&self.pool)
                .await?;

            Ok(vec![Company { id, name }])
        }

        async fn create(&self, payload: CreateCompany) -> anyhow::Result<Company> {
            let created = Company {
                id: 1337,
                name: payload.name,
            };
            Ok(created)
        }
    }
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
