use std::error::Error;

use rand::{rng, seq::IndexedRandom, Rng};

/// Publicly define your RgbColor struct so it can be used externally.
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

    pub fn to_hex(&self) -> String {
        let r = self.r as u8;
        let g = self.g as u8;
        let b = self.b as u8;
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
}

/// Publicly define your k_means function.
pub fn k_means(
    data: &[RgbColor],
    k: usize,
    max_iterations: usize,
) -> Result<Vec<RgbColor>, Box<dyn Error>> {
    if data.is_empty() {
        return Err("No data found in the image.".into());
    }
    if k == 0 {
        return Err("Number of clusters (k) must be > 0.".into());
    }

    let mut rng = rng();

    // 1. Randomly initialize centers
    let mut centers: Vec<RgbColor> = data.choose_multiple(&mut rng, k).cloned().collect();

    // 2. Repeatedly assign points and recalc centers
    for _ in 0..max_iterations {
        let mut clusters = vec![Vec::new(); k];

        for &pixel in data {
            let mut min_dist = f32::MAX;
            let mut closest_center = 0;

            for (i, &center) in centers.iter().enumerate() {
                let dist = pixel.distance_squared(&center);
                if dist < min_dist {
                    min_dist = dist;
                    closest_center = i;
                }
            }
            clusters[closest_center].push(pixel);
        }

        let mut new_centers = Vec::with_capacity(k);
        for cluster in clusters {
            if cluster.is_empty() {
                // If a cluster is empty, pick a random data point as the new center
                new_centers.push(data[rng.random_range(0..data.len())]);
            } else {
                let sum_r: f32 = cluster.iter().map(|c| c.r).sum();
                let sum_g: f32 = cluster.iter().map(|c| c.g).sum();
                let sum_b: f32 = cluster.iter().map(|c| c.b).sum();
                let count = cluster.len() as f32;
                new_centers.push(RgbColor {
                    r: sum_r / count,
                    g: sum_g / count,
                    b: sum_b / count,
                });
            }
        }
        centers = new_centers;
    }

    Ok(centers)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_squared() {
        let c1 = RgbColor {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
        let c2 = RgbColor {
            r: 10.0,
            g: 0.0,
            b: 0.0,
        };
        assert_eq!(c1.distance_squared(&c2), 100.0);
    }

    #[test]
    fn test_k_means_empty_data() {
        let data = vec![];
        let result = k_means(&data, 3, 10);
        assert!(result.is_err(), "Expected an error for empty data");
    }
}
