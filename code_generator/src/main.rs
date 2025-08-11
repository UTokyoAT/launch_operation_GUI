use clap::Parser;
use code_generator::code_generation_main;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to config file
    config_path: String,
    /// Path to output directory
    output_dir: String,
}

fn main() {
    let args = Args::parse();

    let config_path = Box::from(Path::new(&args.config_path));
    let output_path = Box::from(Path::new(&args.output_dir));

    code_generation_main::generate(config_path, output_path);
}
