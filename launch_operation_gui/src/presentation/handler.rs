use axum::extract::ws::{WebSocket, Message, WebSocketUpgrade};
use axum::response::IntoResponse;
use crate::presentation::state::AppState;
use axum::extract::State;
use crate::presentation::error::InternalServerError;
use tracing;

pub async fn log_sender(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    loop {
        let result = (state.receive)().await;
        match result {
            Ok(data) => {
                let result = socket.send(Message::Text(data.to_string().into())).await;
                if let Err(e) = result {
                    tracing::error!("websocket error: {}", e);
                    break;
                }
            },
            Err(e) => {
                tracing::error!("receive error: {}", e);
            }
        }
    }
}

pub async fn send_command(State(state): State<AppState>, body: String) -> Result<impl IntoResponse, InternalServerError> {
    (state.send)(body).await?;
    Ok("Command sent successfully")
}