#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_one() {
    }
}

use std::error::Error;
use image::imageops::FilterType;

const ARG_NUM: usize = 3;
const VALUES: [char; 11] = [' ', '.', ':', ';', '=','-', '+', '*', '#', '%', '@'];
// const VALUES: [char; 11] = [ '@', '%', '#', '*','+', '=', '-', ';', ':', '.', ' '];
pub struct Config {
    pub file_path: String,
    pub width: u32,
    // 
    // pub target_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // if args.len() < ARG_NUM + 1 // save image {
        if args.len() < ARG_NUM {
            Err("Not enough arguments.")
        } else {
            let width = match args[2].parse::<u32>() {
                Ok(value)=> value,
                _ => return Err("Width must be a positive integer."),
            };

            let config = Config {
                file_path: args[1].clone(),
                width,
                // 
                // target_path: args[ARG_NUM - 1].clone()
            };
            Ok(config)
        }
    }   
}

pub fn format_image(path: &String, w: u32) -> Result<image::GrayImage, Box<dyn Error>> {
    let img = image::open(path)?;
    
    // height is a third of width so the text mantains the aspect ratio.
    let ratio= (img.width() as f32 /  img.height() as f32) / 1.8;
    let h= (w as f32 * ratio) as u32;

    let img = img
        .resize_exact(w, h, FilterType::Nearest)
        .to_luma8();

    Ok(img)
}

#[allow(clippy::println_empty_string)]
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