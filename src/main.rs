use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    // build our application with a single route

    let router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health-check", get(health_check));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

// which calls one of these handlers
async fn health_check() -> String {
    return "Healthy!".to_string();
}

#[cfg(test)]
mod tests {
    use axum::{Router, routing::get};
    use axum_test::TestServer;

    use crate::health_check;

    #[tokio::test]
    async fn test_health_check_api() {
        // Assuming your server is running on port 3000
        let url = "http://0.0.0.0:3000/health-check";
        let router = Router::new().route("/health-check", get(health_check));
        let server = TestServer::new(router).unwrap();

        //let json_payload = serde_json::json!({"foo": "test"});
        let response = server.get(url).await;
        response.assert_status_ok();
        let response_text = response.text();
        response.assert_text("Healthy!");
        assert!(response_text == "Healthy!".to_string());
    }
}
