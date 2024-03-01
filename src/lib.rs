#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_one() {
    }
}

use std::fs::read_to_string;
use std::error::Error;
use image::imageops::FilterType;

const ARG_NUM: usize = 3;

pub struct Config {
    pub file_path: String,
    pub target_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < ARG_NUM {
            return Err("Not enough arguments.");
        } else {
            let config = Config {
                file_path: args[1].clone(),
                target_path: args[2].clone(),
            };
            Ok(config)
        }
    }   
}

pub fn format_image(path: &String) -> Result<image::DynamicImage, Box<dyn Error>> {
    let img = image::open(&path)?
        .grayscale()
        .resize(400, 400, FilterType::Nearest);
    
    Ok(img)
}

pub fn read_file(config: &Config) -> String {
    let text: String = read_to_string(&config.file_path).unwrap();
    text
}
