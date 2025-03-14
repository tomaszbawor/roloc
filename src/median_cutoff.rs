use std::{collections::VecDeque, error::Error};

use crate::{HexColor, RgbColor};

pub fn median_cut(all_pixels: &[RgbColor], k: usize) -> Result<Vec<HexColor>, Box<dyn Error>> {
    if all_pixels.is_empty() {
        return Err("No data found in the image.".into());
    }
    if k == 0 {
        return Err("Number of clusters (k) must be > 0.".into());
    }

    let hex_pixels: Vec<HexColor> = all_pixels.iter().map(|c| HexColor::from(c)).collect();
    let mut queue: VecDeque<Vec<HexColor>> = VecDeque::new();

    queue.push_back(hex_pixels);

    while queue.len() < k {
        if let Some(group_to_split) = queue.pop_front() {
            let (first_half, second_half) = split_pixels_at_median(group_to_split);
            queue.push_back(first_half);
            queue.push_back(second_half);
        }
    }

    let res: Vec<HexColor> = queue
        .iter()
        .map(|group| average_color_from_group(group))
        .collect();

    Ok(res)
}

fn average_color_from_group(group: &Vec<HexColor>) -> HexColor {
    let mut red_sum = 0u32;
    let mut green_sum = 0u32;
    let mut blue_sum = 0u32;

    for color in group {
        red_sum = red_sum + color.r as u32;
        green_sum = green_sum + color.g as u32;
        blue_sum = blue_sum + color.b as u32;
    }

    HexColor {
        r: (red_sum / group.len() as u32) as u8,
        g: (green_sum / group.len() as u32) as u8,
        b: (blue_sum / group.len() as u32) as u8,
    }
}

fn split_pixels_at_median(mut pixels: Vec<HexColor>) -> (Vec<HexColor>, Vec<HexColor>) {
    let [HexColor {
        r: min_r,
        g: min_g,
        b: min_b,
    }, HexColor {
        r: max_r,
        g: max_g,
        b: max_b,
    }] = color_min_max(&pixels);

    let range_r = max_r - min_r;
    let range_g = max_g - min_g;
    let range_b = max_b - min_b;

    let sort_channel = if range_r >= range_g && range_r >= range_b {
        |p: &HexColor| p.r
    } else if range_g >= range_r && range_g >= range_b {
        |p: &HexColor| p.g
    } else {
        |p: &HexColor| p.b
    };

    pixels.sort_by_key(sort_channel);

    let median = pixels.len() / 2;
    let new_group = pixels.split_off(median);
    (pixels, new_group)
}

fn color_min_max(pixels: &[HexColor]) -> [HexColor; 2] {
    let mut maximum: HexColor = HexColor {
        r: u8::MIN,
        g: u8::MIN,
        b: u8::MIN,
    };

    let mut minimum: HexColor = HexColor {
        r: u8::MAX,
        g: u8::MAX,
        b: u8::MAX,
    };

    for pixel in pixels {
        maximum.r = pixel.r.max(maximum.r);
        maximum.g = pixel.r.max(maximum.g);
        maximum.b = pixel.r.max(maximum.b);

        minimum.r = pixel.r.min(minimum.r);
        minimum.g = pixel.r.min(minimum.g);
        minimum.b = pixel.r.min(minimum.b);
    }

    [minimum, maximum]
}
