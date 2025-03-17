use super::PalleteParser;

pub struct JsonParser {}

impl PalleteParser for JsonParser {
    fn parse(
        pallete: &[crate::HexColor],
        out_file_path: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json: Vec<String> = pallete.iter().map(String::from).collect();

        println!("{:?}", json);
        Ok(())
    }
}
