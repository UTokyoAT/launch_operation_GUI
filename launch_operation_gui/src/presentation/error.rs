use axum::response::IntoResponse;
use axum::response::Response;
use axum::http::StatusCode;
use log::error;

#[derive(Debug)]

pub struct InternalServerError(anyhow::Error);

impl IntoResponse for InternalServerError {
    fn into_response(self) -> Response {
        error!("InternalServerError: {}", self.0);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

impl<E> From<E> for InternalServerError
where
    E: Into<anyhow::Error>,
{
    fn from(e: E) -> Self {
        InternalServerError(e.into())
    }
}

