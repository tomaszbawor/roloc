# Roloc

## Development

Enter nix shell in order to download all needed tools

```bash
nix develop
```

## Commits
You can make a commit by using commitizen to follow conventional commits

```bash
cz commmit
```

## Running
To run application you must execute 

```bash
cargo run -- {path_to_img}
```

# Pallete extraction algorithms by Chat GPT

1. Histogram (Most Frequent Colors) – Counts pixel occurrences and selects the most common colors.

✅ Fast, exact colors from the image.

❌ Sensitive to noise and large uniform areas.

2. K-means Clustering – Groups similar colors into a fixed number of clusters.

✅ Produces aesthetically pleasing palettes.

❌ Slower, depends on initial cluster selection.

3. Octree Quantization – Builds a hierarchical color tree to find representative colors.

✅ Efficient for color reduction, used in image compression.

❌ May not work well with small palettes.

4. Median Cut – Recursively divides color space to select representative colors.

✅ Works well for diverse images.

❌ Sometimes produces unnatural results.

5. Mean Shift Clustering – Detects dense color regions without predefined cluster count.

✅ Adapts to image content dynamically.

❌ Slower and memory-intensive.

5. Self-Organizing Map (SOM) – Uses a neural network to map colors into clusters.

✅ Captures color variations effectively.

❌ Complex and requires more computation.

6. Principal Component Analysis (PCA) – Reduces color dimensions and extracts dominant variations.

✅ Removes noise, finds key color patterns.

❌ Less intuitive and harder to implement.

# Interesing reading 
- [Color quantization using octrees](http://www.leptonica.org/papers/colorquant.pdf)
- [Color quantization using modified median cut](https://web.archive.org/web/20190202154003/http://www.leptonica.com/papers/mediancut.pdf)
