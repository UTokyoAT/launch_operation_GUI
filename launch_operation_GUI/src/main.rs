use env_logger;
use launch_operation_GUI::integration_test::integration_test;
use launch_operation_GUI::code_generation::code_generation_main;
use std::env;
use std::path::Path;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);
    if args.len() <= 2 {
        panic!("invalid number of arguments");
    }
    match args[1].as_str() {
        "generate" => {
            if args.len() != 3 {
                panic!("invalid number of arguments");
            }
            let config_path = args[2].clone();
            let config_path = Box::from(Path::new(&config_path));
            let output_path = Box::from(Path::new("src/generated_code"));
            code_generation_main::generate(config_path, output_path);
        },
        "test" => {
            env::set_var("RUST_LOG", "info");
            env_logger::init();
            integration_test();
        }
        _ => {
            panic!("Unknown command: {}", args[0]);
        }
    }
}
