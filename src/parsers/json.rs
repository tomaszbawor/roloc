use std::{fs::File, io::Write, path::Path};

use super::PalleteParser;

pub struct JsonParser {}

impl PalleteParser for JsonParser {
    fn parse(
        pallete: &[crate::HexColor],
        out_file_path: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json: Vec<String> = pallete.iter().map(String::from).collect();

        let result = serde_json::to_string_pretty(&json).unwrap();

        if let Some(path) = out_file_path {
            let mut file = File::create(Path::new(path))?;
            file.write_all(result.as_bytes())?;
        } else {
            println!("{}", result);
        }

        Ok(())
    }
}
