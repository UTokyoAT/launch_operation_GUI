use env_logger;
use std::env;
use launch_operation_gui::presentation::router::router;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let app = router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Server is running on http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}
