use env_logger;
use launch_operation_GUI::integration_test::integration_test;
use std::env;
fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    integration_test();
}
