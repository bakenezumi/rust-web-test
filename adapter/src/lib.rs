pub mod company_dao_impl {
    use application::company::company_dao::CompanyDao;
    use application::company::Company;
    use application::company::CreateCompany;
    use async_trait::async_trait;
    use futures_core::stream::BoxStream;
    use futures_util::StreamExt;
    use sqlx::mysql::MySqlPool;

    pub struct CompanyDaoImpl {
        pub pool: MySqlPool,
    }

    #[async_trait]
    impl CompanyDao for CompanyDaoImpl {
        async fn find(&self) -> anyhow::Result<Vec<Company>> {
            let companies = sqlx::query_as(
                "
SELECT id, name, alphabet
FROM companies
ORDER BY id
",
            )
            .fetch_all(&self.pool)
            .await?;

            let result = companies
                .into_iter()
                .map(|(id, name, alphabet)| Company { id, name, alphabet })
                .collect();

            Ok(result)
        }

        fn find_iter(&self) -> BoxStream<anyhow::Result<Company>> {
            let companies = sqlx::query_as(
                "
SELECT id, name, alphabet
FROM companies
ORDER BY id
",
            )
            .fetch(&self.pool);

            let stream = companies
                .map(|result| match result {
                    Ok((id, name, alphabet)) => Ok(Company { id, name, alphabet }),
                    Err(e) => Err(anyhow::anyhow!(e)),
                })
                .boxed();

            stream
        }

        async fn create(&self, payload: CreateCompany) -> anyhow::Result<Company> {
            let id = sqlx::query("INSERT INTO companies (name, alphabet, created_at, updated_at) VALUES (?, ?, now(), now())")
                .bind(&payload.name)
                .bind(&payload.alphabet)
                .execute(&self.pool)
                .await?
                .last_insert_id();

            let created = Company {
                id: id as i64,
                name: payload.name,
                alphabet: payload.alphabet,
            };
            Ok(created)
        }
    }
}
