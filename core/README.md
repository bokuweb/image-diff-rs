# image-diff-rs

<img src="https://github.com/bokuweb/image-diff-rs/workflows/Continuous%20Integration/badge.svg" alt="Build Status" />

This library is an image differencing crate that supports PNG, JPEG, GIF, TIFF, and WebP formats.

## installation

```
cargo add image-diff-rs
```

## examples

```js
import { readFile } from "node:fs/promises";
import { diff } from "@bokuweb/image-diff-wasm";

const imga = await readFile(PATH_TO_IMAGE_A);
const imgb = await readFile(PATH_TO_IMAGE_B);

const result = diff(imga, imgb, { enableAntiAlias: true, threshold: 0.01 });
```

## API

### diff(imga: Uint8Array, imgb: Uint8Array, opts: Opts): Output;

The diff function is designed to compare two images and identify their differences.
It takes two image buffers as input and returns an `Output` object containing information about the differences.

#### Input

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

#### Output

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

#### Error

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

## License

JS glue is provided under the MIT License, and the libwebp is provided by Google under the BSD 3-Clause License.
