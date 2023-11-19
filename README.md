# image-diff-rs

``` sh
$ cd wasm && cargo rustc --target=wasm32-unknown-emscripten -- -C link-args="-s EXPORTED_FUNCTIONS=['_diff'] -s STANDALONE_WASM -s ERROR_ON_UNDEFINED_SYMBOLS=0 -s --no-entry"
```

```
export WASI_VERSION=20
export WASI_VERSION_FULL=${WASI_VERSION}.0
```

```
cd wasm && CFLAGS='--sysroot ../wasi-sdk/share/wasi-sysroot' cargo wasi build
```

```
LLVM ERROR: malformed uleb128, extends past end
```

????

```
AR=llvm-ar CFLAGS='--sysroot ../wasi-sdk/share/wasi-sysroot' cargo wasi build --release
```

でうまく行った
