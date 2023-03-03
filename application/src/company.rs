pub mod company_dao {
    use crate::company::Company;
    use crate::company::CreateCompany;
    use async_trait::async_trait;
    use futures_core::stream::BoxStream;

    #[async_trait]
    pub trait CompanyDao: Send + Sync {
        async fn find(&self) -> anyhow::Result<Vec<Company>>;
        fn find_iter(&self) -> BoxStream<anyhow::Result<Company>>;
        async fn create(&self, payload: CreateCompany) -> anyhow::Result<Company>;
    }
}

pub struct CreateCompany {
    pub name: String,
    pub alphabet: String,
}

pub struct Company {
    pub id: i64,
    pub name: String,
    pub alphabet: String,
}
