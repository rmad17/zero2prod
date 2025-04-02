use zero2prod::run;

#[tokio::main]
async fn main() {
    run("0.0.0.0:3000".to_string()).await
}
