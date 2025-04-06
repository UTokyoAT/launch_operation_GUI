pub mod csv_writer;
pub mod gui {
    pub mod gui;
}
pub mod code_generation {
    pub mod code_generation_main;
    pub mod config_parser;
    pub mod code_generation_context;
    pub mod var_type;
    pub mod code_generator;
    pub mod template_parser;
}
pub mod generated_code {
    pub mod rust {
        pub mod command;
        pub mod log;
    }
}
pub mod integration_test;
pub mod serial_communication;
pub mod service;
pub mod traits;
