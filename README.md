# image-diff-rs

``` sh
$ cd wasm && cargo rustc --bin image-diff-wasm --target=wasm32-unknown-emscripten -- -C link-args="-s EXPORTED_FUNCTIONS=['_compare','_main'] -s STANDALONE_WASM"
```