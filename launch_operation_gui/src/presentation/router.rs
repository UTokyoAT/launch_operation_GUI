use axum::{routing::get, Router, routing::post};
use crate::presentation::handler;
use crate::presentation::state::AppState;
use axum::http::HeaderValue;
use tower_http::cors::CorsLayer;
use axum::http::Method;


pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/log", get(handler::log_sender))
        .route("/send", post(handler::send_command))
        .layer(
            CorsLayer::new()
                .allow_origin(vec![HeaderValue::from_static("http://localhost:5173")])
                .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
        )
        .with_state(state)
}