use std::env;

use axum::{
    Form, Router,
    http::HeaderMap,
    response::Html,
    routing::{get, post},
};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DbConn};
use serde::Deserialize;
use Subscription;

#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

pub async fn run(addr: String) {
    // build our application with a single route
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    Migrator::up(&conn, None).await.unwrap();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}

pub fn app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health-check", get(health_check))
        .route("/subscribe", post(subscribe))
}

// which calls one of these handlers
async fn health_check() -> String {
    "Healthy!".to_string()
}

async fn subscribe(headers: HeaderMap, Form(_data): Form<FormData>) -> Html<String> {
    println!("Content: {:?}", headers);
    Html(format!("Congratulations {} {}!", &_data.name, &_data.email))
}

impl dborm{
    pub async fn create(db: %DbConn, form_data: Subscription){

    }
}
