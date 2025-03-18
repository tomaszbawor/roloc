mod generators;
mod parsers;

pub use generators::*;
pub use parsers::*;

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub struct HexColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct LabColor {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

impl From<&HexColor> for LabColor {
    fn from(value: &HexColor) -> Self {
        // Step 1: Convert RGB (0-255) to Linear RGB (0-1)
        let r = value.r as f32 / 255.0;
        let g = value.g as f32 / 255.0;
        let b = value.b as f32 / 255.0;

        // Step 2: Apply gamma correction (sRGB to linear RGB)
        let linearize = |v: f32| -> f32 {
            if v > 0.04045 {
                ((v + 0.055) / 1.055).powf(2.4)
            } else {
                v / 12.92
            }
        };

        let r = linearize(r);
        let g = linearize(g);
        let b = linearize(b);

        // Step 3: Convert RGB to XYZ color space
        let x = (r * 0.4124564 + g * 0.3575761 + b * 0.1804375) / 0.95047;
        let y = (r * 0.2126729 + g * 0.7151522 + b * 0.0721750) / 1.00000;
        let z = (r * 0.0193339 + g * 0.1191920 + b * 0.9503041) / 1.08883;

        // Step 4: Convert XYZ to LAB
        let f = |t: f32| -> f32 {
            if t > 0.008856 {
                t.powf(1.0 / 3.0)
            } else {
                (7.787 * t) + (16.0 / 116.0)
            }
        };

        let fx = f(x);
        let fy = f(y);
        let fz = f(z);

        let l = (116.0 * fy) - 16.0;
        let a = 500.0 * (fx - fy);
        let b = 200.0 * (fy - fz);

        Self { l, a, b }
    }
}

impl From<&HexColor> for String {
    fn from(value: &HexColor) -> Self {
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
