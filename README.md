# image-diff-rs

<img src="https://github.com/bokuweb/image-diff-rs/workflows/Continuous%20Integration/badge.svg" alt="Build Status" />

This project provides an image differencing library that supports `PNG`,`JPEG`,`GIF`,`TIFF`,and `WebP` formats for `Node.js`, `Deno`, and `Rust`. For more details, please refer to each respective directory.

The code for `Node.js` and `Deno` is generated using `wit-bindgen` and `jco`.

- JS: https://github.com/bokuweb/image-diff-rs/tree/main/js
- Rust: https://github.com/bokuweb/image-diff-rs/tree/main/core


## Generate JS code.

```sh
AR=llvm-ar CFLAGS='--sysroot ../wasi-sdk/share/wasi-sysroot' cargo wasi build --release
```

```sh
wasm-tools component new target/wasm32-wasi/release/image_diff_wasm.wasm -o wasm/component.wasm --adapt wasm/wasi_snapshot_preview1.wasm
```

```sh
jco transpile wasm/component.wasm -o js --name index && mv js/index.js js/index.mjs
```
