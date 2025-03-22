#[cfg(test)]
//use axum::{Router, routing::get};
use axum_test::TestServer;

//use crate::health_check;
async fn start_app() {
    let server = zero2prod::run();
    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn test_health_check_api() {
    start_app().await;

    // Assuming your server is running on port 3000
    let url = "http://0.0.0.0:3000/health-check";

    let router = zero2prod::app();
    let server = TestServer::new(router).unwrap();

    //let json_payload = serde_json::json!({"foo": "test"});
    let response = server.get(url).await;
    response.assert_status_ok();
    let response_text = response.text();
    response.assert_text("Healthy!");
    assert!(response_text == "Healthy!".to_string());
}
