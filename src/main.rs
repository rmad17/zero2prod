use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    // build our application with a single route

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health-check", get(health_check))
}

// which calls one of these handlers
async fn health_check() -> String {
    return "Healthy!".to_string();
}

#[cfg(test)]
mod tests {
    use axum::Router;
    use axum::extract::Json;
    use axum::routing::put;
    use axum_test::TestServer;
    use serde_json::Value;
    use serde_json::json;

    use super::*;
    use crate::app;
    //use axum::{
    //    body::Body,
    //    extract::connect_info::MockConnectInfo,
    //    http::{self, Request, StatusCode},
    //};

    #[tokio::test]
    async fn health_check_succeeds() {
        let app = app();
        let server = TestServer::new(app)?;

        let response = server
            .put("/users")
            .json(&json!({
                "username": "Terrance Pencilworth",
            }))
            .await;

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Healthy!");
    }
}
