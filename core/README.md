# image-diff-rs

<img src="https://github.com/bokuweb/image-diff-rs/workflows/Continuous%20Integration/badge.svg" alt="Build Status" />

This library is an image differencing crate that supports PNG, JPEG, GIF, TIFF, and WebP formats.

## installation

```
cargo add image-diff-rs
```

## examples

```Rust
pub fn main() {
    let imga = std::fs::read("../fixtures/sample0.webp").unwrap();
    let imgb = std::fs::read("../fixtures/sample1.webp").unwrap();

    let _result = diff(
        imga,
        imgb,
        &DiffOption {
            threshold: Some(0.01),
            include_anti_alias: Some(true),
        },
    )
    .unwrap();
}
```

If the caller only needs a diff image after applying its own acceptance
threshold, compare first and encode conditionally:

```Rust
let rgba_diff = diff_rgba(imga, imgb, &options)?;

if rgba_diff.diff_count > accepted_pixel_count {
    let output = encode_diff(&rgba_diff, EncodeFormat::Webp)?;
    // Write the encoded image from `output`.
}
```

`diff_rgba` returns the diff count, dimensions, and unencoded RGBA pixels.
The existing `diff` function remains the convenient compare-and-encode API.

``` sh
cargo run --example compare
```

## Demo

| img1     | img2         | diff       |
| --------------- |---------------| -------------------- |
| ![](https://github.com/bokuweb/pixelmatch-rs/raw/main/fixtures/001a.png) | ![](https://github.com/bokuweb/pixelmatch-rs/raw/main/fixtures/001b.png) |![](https://github.com/bokuweb/pixelmatch-rs/raw/main/assets/diff1.png)|

## License

Rust glue is provided under the MIT License, and the libwebp is provided by Google under the BSD 3-Clause License.
