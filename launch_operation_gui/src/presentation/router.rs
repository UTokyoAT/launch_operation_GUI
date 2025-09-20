use axum::{routing::get, Router, routing::post};
use crate::presentation::handler;
use crate::presentation::state::AppState;
use axum::http::HeaderValue;
use tower_http::cors::CorsLayer;
use axum::http::Method;
use tower_http::trace::TraceLayer;
use tower_http::trace::DefaultMakeSpan;
use tower_http::trace::DefaultOnRequest;
use tower_http::trace::DefaultOnResponse;
use tower_http::trace::DefaultOnFailure;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/accept_names", get(handler::get_accept_names))
        .route("/log", get(handler::log_sender))
        .route("/send", post(handler::send_command))
        .layer(
            CorsLayer::new()
                .allow_origin(vec![HeaderValue::from_static("http://localhost:5173")])
                .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new())
                .on_request(DefaultOnRequest::new())
                .on_response(DefaultOnResponse::new())
                .on_failure(DefaultOnFailure::new())
        )
        .with_state(state)
}