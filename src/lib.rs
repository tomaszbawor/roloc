pub mod kmeans;

pub use kmeans::k_means;

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct HexColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<HexColor> for String {
    fn from(value: HexColor) -> Self {
        format!("#{:02X}{:02X}{:02X}", value.r, value.g, value.b)
    }
}

impl From<&RgbColor> for HexColor {
    fn from(value: &RgbColor) -> Self {
        Self {
            r: value.r as u8,
            g: value.g as u8,
            b: value.b as u8,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct RgbColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl RgbColor {
    pub fn distance_squared(&self, other: &RgbColor) -> f32 {
        (self.r - other.r).powi(2) + (self.g - other.g).powi(2) + (self.b - other.b).powi(2)
    }
}
