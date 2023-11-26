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

## License

Rust glue is provided under the MIT License, and the libwebp is provided by Google under the BSD 3-Clause License.
