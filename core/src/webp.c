#ifdef __EMSCRIPTEN__
  #include "emscripten.h"
#else
  #define EMSCRIPTEN_KEEPALIVE
#endif

#include "../../libwebp/src/webp/encode.h"
#include "../../libwebp/src/webp/decode.h"

EMSCRIPTEN_KEEPALIVE
int version() {
  return WebPGetEncoderVersion();
}

EMSCRIPTEN_KEEPALIVE
uint8_t* decode(const uint8_t* data, size_t data_size, int* width, int* height) {
  return WebPDecodeRGBA(data, data_size, width, height);
}