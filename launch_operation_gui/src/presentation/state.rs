use crate::application::SendService;
use crate::application::ReceiveService;
use crate::traits::{Sender, Receiver};
use crate::traits::Sendable;
use std::fmt::Debug;
use std::sync::Arc;
use anyhow::Result;
use axum::Json;
use serde_json::Value;
use crate::command_parser::CommandParser;
use std::future::Future;
use std::pin::Pin;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub send: Arc<dyn Fn(String) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send + Sync>,
    pub receive: Arc<dyn Fn() -> Pin<Box<dyn Future<Output = Result<Json<Value>>> + Send>> + Send + Sync>,
}

impl AppState {
    pub fn new<S: Sender + Send + Sync + 'static, R: Receiver + Send + Sync + 'static, T: Clone + Send + Sync + Sendable + Debug + serde::Serialize + 'static, F: Fn(T) + Send + Sync + 'static>(sender: S, receiver: R, command_parser: CommandParser<T>, on_receive: F) -> AppState {
        let send_service = Arc::new(Mutex::new(SendService::new(command_parser, sender)));
        let receive_service = Arc::new(Mutex::new(ReceiveService::new(on_receive, receiver)));

        let receive_service_clone = receive_service.clone();

        AppState {
            send: Arc::new(move |command: String| -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
                let service = send_service.clone();
                Box::pin(async move {
                    let mut service = service.lock().await;
                    service.send(command).await
                })
            }),
            receive: Arc::new(move || -> Pin<Box<dyn Future<Output = Result<Json<Value>>> + Send>> {
                let service = receive_service_clone.clone();
                Box::pin(async move {
                    let mut service = service.lock().await;
                    let data = service.receive().await?;
                    Ok(Json(serde_json::to_value(data)?))
                })
            }),
        }
    }
}
