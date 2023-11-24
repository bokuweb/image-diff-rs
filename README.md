# image-diff-rs

``` sh
$ cd wasm && cargo rustc --target=wasm32-unknown-emscripten -- -C link-args="-s EXPORTED_FUNCTIONS=['_diff'] -s STANDALONE_WASM -s ERROR_ON_UNDEFINED_SYMBOLS=0 -s --no-entry"
```

``` sh
AR=llvm-ar CFLAGS='--sysroot ../wasi-sdk/share/wasi-sysroot' cargo wasi build --release
```

``` sh
wasm-tools component new target/wasm32-wasi/release/image_diff_wasm.wasm -o wasm/component.wasm --adapt wasm/wasi_snapshot_preview1.wasm
```


``` sh
jco transpile component.wasm -o js --name index && mv js/index.js js/index.mjs
```

```
wasm-tools component new target/wasm32-wasi/release/rayon_test.wasm -o component.wasm --adapt wasi_snapshot_preview1.wasm 
```

```
wasm-tools --version
wasm-tools 1.0.53
```