use clap::{Arg, Command};

use image::GenericImageView;
use rand::prelude::*;
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// A simple struct for storing color data in RGB.
#[derive(Debug, Clone, Copy, Serialize)]
struct RgbColor {
    r: f32,
    g: f32,
    b: f32,
}

impl RgbColor {
    fn distance_squared(&self, other: &RgbColor) -> f32 {
        (self.r - other.r).powi(2) + (self.g - other.g).powi(2) + (self.b - other.b).powi(2)
    }

    // Convert an RgbColor (0-255 in f32 form) to a hex string
    fn to_hex(&self) -> String {
        let r = self.r as u8;
        let g = self.g as u8;
        let b = self.b as u8;
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("roloc")
        .version("0.0.1")
        .author("Tomasz Bawor <[email protected]>")
        .about("Extracts a color palette from an image using k-means")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("Input image file path")
                .required(true),
        )
        .arg(
            Arg::new("colors")
                .short('k')
                .long("colors")
                .help("Number of colors to extract (default: 5)")
                .default_value("5"),
        )
        .arg(
            Arg::new("svg")
                .short('s')
                .long("svg")
                .help("Generate an SVG palette instead of JSON (output file path)"),
        )
        .get_matches();

    let k = 16;
    //let k = matches.get_one::<usize>("colors").unwrap().to_owned();
    let input_path: &String = matches.get_one("input").unwrap();
    let svg_path = matches.get_one::<String>("svg");

    // Load image
    let img = image::open(&input_path)?;
    let (width, height) = img.dimensions();

    // Gather all pixels into a vector (in float form)
    let mut pixels = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            // Convert to f32 for K-means
            pixels.push(RgbColor {
                r: pixel[0] as f32,
                g: pixel[1] as f32,
                b: pixel[2] as f32,
            });
        }
    }

    // Run K-means
    let clusters = k_means(&pixels, k, 10)?;

    // Format the output
    if let Some(output) = svg_path {
        // Generate an SVG
        generate_svg(&clusters, output)?;
        println!("SVG palette generated at: {}", output);
    } else {
        // Generate JSON array
        let hex_values: Vec<String> = clusters.iter().map(|c| c.to_hex()).collect();
        let json_output = serde_json::to_string_pretty(&hex_values)?;
        println!("{}", json_output);
    }

    Ok(())
}

/// * `data` is the list of all pixel colors.
/// * `k` is the number of clusters.
/// * `max_iterations` is the maximum number of iterations for K-means.
///
/// Returns a vector of length `k` containing the final cluster centers.
fn k_means(
    data: &[RgbColor],
    k: usize,
    max_iterations: usize,
) -> Result<Vec<RgbColor>, Box<dyn Error>> {
    if data.is_empty() {
        return Err("No data found in the image.".into());
    }

    if k == 0 {
        return Err("Number of clusters (k) must be > 0.".into());
    }

    // Initialize cluster centers randomly from the data
    let mut rng = thread_rng();
    let mut centers = Vec::new();
    for _ in 0..k {
        let random_pixel = data[rng.gen_range(0..data.len())];
        centers.push(random_pixel);
    }

    // Repeatedly assign points to the nearest cluster and recalc centers
    for _ in 0..max_iterations {
        // Create k buckets to hold assigned pixels
        let mut clusters = vec![Vec::new(); k];

        // Assign each pixel to the closest center
        for &pixel in data {
            let mut min_dist = f32::MAX;
            let mut closest_center = 0;
            for (i, &center) in centers.iter().enumerate() {
                let dist = pixel.distance_squared(&center);
                if dist < min_dist {
                    min_dist = dist;
                    closest_center = i;
                }
            }
            clusters[closest_center].push(pixel);
        }

        // Recompute centers
        let mut new_centers = Vec::with_capacity(k);
        for cluster in clusters {
            if cluster.is_empty() {
                // If a cluster is empty, pick a random data point as the new center
                new_centers.push(data[rng.gen_range(0..data.len())]);
            } else {
                let sum_r: f32 = cluster.iter().map(|c| c.r).sum();
                let sum_g: f32 = cluster.iter().map(|c| c.g).sum();
                let sum_b: f32 = cluster.iter().map(|c| c.b).sum();
                let count = cluster.len() as f32;
                new_centers.push(RgbColor {
                    r: sum_r / count,
                    g: sum_g / count,
                    b: sum_b / count,
                });
            }
        }
        centers = new_centers;
    }

    Ok(centers)
}

/// Generate an SVG file displaying the colors along with their hex codes.
fn generate_svg(colors: &[RgbColor], output_path: &str) -> Result<(), Box<dyn Error>> {
    let swatch_width = 100;
    let swatch_height = 60;
    let spacing = 10;
    let font_size = 14;

    // Overall width = swatch_width, but we have k swatches stacked vertically
    let total_width = swatch_width;
    let total_height = (swatch_height + spacing) * colors.len() as i32;

    let mut svg_content = format!(
        r#"<svg width="{width}" height="{height}" xmlns="http://www.w3.org/2000/svg">
<style>
    .label {{ font-family: sans-serif; font-size: {font_size}px; fill: #000; }}
</style>
"#,
        width = total_width,
        height = total_height,
        font_size = font_size
    );

    for (i, color) in colors.iter().enumerate() {
        let y_offset = i as i32 * (swatch_height + spacing);
        let rect = format!(
            r#"<rect x="0" y="{y}" width="{w}" height="{h}" fill="{color}" />"#,
            y = y_offset,
            w = swatch_width,
            h = swatch_height,
            color = color.to_hex()
        );
        let text = format!(
            r#"<text x="{x}" y="{y}" class="label">{text}</text>"#,
            x = 5,
            y = y_offset + swatch_height / 2 + (font_size / 2),
            text = color.to_hex()
        );
        svg_content.push_str(&rect);
        svg_content.push_str("\n");
        svg_content.push_str(&text);
        svg_content.push_str("\n");
    }

    svg_content.push_str("</svg>\n");

    let mut file = File::create(Path::new(output_path))?;
    file.write_all(svg_content.as_bytes())?;

    Ok(())
}
