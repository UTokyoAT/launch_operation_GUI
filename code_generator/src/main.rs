use code_generator::code_generation_main;
use std::env::args;
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        panic!("invalid number of arguments");
    }
    let config_path = args[1].clone();
    let config_path = Box::from(Path::new(&config_path));
    let output_path = Box::from(Path::new("../launch_operation_GUI/src/generated_code"));
    code_generation_main::generate(config_path, output_path);
}
