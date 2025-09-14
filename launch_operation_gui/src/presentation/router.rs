use axum::{routing::get, Router, routing::post};
use crate::presentation::handler;
use crate::presentation::state::AppState;


pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/log", get(handler::log_sender))
        .route("/send", post(handler::send_command))
        .with_state(state)
}