# image-diff-rs

<img src="https://github.com/bokuweb/image-diff-rs/workflows/Continuous%20Integration/badge.svg" alt="Build Status" />

This project provides an image differencing library that supports `PNG`,`JPEG`,`GIF`,`TIFF`,and `WebP` formats for `Node.js`, `Deno`, and `Rust`. For more details, please refer to each respective directory.

The code for `Node.js` and `Deno` is generated using `wit-bindgen` and `jco`.

- JS: https://github.com/bokuweb/image-diff-rs/tree/main/js
- Wasm: https://github.com/bokuweb/image-diff-rs/tree/main/wasm
- Rust: https://github.com/bokuweb/image-diff-rs/tree/main/core

## Demo

| img1     | img2         | diff       |
| --------------- |---------------| -------------------- |
| ![](https://github.com/bokuweb/pixelmatch-rs/raw/main/fixtures/001a.png) | ![](https://github.com/bokuweb/pixelmatch-rs/raw/main/fixtures/001b.png) |![](https://github.com/bokuweb/pixelmatch-rs/raw/main/assets/diff1.png)|

## JavaScript


### installation

```
npm install @bokuweb/image-diff-wasm
```

### examples

```js
import { readFile } from "node:fs/promises";
import { diff } from "@bokuweb/image-diff-wasm";

const imga = await readFile(PATH_TO_IMAGE_A);
const imgb = await readFile(PATH_TO_IMAGE_B);

const result = diff(imga, imgb, { enableAntiAlias: true, threshold: 0.01 });
```

### API

#### diff(imga: Uint8Array, imgb: Uint8Array, opts: Opts): Output;

The diff function is designed to compare two images and identify their differences.
It takes two image buffers as input and returns an `Output` object containing information about the differences.

##### Input

- `imga`: Uint8Array: The first image buffer.
- `imgb`: Uint8Array: The second image buffer.
- `opts`: Opts: Options object for the function.

```Typescript
export interface Opts {
  threshold?: number,
  includeAntiAlias?: boolean,
}
```

- `threshold`: Matching threshold, ranges from 0 to 1. Smaller values make the comparison more sensitive. 0.1 by default.
- `includeAntiAlias`: The flag of antialias. If omitted false.

##### Output

```Typescript
export interface Output {
  diffCount: number,
  diffImage: Uint8Array,
  width: number,
  height: number,
}
```

- `diffCount`: The number of pixels that differ between the two images.
- `diffImage`: The buffer of the difference image in `WebP` format.
- `width`: The width of the difference image.
- `height`: The height of the difference image.

##### Error

The function may throw following values as `ComponentError`.

```Typescript
export type Error = ErrorDecode | ErrorEncode;
export interface ErrorDecode {
  tag: 'decode',
  val: string,
}
export interface ErrorEncode {
  tag: 'encode',
  val: string,
}
```

## Rust

### examples

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

``` sh
cargo run --example compare
```

## Wasm

### Generate JS code from wasm component.

```sh
AR=llvm-ar CFLAGS='--sysroot ../wasi-sdk/share/wasi-sysroot' cargo wasi build --release
```

```sh
wasm-tools component new target/wasm32-wasi/release/image_diff_wasm.wasm -o wasm/component.wasm --adapt wasm/wasi_snapshot_preview1.wasm
```

```sh
jco transpile wasm/component.wasm -o js --name index && mv js/index.js js/index.mjs
```
