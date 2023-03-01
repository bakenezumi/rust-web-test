pub mod company;

use company::company_dao::CompanyDao;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub company_dao: Arc<RwLock<dyn CompanyDao>>,
}
