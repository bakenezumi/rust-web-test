use serde::{Deserialize, Serialize};

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

#[derive(Deserialize)]
pub struct CreateCompany {
    pub name: String,
    pub alphabet: String,
}

#[derive(Serialize)]
pub struct Company {
    pub id: i64,
    pub name: String,
    pub alphabet: String,
}
