use std::process;
use std::env;
use sita_rs::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|e| {
        eprintln!("Error Parsing Arguments: {e}");
        process::exit(1);
    });

    let img = sita_rs::format_image(&config.file_path).unwrap_or_else(|e| {
        eprintln!("File Error: {e}");
        process::exit(1);
    });

    sita_rs::image_to_text(&img);    
}