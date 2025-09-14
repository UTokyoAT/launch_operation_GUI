use axum::extract::ws::{WebSocket, Message, WebSocketUpgrade};
use axum::response::IntoResponse;
use tokio::time::{sleep, Duration};
use chrono::Utc;

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
