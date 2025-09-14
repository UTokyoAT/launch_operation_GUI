use axum::extract::ws::{WebSocket, Message, WebSocketUpgrade};
use axum::response::IntoResponse;
use tokio::time::{sleep, Duration};
use chrono::Utc;
use crate::presentation::state::AppState;
use axum::extract::State;
use crate::presentation::error::InternalServerError;

pub async fn log_sender(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    loop {
        sleep(Duration::from_millis(1)).await;
        let date_time = Utc::now().to_string();
        let result =socket.send(Message::Text(date_time.into())).await;
        if let Err(_) = result {
            println!("error");
            break;
        }
    }
}

pub async fn send_command(State(state): State<AppState>, body: String) -> Result<impl IntoResponse, InternalServerError> {
    (state.send)(body).await?;
    Ok("Command sent successfully")
}