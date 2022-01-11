#ifdef __EMSCRIPTEN__
  #include "emscripten.h"
#else
  #define EMSCRIPTEN_KEEPALIVE
#endif

#include "../libwebp/src/webp/encode.h"
#include "../libwebp/src/webp/decode.h"

EMSCRIPTEN_KEEPALIVE
extern "C" {
  int version() {
    return WebPGetEncoderVersion();
  }

  uint8_t* decode(const uint8_t* data, size_t data_size, int* width, int* height) {
    return WebPDecodeRGBA(data, data_size, width, height);
  }

  int add_one(int x) {
    return x + 1;
  }
}

