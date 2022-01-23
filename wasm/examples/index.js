const m = require("../../target/wasm32-unknown-emscripten/debug/image-diff-wasm");

m.run();
m.onRuntimeInitialized = () => {
  m._compare();
};
