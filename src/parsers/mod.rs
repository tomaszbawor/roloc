mod svg;

use std::error::Error;

pub use svg::*;

use crate::HexColor;

pub trait PalleteParser {
    fn parse(pallete: &[HexColor], out_file_path: Option<&str>) -> Result<(), Box<dyn Error>>;
}
