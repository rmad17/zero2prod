use axum::{Form, http};
use axum_test::{TestServer, multipart::MultipartForm};
use serde_json::json;
use tokio::net::TcpListener;

async fn start_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    //let port = listener.local_addr().unwrap().port();
    let addr = listener.local_addr().unwrap();
    print!(
        "Using address : http://{} \n",
        listener.local_addr().unwrap()
    );
    let server = zero2prod::run(addr.to_string());
    let _ = tokio::spawn(server);
    addr.to_string()
}

#[tokio::test]
async fn test_health_check_api() {
    let addr = start_app().await;
    let url = format!("http://{}/{}", addr, "health-check");

    let router = zero2prod::app();
    let server = TestServer::new(router).unwrap();

    //let json_payload = serde_json::json!({"foo": "test"});
    let response = server.get(url.as_str()).await;
    response.assert_status_ok();
    let response_text = response.text();
    response.assert_text("Healthy!");
    assert!(response_text == "Healthy!".to_string());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let addr = start_app().await;
    let url = format!("http://{}/{}", addr, "subscribe");

    let router = zero2prod::app();
    let server = TestServer::new(router).unwrap();
    let multipart_form = MultipartForm::new()
        .add_text("name", "Isco Alcaron")
        .add_text("email", "isco@rmad.com");

    //let payload = serde_json::json!({"name": "Isco Alcaron", "email": "isco@rmad.com"});
    //let response = server.post(url.as_str()).json(&json!(payload)).await;
    let response = server
        .post(url.as_str())
        .add_header(
            http::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .content_type("application/x-www-form-urlencoded")
        //.content_type("multipart/form-data")
        .multipart(multipart_form)
        .await;
    response.assert_status_ok();
}

#[tokio::test]
async fn subscribe_returns_a_400_for_valid_form_data() {
    let addr = start_app().await;
    let url = format!("http://{}/{}", addr, "subscribe");

    let router = zero2prod::app();
    let server = TestServer::new(router).unwrap();

    let payload = serde_json::json!({});
    let response = server
        .post(url.as_str())
        .add_header(
            http::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .content_type("application/x-www-form-urlencoded")
        .json(&json!(payload))
        .await;
    response.assert_status_bad_request();
}
