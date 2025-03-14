use clap::{Arg, Command};

use image::GenericImageView;
use roloc::{k_means, median_cut, median_cutoff, HexColor, RgbColor};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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

    let clusters = median_cut(&pixels, k)?;

    // Format the output
    if let Some(output) = svg_path {
        // Generate an SVG
        generate_svg(&clusters, output)?;
        println!("SVG palette generated at: {}", output);
    } else {
        // Generate JSON array
        let hex_values: Vec<String> = clusters.iter().map(|d| String::from(d)).collect();
        let json_output = serde_json::to_string_pretty(&hex_values)?;
        println!("{}", json_output);
    }

    Ok(())
}

/// Generate an SVG file displaying the colors along with their hex codes.
fn generate_svg(colors: &[HexColor], output_path: &str) -> Result<(), Box<dyn Error>> {
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
            color = String::from(color)
        );
        let text = format!(
            r#"<text x="{x}" y="{y}" class="label">{text}</text>"#,
            x = 5,
            y = y_offset + swatch_height / 2 + (font_size / 2),
            text = String::from(color)
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
