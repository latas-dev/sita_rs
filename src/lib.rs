#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_one() {
    }
}

use std::error::Error;
use image::buffer::ConvertBuffer;
use image::GrayImage;
use image::imageops::FilterType;

const ARG_NUM: usize = 2;
const VALUES: [char; 11] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@', '█'];

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < ARG_NUM {
            return Err("Not enough arguments.");
        } else {
            let config = Config {
                file_path: args[1].clone(),
            };
            Ok(config)
        }
    }   
}

pub fn format_image(path: &String) -> Result<image::GrayImage, Box<dyn Error>> {
    let img = image::open(&path)?
        .resize(100, 100, FilterType::Nearest)
        .to_rgb8();
    
    let img: GrayImage = img.convert();

    Ok(img)
}

pub fn image_to_text(img: &image::GrayImage) {
    let width = img.width();
    let height = img.height();

    let inter: usize = 256 / 10;

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y)[0];
            print!("{}", VALUES[(pixel / inter as u8) as usize]);
        }
        println!("");
    }
}    