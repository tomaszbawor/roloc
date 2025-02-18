use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{prelude::*, rng};
use roloc::{k_means, RgbColor};

fn generate_random_pixels(size: usize) -> Vec<RgbColor> {
    let mut rng = rng();
    let mut data = Vec::with_capacity(size);
    for _ in 0..size {
        data.push(RgbColor {
            r: rng.random_range(0..=255) as f32,
            g: rng.random_range(0..=255) as f32,
            b: rng.random_range(0..=255) as f32,
        });
    }
    data
}

fn kmeans_benchmark(c: &mut Criterion) {
    let sizes = [1_000, 5_000, 10_000, 20_000];

    let k = 5;
    let max_iterations = 10;

    let mut group = c.benchmark_group("kmeans_bench");

    for &size in &sizes {
        // Generate random data of length `size`.
        let data = generate_random_pixels(size);

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &_size| {
            b.iter(|| {
                // black_box prevents the compiler from optimizing away unused results
                let _ = k_means(black_box(&data), black_box(k), black_box(max_iterations));
            });
        });
    }

    group.finish();
}

criterion_group!(benches, kmeans_benchmark);
criterion_main!(benches);
