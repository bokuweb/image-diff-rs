#ifdef __EMSCRIPTEN__
  #include "emscripten.h"
#else
  #define EMSCRIPTEN_KEEPALIVE
#endif

#include "../../libwebp/src/webp/encode.h"
#include "../../libwebp/src/webp/decode.h"

EMSCRIPTEN_KEEPALIVE
uint8_t* decode(const uint8_t* data, size_t data_size, int* width, int* height) {
  return WebPDecodeRGBA(data, data_size, width, height);
}

EMSCRIPTEN_KEEPALIVE
size_t encode_lossless(const uint8_t* rgba, int width, int height, int stride, uint8_t** output) {
  return WebPEncodeLosslessRGBA(rgba, width, height, stride, output);
}

EMSCRIPTEN_KEEPALIVE
size_t encode(const uint8_t* rgba, int width, int height, int stride, double quality, uint8_t** output) {
  return WebPEncodeRGBA(rgba, width, height, stride, quality,  output);
}