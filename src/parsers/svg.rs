use std::{error::Error, fs::File, io::Write, path::Path};

use crate::HexColor;

use super::PalleteParser;

pub struct SvgParser {}

impl PalleteParser for SvgParser {
    fn parse(pallete: &[HexColor], out_file_path: Option<&str>) -> Result<(), Box<dyn Error>> {
        let swatch_width = 100;
        let swatch_height = 60;
        let spacing = 10;
        let font_size = 14;

        // Overall width = swatch_width, but we have k swatches stacked vertically
        let total_width = swatch_width;
        let total_height = (swatch_height + spacing) * pallete.len() as i32;

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

        for (i, color) in pallete.iter().enumerate() {
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
            svg_content.push('\n');
            svg_content.push_str(&text);
            svg_content.push('\n');
        }

        svg_content.push_str("</svg>\n");

        if let Some(path) = out_file_path {
            let mut file = File::create(Path::new(path))?;
            file.write_all(svg_content.as_bytes())?;
        } else {
            println!("{}", svg_content)
        }

        Ok(())
    }
}
