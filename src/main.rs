use color_thief::{get_palette, ColorFormat};
use image::GenericImageView;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <image_path>", args[0]);
        std::process::exit(1);
    }

    let image_path = &args[1];

    // Load the image
    let img = match image::open(&Path::new(image_path)) {
        Ok(img) => img,
        Err(err) => {
            eprintln!("Failed to open image: {}", err);
            std::process::exit(1);
        }
    };

    // Convert image to raw RGB bytes
    let (_width, _height) = img.dimensions();
    let img_bytes = img.to_rgb8().into_raw();

    // Extract color palette
    match get_palette(&img_bytes, ColorFormat::Rgb, 8, 16) {
        Ok(palette) => {
            println!("Generated 16-color palette:");
            for (i, color) in palette.iter().enumerate() {
                println!(
                    "Color {}: #{:02X}{:02X}{:02X}",
                    i + 1,
                    color.r,
                    color.g,
                    color.b
                );
            }
        }
        Err(err) => {
            eprintln!("Failed to extract colors: {}", err);
        }
    }
}
