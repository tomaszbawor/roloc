use clap::{Arg, Command};

use image::GenericImageView;
use roloc::{median_cut, HexColor, PalleteParser, SvgParser};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("roloc")
        .version("0.0.1")
        .author("Tomasz Bawor <[email protected]>")
        .about("Extracts a color palette from an image")
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

    let colors_count: &String = matches.get_one::<String>("colors").unwrap();
    let k = colors_count.parse::<usize>()?;
    let input_path: &String = matches.get_one("input").unwrap();
    let svg_path = matches.get_one::<String>("svg");

    // Load image
    let img = image::open(input_path)?;
    let (width, height) = img.dimensions();

    // Gather all pixels into a vector (in float form)
    let mut pixels: Vec<HexColor> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);

            pixels.push(HexColor {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
            });
        }
    }

    let clusters = median_cut(&pixels, k)?;

    SvgParser::parse(&clusters, svg_path.map(|x| x.as_str()))?;

    Ok(())
}
