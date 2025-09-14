use axum::{routing::get, Router};
use crate::presentation::handler;

pub fn router() -> Router {
    Router::new()
        .route("/log", get(handler::log_sender))
}