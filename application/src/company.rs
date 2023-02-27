use crate::{AppError, AppState};
use axum::{debug_handler, extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[debug_handler]
pub async fn find_companies(State(state): State<AppState>) -> Result<Json<Vec<Company>>, AppError> {
    let dao = state.company_dao.lock().await;
    let future = dao.select_companies();
    {
        let result = future.await.map_err(|e| AppError(e));
        Ok(Json(result?))
    }
}

pub mod company_dao {
    use crate::company::Company;
    use async_trait::async_trait;

    #[async_trait]
    pub trait CompanyDao: Send {
        async fn select_companies(&self) -> Result<Vec<Company>, anyhow::Error>;
    }
}

pub mod company_dao_impl {
    use crate::company::company_dao::CompanyDao;
    use crate::company::Company;
    use async_trait::async_trait;
    use sqlx::mysql::MySqlPool;

    pub struct CompanyDaoImpl {
        pub pool: MySqlPool,
    }

    #[async_trait]
    impl CompanyDao for CompanyDaoImpl {
        async fn select_companies(&self) -> Result<Vec<Company>, anyhow::Error> {
            let row: (i64, String) = sqlx::query_as("SELECT id, name from companies")
                .fetch_one(&self.pool)
                .await
                .map_err(|e| anyhow::anyhow!(e))?;

            Ok(vec![Company {
                id: row.0,
                name: row.1,
            }])
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
