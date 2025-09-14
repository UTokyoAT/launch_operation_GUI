pub mod csv_writer;
pub mod serial_communication;
pub mod application;
pub mod traits;
pub mod presentation {
    pub mod router;
    pub mod state;
    mod handler;
    mod error;
}
pub mod command_parser;
pub mod server;