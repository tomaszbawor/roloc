use std::error::Error;

use rand::{rng, seq::IndexedRandom, Rng};

use crate::RgbColor;

pub fn k_means(
    all_pixels: &[RgbColor],
    k: usize,
    max_iterations: usize,
) -> Result<Vec<RgbColor>, Box<dyn Error>> {
    if all_pixels.is_empty() {
        return Err("No data found in the image.".into());
    }
    if k == 0 {
        return Err("Number of clusters (k) must be > 0.".into());
    }

    let mut rng = rng();

    // 1. Randomly initialize centers
    let mut centers: Vec<RgbColor> = all_pixels.choose_multiple(&mut rng, k).cloned().collect();

    // 2. Repeatedly assign points and recalc centers
    for _ in 0..max_iterations {
        let mut clusters = vec![Vec::new(); k];

        for &pixel in all_pixels {
            let mut min_distance = f32::MAX;
            let mut closest_center_index = 0;

            for (i, &center) in centers.iter().enumerate() {
                let dist = pixel.distance_squared(&center);
                if dist < min_distance {
                    min_distance = dist;
                    closest_center_index = i;
                }
            }
            clusters[closest_center_index].push(pixel);
        }

        let mut new_centers = Vec::with_capacity(k);
        for cluster in clusters {
            if cluster.is_empty() {
                // If a cluster is empty, pick a random data point as the new center
                new_centers.push(all_pixels[rng.random_range(0..all_pixels.len())]);
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
