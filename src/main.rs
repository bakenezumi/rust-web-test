use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::mysql::MySqlPool;

mod company;

#[tokio::main]
async fn main() {
    // initialize tracing
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("global dispatch set failed");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root@localhost/temporter_development")
        .await
        .expect("failed to create MySql connection pool");

    let app = router(pool);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


fn router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/companies", post(company::create_company))
        .route("/companies", get({
            || company::find_companies(pool)
        }))
}

async fn root() -> &'static str {
    "Hello, World!"
}
