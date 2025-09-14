use crate::command_parser::CommandParser;
use crate::presentation::state::AppState;
use crate::traits::{Sender, Receiver};
use crate::traits::Sendable;
use std::fmt::Debug;
use crate::presentation::router::router;
pub struct Server<T: Clone + Send + Sync + Sendable + Debug + serde::Serialize + 'static> {
    command_parser: CommandParser<T>,
}
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

impl<T: Clone + Send + Sync + Sendable + Debug + serde::Serialize + 'static> Server<T> {
    pub fn new() -> Self {
        Self { command_parser: CommandParser::new() }
    }

    pub fn add_command(self, name: &str, command: T) -> Self {
        let mut command_parser = self.command_parser;
        command_parser.add_command(name.to_string(), command);
        Self { command_parser }
    }

    pub async fn run<S: Sender + Send + Sync + 'static, R: Receiver + Send + Sync + 'static>(self, sender: S, receiver: R) {

        let _ = tracing_subscriber::registry()
            .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "debug".into()))
            .with(tracing_subscriber::fmt::layer())
            .try_init();
        let app_state = AppState::new(sender, receiver, self.command_parser, |_| {});
        let router = router(app_state);
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
        println!("Server is running on http://localhost:8080");
        axum::serve(listener, router).await.unwrap();
    }
}