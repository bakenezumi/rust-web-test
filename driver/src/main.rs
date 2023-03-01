use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use sqlx::mysql::MySqlPoolOptions;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

use adapter::company_dao_impl::CompanyDaoImpl;
use application::AppState;

mod company;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize tracing
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_thread_ids(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root@localhost/temporter_development")
        .await?;

    let company_dao = CompanyDaoImpl { pool };

    let state = AppState {
        company_dao: Arc::new(RwLock::new(company_dao)),
    };
    let app = router(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/companies", post(company::create_company))
        .route("/companies", get(company::find_companies))
        .with_state(state)
}

async fn root(_: State<AppState>) -> &'static str {
    "Hello, World!"
}

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("error occured: {}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
