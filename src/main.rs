use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    // build our application with a single route
    //let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    // our router
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health-check", get(health_check));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// which calls one of these handlers
async fn health_check() -> String {
    return "Healthy".to_string();
}
