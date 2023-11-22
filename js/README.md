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
import { fileURLToPath } from "node:url";
import { strictEqual } from "node:assert";

import path from "node:path";

import { diff } from "../index.mjs";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const patha = path.resolve(__dirname, "../../fixtures/sample0.webp");
const imga = await readFile(patha);

const pathb = path.resolve(__dirname, "../../fixtures/sample1.webp");
const imgb = await readFile(pathb);

const result = diff(imga, imgb, { enableAntiAlias: true, threshold: 0.01 });
```

## API

### diff(imga: Uint8Array, imgb: Uint8Array, opts: Opts): Output;

## License

JS glue is provided under the MIT License, and the libwebp is provided by Google under the BSD 3-Clause License.
