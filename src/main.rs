use image::{DynamicImage, GenericImageView, Pixel};
use std::collections::HashMap;
use std::env;
use std::path::Path;

fn get_palette(img: &DynamicImage, color_count: usize) -> Vec<(u8, u8, u8)> {
    let mut color_map: HashMap<(u8, u8, u8), usize> = HashMap::new();

    for pixel in img.pixels() {
        let rgb = pixel.2.to_rgb();
        let color = (rgb[0], rgb[1], rgb[2]);
        *color_map.entry(color).or_insert(0) += 1;
    }

    let mut sorted_colors: Vec<_> = color_map.into_iter().collect();
    sorted_colors.sort_by(|a, b| b.1.cmp(&a.1));

    sorted_colors
        .into_iter()
        .take(color_count)
        .map(|(color, _)| color)
        .collect()
}

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

    // Extract color palette
    let palette = get_palette(&img, 16);
    println!("Generated 16-color palette:");
    for (i, color) in palette.iter().enumerate() {
        println!(
            "Color {}: #{:02X}{:02X}{:02X}",
            i + 1,
            color.0,
            color.1,
            color.2
        );
    }
}
