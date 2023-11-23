# image-diff-rs

``` sh
$ cd wasm && cargo rustc --target=wasm32-unknown-emscripten -- -C link-args="-s EXPORTED_FUNCTIONS=['_diff'] -s STANDALONE_WASM -s ERROR_ON_UNDEFINED_SYMBOLS=0 -s --no-entry"
```

``` sh
AR=llvm-ar CFLAGS='--sysroot ../wasi-sdk/share/wasi-sysroot' cargo wasi build --release
```


``` sh
jco transpile component.wasm -o js --name index && mv js/index.js js/index.mjs
```