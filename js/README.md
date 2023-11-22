# @bokuweb/image-diff-wasm

<img src="https://github.com/bokuweb/image-diff-rs/workflows/Continuous%20Integration/badge.svg" alt="Build Status" />

Image diff library for Node.js and Deno.

## installation

```
npm install @bokuweb/image-diff-wasm
```

## examples

```js
import { readFile } from "node:fs/promises";
import { diff } from "../index.mjs";

const imga = await readFile(PATH_TO_IMAGE_A);
const imgb = await readFile(PATH_TO_IMAGE_B);

const result = diff(imga, imgb, { enableAntiAlias: true, threshold: 0.01 });
```

## API

### diff(imga: Uint8Array, imgb: Uint8Array, opts: Opts): Output;

``` Typescript
export interface Output {
  diffCount: number,
  diffImage: Uint8Array,
  width: number,
  height: number,
}
```

## License

JS glue is provided under the MIT License, and the libwebp is provided by Google under the BSD 3-Clause License.
