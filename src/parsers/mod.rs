mod json;
mod svg;

use std::{error::Error, str::FromStr};

pub use json::*;
pub use svg::*;

use crate::HexColor;

pub trait PalleteParser {
    fn parse(pallete: &[HexColor], out_file_path: Option<&str>) -> Result<(), Box<dyn Error>>;
}

pub enum OutputFormat {
    Svg,
    Json,
}

impl FromStr for OutputFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "svg" => Ok(OutputFormat::Svg),
            "json" => Ok(OutputFormat::Json),
            _ => Err("Failed to parse Output Format "),
        }
    }
}
