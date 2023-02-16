use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("global dispatch set failed");

    let app = Router::new()
        .route("/", get(root))
        .route("/companies", post(create_company))
        .route("/companies", get(find_companies));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn find_companies() -> Json<Vec<Company>> {
    let companies = vec!(Company {
        id: 1337,
        name: "John".to_string(),
    });
    Json(companies)
}

async fn create_company(
    Json(payload): Json<CreateCompany>,
) -> (StatusCode, Json<Company>) {
    let company = Company {
        id: 1337,
        name: payload.name,
    };
    (StatusCode::CREATED, Json(company))
}


#[derive(Deserialize)]
struct CreateCompany {
    name: String,
}

#[derive(Serialize)]
struct Company {
    id: u64,
    name: String,
}
