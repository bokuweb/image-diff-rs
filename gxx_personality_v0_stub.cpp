// https://stackoverflow.com/questions/67474533/error-in-compiling-rustemcc%20-c%20gxx_personality_v0_stub.cpp-into-webassembly-using-emscripten-on-windows
#include "unwind.h"
#include "stdint.h"

extern "C" _Unwind_Reason_Code __gxx_personality_v0(int version, _Unwind_Action actions, uint64_t exceptionClass, _Unwind_Exception* unwind_exception, _Unwind_Context* context) {
    return  _URC_NO_REASON;
}