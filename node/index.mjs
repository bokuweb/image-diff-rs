import { environment, exit as exit$1, stderr, stdin, stdout, terminalInput, terminalOutput, terminalStderr, terminalStdin, terminalStdout } from '@bytecodealliance/preview2-shim/cli';
import { preopens, types } from '@bytecodealliance/preview2-shim/filesystem';
import { streams } from '@bytecodealliance/preview2-shim/io';
import { random } from '@bytecodealliance/preview2-shim/random';
const { getEnvironment } = environment;
const { exit } = exit$1;
const { getStderr } = stderr;
const { getStdin } = stdin;
const { getStdout } = stdout;
const { TerminalInput } = terminalInput;
const { TerminalOutput } = terminalOutput;
const { getTerminalStderr } = terminalStderr;
const { getTerminalStdin } = terminalStdin;
const { getTerminalStdout } = terminalStdout;
const { getDirectories } = preopens;
const { Descriptor,
  DirectoryEntryStream,
  filesystemErrorCode } = types;
const { Error: Error$1,
  InputStream,
  OutputStream } = streams;
const { getRandomBytes } = random;

const base64Compile = str => WebAssembly.compile(typeof Buffer !== 'undefined' ? Buffer.from(str, 'base64') : Uint8Array.from(atob(str), b => b.charCodeAt(0)));

class ComponentError extends Error {
  constructor (value) {
    const enumerable = typeof value !== 'string';
    super(enumerable ? `${String(value)} (see error.payload)` : value);
    Object.defineProperty(this, 'payload', { value, enumerable });
  }
}

let dv = new DataView(new ArrayBuffer());
const dataView = mem => dv.buffer === mem.buffer ? dv : dv = new DataView(mem.buffer);

const isNode = typeof process !== 'undefined' && process.versions && process.versions.node;
let _fs;
async function fetchCompile (url) {
  if (isNode) {
    _fs = _fs || await import('fs/promises');
    return WebAssembly.compile(await _fs.readFile(url));
  }
  return fetch(url).then(WebAssembly.compileStreaming);
}

function getErrorPayload(e) {
  if (e && hasOwnProperty.call(e, 'payload')) return e.payload;
  return e;
}

const hasOwnProperty = Object.prototype.hasOwnProperty;

const instantiateCore = WebAssembly.instantiate;

const resourceHandleSymbol = Symbol('resource');

const symbolDispose = Symbol.dispose || Symbol.for('dispose');

const toUint64 = val => BigInt.asUintN(64, val);

function toUint32(val) {
  return val >>> 0;
}

const utf8Decoder = new TextDecoder();

const utf8Encoder = new TextEncoder();

let utf8EncodedLen = 0;
function utf8Encode(s, realloc, memory) {
  if (typeof s !== 'string') throw new TypeError('expected a string');
  if (s.length === 0) {
    utf8EncodedLen = 0;
    return 1;
  }
  let allocLen = 0;
  let ptr = 0;
  let writtenTotal = 0;
  while (s.length > 0) {
    ptr = realloc(ptr, allocLen, 1, allocLen + s.length);
    allocLen += s.length;
    const { read, written } = utf8Encoder.encodeInto(
    s,
    new Uint8Array(memory.buffer, ptr + writtenTotal, allocLen - writtenTotal),
    );
    writtenTotal += written;
    s = s.slice(read);
  }
  if (allocLen > writtenTotal)
  ptr = realloc(ptr, allocLen, 1, writtenTotal);
  utf8EncodedLen = writtenTotal;
  return ptr;
}

let exports0;
let exports1;

function trampoline8() {
  const ret = getStderr();
  if (!(ret instanceof OutputStream)) {
    throw new Error('Not a valid "OutputStream" resource.');
  }
  const handle0 = handleCnt0++;
  handleTable0.set(handle0, { rep: ret, own: true });
  return handle0;
}

function trampoline9(arg0) {
  let variant0;
  switch (arg0) {
    case 0: {
      variant0= {
        tag: 'ok',
        val: undefined
      };
      break;
    }
    case 1: {
      variant0= {
        tag: 'err',
        val: undefined
      };
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for expected');
    }
  }
  exit(variant0);
}

function trampoline10() {
  const ret = getStdin();
  if (!(ret instanceof InputStream)) {
    throw new Error('Not a valid "InputStream" resource.');
  }
  const handle0 = handleCnt2++;
  handleTable2.set(handle0, { rep: ret, own: true });
  return handle0;
}

function trampoline11() {
  const ret = getStdout();
  if (!(ret instanceof OutputStream)) {
    throw new Error('Not a valid "OutputStream" resource.');
  }
  const handle0 = handleCnt0++;
  handleTable0.set(handle0, { rep: ret, own: true });
  return handle0;
}
let exports2;

function trampoline12(arg0) {
  const ret = getDirectories();
  const vec3 = ret;
  const len3 = vec3.length;
  const result3 = realloc0(0, 0, 4, len3 * 12);
  for (let i = 0; i < vec3.length; i++) {
    const e = vec3[i];
    const base = result3 + i * 12;const [tuple0_0, tuple0_1] = e;
    if (!(tuple0_0 instanceof Descriptor)) {
      throw new Error('Not a valid "Descriptor" resource.');
    }
    const handle1 = handleCnt3++;
    handleTable3.set(handle1, { rep: tuple0_0, own: true });
    dataView(memory0).setInt32(base + 0, handle1, true);
    const ptr2 = utf8Encode(tuple0_1, realloc0, memory0);
    const len2 = utf8EncodedLen;
    dataView(memory0).setInt32(base + 8, len2, true);
    dataView(memory0).setInt32(base + 4, ptr2, true);
  }
  dataView(memory0).setInt32(arg0 + 4, len3, true);
  dataView(memory0).setInt32(arg0 + 0, result3, true);
}
let memory0;
let realloc0;

function trampoline13(arg0, arg1, arg2) {
  const handle1 = arg0;
  const rsc0 = handleTable3.get(handle1).rep;
  let ret;
  try {
    ret = { tag: 'ok', val: Descriptor.prototype.writeViaStream.call(rsc0, BigInt.asUintN(64, arg1)) };
  } catch (e) {
    ret = { tag: 'err', val: getErrorPayload(e) };
  }
  const variant4 = ret;
  switch (variant4.tag) {
    case 'ok': {
      const e = variant4.val;
      dataView(memory0).setInt8(arg2 + 0, 0, true);
      if (!(e instanceof OutputStream)) {
        throw new Error('Not a valid "OutputStream" resource.');
      }
      const handle2 = handleCnt0++;
      handleTable0.set(handle2, { rep: e, own: true });
      dataView(memory0).setInt32(arg2 + 4, handle2, true);
      break;
    }
    case 'err': {
      const e = variant4.val;
      dataView(memory0).setInt8(arg2 + 0, 1, true);
      const val3 = e;
      let enum3;
      switch (val3) {
        case 'access': {
          enum3 = 0;
          break;
        }
        case 'would-block': {
          enum3 = 1;
          break;
        }
        case 'already': {
          enum3 = 2;
          break;
        }
        case 'bad-descriptor': {
          enum3 = 3;
          break;
        }
        case 'busy': {
          enum3 = 4;
          break;
        }
        case 'deadlock': {
          enum3 = 5;
          break;
        }
        case 'quota': {
          enum3 = 6;
          break;
        }
        case 'exist': {
          enum3 = 7;
          break;
        }
        case 'file-too-large': {
          enum3 = 8;
          break;
        }
        case 'illegal-byte-sequence': {
          enum3 = 9;
          break;
        }
        case 'in-progress': {
          enum3 = 10;
          break;
        }
        case 'interrupted': {
          enum3 = 11;
          break;
        }
        case 'invalid': {
          enum3 = 12;
          break;
        }
        case 'io': {
          enum3 = 13;
          break;
        }
        case 'is-directory': {
          enum3 = 14;
          break;
        }
        case 'loop': {
          enum3 = 15;
          break;
        }
        case 'too-many-links': {
          enum3 = 16;
          break;
        }
        case 'message-size': {
          enum3 = 17;
          break;
        }
        case 'name-too-long': {
          enum3 = 18;
          break;
        }
        case 'no-device': {
          enum3 = 19;
          break;
        }
        case 'no-entry': {
          enum3 = 20;
          break;
        }
        case 'no-lock': {
          enum3 = 21;
          break;
        }
        case 'insufficient-memory': {
          enum3 = 22;
          break;
        }
        case 'insufficient-space': {
          enum3 = 23;
          break;
        }
        case 'not-directory': {
          enum3 = 24;
          break;
        }
        case 'not-empty': {
          enum3 = 25;
          break;
        }
        case 'not-recoverable': {
          enum3 = 26;
          break;
        }
        case 'unsupported': {
          enum3 = 27;
          break;
        }
        case 'no-tty': {
          enum3 = 28;
          break;
        }
        case 'no-such-device': {
          enum3 = 29;
          break;
        }
        case 'overflow': {
          enum3 = 30;
          break;
        }
        case 'not-permitted': {
          enum3 = 31;
          break;
        }
        case 'pipe': {
          enum3 = 32;
          break;
        }
        case 'read-only': {
          enum3 = 33;
          break;
        }
        case 'invalid-seek': {
          enum3 = 34;
          break;
        }
        case 'text-file-busy': {
          enum3 = 35;
          break;
        }
        case 'cross-device': {
          enum3 = 36;
          break;
        }
        default: {
          if ((e) instanceof Error) {
            console.error(e);
          }
          
          throw new TypeError(`"${val3}" is not one of the cases of error-code`);
        }
      }
      dataView(memory0).setInt8(arg2 + 4, enum3, true);
      break;
    }
    default: {
      throw new TypeError('invalid variant specified for result');
    }
  }
}

function trampoline14(arg0, arg1) {
  const handle1 = arg0;
  const rsc0 = handleTable3.get(handle1).rep;
  let ret;
  try {
    ret = { tag: 'ok', val: Descriptor.prototype.appendViaStream.call(rsc0) };
  } catch (e) {
    ret = { tag: 'err', val: getErrorPayload(e) };
  }
  const variant4 = ret;
  switch (variant4.tag) {
    case 'ok': {
      const e = variant4.val;
      dataView(memory0).setInt8(arg1 + 0, 0, true);
      if (!(e instanceof OutputStream)) {
        throw new Error('Not a valid "OutputStream" resource.');
      }
      const handle2 = handleCnt0++;
      handleTable0.set(handle2, { rep: e, own: true });
      dataView(memory0).setInt32(arg1 + 4, handle2, true);
      break;
    }
    case 'err': {
      const e = variant4.val;
      dataView(memory0).setInt8(arg1 + 0, 1, true);
      const val3 = e;
      let enum3;
      switch (val3) {
        case 'access': {
          enum3 = 0;
          break;
        }
        case 'would-block': {
          enum3 = 1;
          break;
        }
        case 'already': {
          enum3 = 2;
          break;
        }
        case 'bad-descriptor': {
          enum3 = 3;
          break;
        }
        case 'busy': {
          enum3 = 4;
          break;
        }
        case 'deadlock': {
          enum3 = 5;
          break;
        }
        case 'quota': {
          enum3 = 6;
          break;
        }
        case 'exist': {
          enum3 = 7;
          break;
        }
        case 'file-too-large': {
          enum3 = 8;
          break;
        }
        case 'illegal-byte-sequence': {
          enum3 = 9;
          break;
        }
        case 'in-progress': {
          enum3 = 10;
          break;
        }
        case 'interrupted': {
          enum3 = 11;
          break;
        }
        case 'invalid': {
          enum3 = 12;
          break;
        }
        case 'io': {
          enum3 = 13;
          break;
        }
        case 'is-directory': {
          enum3 = 14;
          break;
        }
        case 'loop': {
          enum3 = 15;
          break;
        }
        case 'too-many-links': {
          enum3 = 16;
          break;
        }
        case 'message-size': {
          enum3 = 17;
          break;
        }
        case 'name-too-long': {
          enum3 = 18;
          break;
        }
        case 'no-device': {
          enum3 = 19;
          break;
        }
        case 'no-entry': {
          enum3 = 20;
          break;
        }
        case 'no-lock': {
          enum3 = 21;
          break;
        }
        case 'insufficient-memory': {
          enum3 = 22;
          break;
        }
        case 'insufficient-space': {
          enum3 = 23;
          break;
        }
        case 'not-directory': {
          enum3 = 24;
          break;
        }
        case 'not-empty': {
          enum3 = 25;
          break;
        }
        case 'not-recoverable': {
          enum3 = 26;
          break;
        }
        case 'unsupported': {
          enum3 = 27;
          break;
        }
        case 'no-tty': {
          enum3 = 28;
          break;
        }
        case 'no-such-device': {
          enum3 = 29;
          break;
        }
        case 'overflow': {
          enum3 = 30;
          break;
        }
        case 'not-permitted': {
          enum3 = 31;
          break;
        }
        case 'pipe': {
          enum3 = 32;
          break;
        }
        case 'read-only': {
          enum3 = 33;
          break;
        }
        case 'invalid-seek': {
          enum3 = 34;
          break;
        }
        case 'text-file-busy': {
          enum3 = 35;
          break;
        }
        case 'cross-device': {
          enum3 = 36;
          break;
        }
        default: {
          if ((e) instanceof Error) {
            console.error(e);
          }
          
          throw new TypeError(`"${val3}" is not one of the cases of error-code`);
        }
      }
      dataView(memory0).setInt8(arg1 + 4, enum3, true);
      break;
    }
    default: {
      throw new TypeError('invalid variant specified for result');
    }
  }
}

function trampoline15(arg0, arg1) {
  const handle1 = arg0;
  const rsc0 = handleTable3.get(handle1).rep;
  let ret;
  try {
    ret = { tag: 'ok', val: Descriptor.prototype.getType.call(rsc0) };
  } catch (e) {
    ret = { tag: 'err', val: getErrorPayload(e) };
  }
  const variant4 = ret;
  switch (variant4.tag) {
    case 'ok': {
      const e = variant4.val;
      dataView(memory0).setInt8(arg1 + 0, 0, true);
      const val2 = e;
      let enum2;
      switch (val2) {
        case 'unknown': {
          enum2 = 0;
          break;
        }
        case 'block-device': {
          enum2 = 1;
          break;
        }
        case 'character-device': {
          enum2 = 2;
          break;
        }
        case 'directory': {
          enum2 = 3;
          break;
        }
        case 'fifo': {
          enum2 = 4;
          break;
        }
        case 'symbolic-link': {
          enum2 = 5;
          break;
        }
        case 'regular-file': {
          enum2 = 6;
          break;
        }
        case 'socket': {
          enum2 = 7;
          break;
        }
        default: {
          if ((e) instanceof Error) {
            console.error(e);
          }
          
          throw new TypeError(`"${val2}" is not one of the cases of descriptor-type`);
        }
      }
      dataView(memory0).setInt8(arg1 + 1, enum2, true);
      break;
    }
    case 'err': {
      const e = variant4.val;
      dataView(memory0).setInt8(arg1 + 0, 1, true);
      const val3 = e;
      let enum3;
      switch (val3) {
        case 'access': {
          enum3 = 0;
          break;
        }
        case 'would-block': {
          enum3 = 1;
          break;
        }
        case 'already': {
          enum3 = 2;
          break;
        }
        case 'bad-descriptor': {
          enum3 = 3;
          break;
        }
        case 'busy': {
          enum3 = 4;
          break;
        }
        case 'deadlock': {
          enum3 = 5;
          break;
        }
        case 'quota': {
          enum3 = 6;
          break;
        }
        case 'exist': {
          enum3 = 7;
          break;
        }
        case 'file-too-large': {
          enum3 = 8;
          break;
        }
        case 'illegal-byte-sequence': {
          enum3 = 9;
          break;
        }
        case 'in-progress': {
          enum3 = 10;
          break;
        }
        case 'interrupted': {
          enum3 = 11;
          break;
        }
        case 'invalid': {
          enum3 = 12;
          break;
        }
        case 'io': {
          enum3 = 13;
          break;
        }
        case 'is-directory': {
          enum3 = 14;
          break;
        }
        case 'loop': {
          enum3 = 15;
          break;
        }
        case 'too-many-links': {
          enum3 = 16;
          break;
        }
        case 'message-size': {
          enum3 = 17;
          break;
        }
        case 'name-too-long': {
          enum3 = 18;
          break;
        }
        case 'no-device': {
          enum3 = 19;
          break;
        }
        case 'no-entry': {
          enum3 = 20;
          break;
        }
        case 'no-lock': {
          enum3 = 21;
          break;
        }
        case 'insufficient-memory': {
          enum3 = 22;
          break;
        }
        case 'insufficient-space': {
          enum3 = 23;
          break;
        }
        case 'not-directory': {
          enum3 = 24;
          break;
        }
        case 'not-empty': {
          enum3 = 25;
          break;
        }
        case 'not-recoverable': {
          enum3 = 26;
          break;
        }
        case 'unsupported': {
          enum3 = 27;
          break;
        }
        case 'no-tty': {
          enum3 = 28;
          break;
        }
        case 'no-such-device': {
          enum3 = 29;
          break;
        }
        case 'overflow': {
          enum3 = 30;
          break;
        }
        case 'not-permitted': {
          enum3 = 31;
          break;
        }
        case 'pipe': {
          enum3 = 32;
          break;
        }
        case 'read-only': {
          enum3 = 33;
          break;
        }
        case 'invalid-seek': {
          enum3 = 34;
          break;
        }
        case 'text-file-busy': {
          enum3 = 35;
          break;
        }
        case 'cross-device': {
          enum3 = 36;
          break;
        }
        default: {
          if ((e) instanceof Error) {
            console.error(e);
          }
          
          throw new TypeError(`"${val3}" is not one of the cases of error-code`);
        }
      }
      dataView(memory0).setInt8(arg1 + 1, enum3, true);
      break;
    }
    default: {
      throw new TypeError('invalid variant specified for result');
    }
  }
}

function trampoline16(arg0, arg1) {
  const handle1 = arg0;
  const rsc0 = handleTable3.get(handle1).rep;
  let ret;
  try {
    ret = { tag: 'ok', val: Descriptor.prototype.stat.call(rsc0) };
  } catch (e) {
    ret = { tag: 'err', val: getErrorPayload(e) };
  }
  const variant11 = ret;
  switch (variant11.tag) {
    case 'ok': {
      const e = variant11.val;
      dataView(memory0).setInt8(arg1 + 0, 0, true);
      const {type: v2_0, linkCount: v2_1, size: v2_2, dataAccessTimestamp: v2_3, dataModificationTimestamp: v2_4, statusChangeTimestamp: v2_5 } = e;
      const val3 = v2_0;
      let enum3;
      switch (val3) {
        case 'unknown': {
          enum3 = 0;
          break;
        }
        case 'block-device': {
          enum3 = 1;
          break;
        }
        case 'character-device': {
          enum3 = 2;
          break;
        }
        case 'directory': {
          enum3 = 3;
          break;
        }
        case 'fifo': {
          enum3 = 4;
          break;
        }
        case 'symbolic-link': {
          enum3 = 5;
          break;
        }
        case 'regular-file': {
          enum3 = 6;
          break;
        }
        case 'socket': {
          enum3 = 7;
          break;
        }
        default: {
          if ((v2_0) instanceof Error) {
            console.error(v2_0);
          }
          
          throw new TypeError(`"${val3}" is not one of the cases of descriptor-type`);
        }
      }
      dataView(memory0).setInt8(arg1 + 8, enum3, true);
      dataView(memory0).setBigInt64(arg1 + 16, toUint64(v2_1), true);
      dataView(memory0).setBigInt64(arg1 + 24, toUint64(v2_2), true);
      const variant5 = v2_3;
      if (variant5 === null || variant5=== undefined) {
        dataView(memory0).setInt8(arg1 + 32, 0, true);
      } else {
        const e = variant5;
        dataView(memory0).setInt8(arg1 + 32, 1, true);
        const {seconds: v4_0, nanoseconds: v4_1 } = e;
        dataView(memory0).setBigInt64(arg1 + 40, toUint64(v4_0), true);
        dataView(memory0).setInt32(arg1 + 48, toUint32(v4_1), true);
      }
      const variant7 = v2_4;
      if (variant7 === null || variant7=== undefined) {
        dataView(memory0).setInt8(arg1 + 56, 0, true);
      } else {
        const e = variant7;
        dataView(memory0).setInt8(arg1 + 56, 1, true);
        const {seconds: v6_0, nanoseconds: v6_1 } = e;
        dataView(memory0).setBigInt64(arg1 + 64, toUint64(v6_0), true);
        dataView(memory0).setInt32(arg1 + 72, toUint32(v6_1), true);
      }
      const variant9 = v2_5;
      if (variant9 === null || variant9=== undefined) {
        dataView(memory0).setInt8(arg1 + 80, 0, true);
      } else {
        const e = variant9;
        dataView(memory0).setInt8(arg1 + 80, 1, true);
        const {seconds: v8_0, nanoseconds: v8_1 } = e;
        dataView(memory0).setBigInt64(arg1 + 88, toUint64(v8_0), true);
        dataView(memory0).setInt32(arg1 + 96, toUint32(v8_1), true);
      }
      break;
    }
    case 'err': {
      const e = variant11.val;
      dataView(memory0).setInt8(arg1 + 0, 1, true);
      const val10 = e;
      let enum10;
      switch (val10) {
        case 'access': {
          enum10 = 0;
          break;
        }
        case 'would-block': {
          enum10 = 1;
          break;
        }
        case 'already': {
          enum10 = 2;
          break;
        }
        case 'bad-descriptor': {
          enum10 = 3;
          break;
        }
        case 'busy': {
          enum10 = 4;
          break;
        }
        case 'deadlock': {
          enum10 = 5;
          break;
        }
        case 'quota': {
          enum10 = 6;
          break;
        }
        case 'exist': {
          enum10 = 7;
          break;
        }
        case 'file-too-large': {
          enum10 = 8;
          break;
        }
        case 'illegal-byte-sequence': {
          enum10 = 9;
          break;
        }
        case 'in-progress': {
          enum10 = 10;
          break;
        }
        case 'interrupted': {
          enum10 = 11;
          break;
        }
        case 'invalid': {
          enum10 = 12;
          break;
        }
        case 'io': {
          enum10 = 13;
          break;
        }
        case 'is-directory': {
          enum10 = 14;
          break;
        }
        case 'loop': {
          enum10 = 15;
          break;
        }
        case 'too-many-links': {
          enum10 = 16;
          break;
        }
        case 'message-size': {
          enum10 = 17;
          break;
        }
        case 'name-too-long': {
          enum10 = 18;
          break;
        }
        case 'no-device': {
          enum10 = 19;
          break;
        }
        case 'no-entry': {
          enum10 = 20;
          break;
        }
        case 'no-lock': {
          enum10 = 21;
          break;
        }
        case 'insufficient-memory': {
          enum10 = 22;
          break;
        }
        case 'insufficient-space': {
          enum10 = 23;
          break;
        }
        case 'not-directory': {
          enum10 = 24;
          break;
        }
        case 'not-empty': {
          enum10 = 25;
          break;
        }
        case 'not-recoverable': {
          enum10 = 26;
          break;
        }
        case 'unsupported': {
          enum10 = 27;
          break;
        }
        case 'no-tty': {
          enum10 = 28;
          break;
        }
        case 'no-such-device': {
          enum10 = 29;
          break;
        }
        case 'overflow': {
          enum10 = 30;
          break;
        }
        case 'not-permitted': {
          enum10 = 31;
          break;
        }
        case 'pipe': {
          enum10 = 32;
          break;
        }
        case 'read-only': {
          enum10 = 33;
          break;
        }
        case 'invalid-seek': {
          enum10 = 34;
          break;
        }
        case 'text-file-busy': {
          enum10 = 35;
          break;
        }
        case 'cross-device': {
          enum10 = 36;
          break;
        }
        default: {
          if ((e) instanceof Error) {
            console.error(e);
          }
          
          throw new TypeError(`"${val10}" is not one of the cases of error-code`);
        }
      }
      dataView(memory0).setInt8(arg1 + 8, enum10, true);
      break;
    }
    default: {
      throw new TypeError('invalid variant specified for result');
    }
  }
}

function trampoline17(arg0, arg1) {
  const handle1 = arg0;
  const rsc0 = handleTable1.get(handle1).rep;
  const ret = filesystemErrorCode(rsc0);
  const variant3 = ret;
  if (variant3 === null || variant3=== undefined) {
    dataView(memory0).setInt8(arg1 + 0, 0, true);
  } else {
    const e = variant3;
    dataView(memory0).setInt8(arg1 + 0, 1, true);
    const val2 = e;
    let enum2;
    switch (val2) {
      case 'access': {
        enum2 = 0;
        break;
      }
      case 'would-block': {
        enum2 = 1;
        break;
      }
      case 'already': {
        enum2 = 2;
        break;
      }
      case 'bad-descriptor': {
        enum2 = 3;
        break;
      }
      case 'busy': {
        enum2 = 4;
        break;
      }
      case 'deadlock': {
        enum2 = 5;
        break;
      }
      case 'quota': {
        enum2 = 6;
        break;
      }
      case 'exist': {
        enum2 = 7;
        break;
      }
      case 'file-too-large': {
        enum2 = 8;
        break;
      }
      case 'illegal-byte-sequence': {
        enum2 = 9;
        break;
      }
      case 'in-progress': {
        enum2 = 10;
        break;
      }
      case 'interrupted': {
        enum2 = 11;
        break;
      }
      case 'invalid': {
        enum2 = 12;
        break;
      }
      case 'io': {
        enum2 = 13;
        break;
      }
      case 'is-directory': {
        enum2 = 14;
        break;
      }
      case 'loop': {
        enum2 = 15;
        break;
      }
      case 'too-many-links': {
        enum2 = 16;
        break;
      }
      case 'message-size': {
        enum2 = 17;
        break;
      }
      case 'name-too-long': {
        enum2 = 18;
        break;
      }
      case 'no-device': {
        enum2 = 19;
        break;
      }
      case 'no-entry': {
        enum2 = 20;
        break;
      }
      case 'no-lock': {
        enum2 = 21;
        break;
      }
      case 'insufficient-memory': {
        enum2 = 22;
        break;
      }
      case 'insufficient-space': {
        enum2 = 23;
        break;
      }
      case 'not-directory': {
        enum2 = 24;
        break;
      }
      case 'not-empty': {
        enum2 = 25;
        break;
      }
      case 'not-recoverable': {
        enum2 = 26;
        break;
      }
      case 'unsupported': {
        enum2 = 27;
        break;
      }
      case 'no-tty': {
        enum2 = 28;
        break;
      }
      case 'no-such-device': {
        enum2 = 29;
        break;
      }
      case 'overflow': {
        enum2 = 30;
        break;
      }
      case 'not-permitted': {
        enum2 = 31;
        break;
      }
      case 'pipe': {
        enum2 = 32;
        break;
      }
      case 'read-only': {
        enum2 = 33;
        break;
      }
      case 'invalid-seek': {
        enum2 = 34;
        break;
      }
      case 'text-file-busy': {
        enum2 = 35;
        break;
      }
      case 'cross-device': {
        enum2 = 36;
        break;
      }
      default: {
        if ((e) instanceof Error) {
          console.error(e);
        }
        
        throw new TypeError(`"${val2}" is not one of the cases of error-code`);
      }
    }
    dataView(memory0).setInt8(arg1 + 1, enum2, true);
  }
}

function trampoline18(arg0, arg1) {
  const handle1 = arg0;
  const rsc0 = handleTable0.get(handle1).rep;
  let ret;
  try {
    ret = { tag: 'ok', val: OutputStream.prototype.checkWrite.call(rsc0) };
  } catch (e) {
    ret = { tag: 'err', val: getErrorPayload(e) };
  }
  const variant4 = ret;
  switch (variant4.tag) {
    case 'ok': {
      const e = variant4.val;
      dataView(memory0).setInt8(arg1 + 0, 0, true);
      dataView(memory0).setBigInt64(arg1 + 8, toUint64(e), true);
      break;
    }
    case 'err': {
      const e = variant4.val;
      dataView(memory0).setInt8(arg1 + 0, 1, true);
      const variant3 = e;
      switch (variant3.tag) {
        case 'last-operation-failed': {
          const e = variant3.val;
          dataView(memory0).setInt8(arg1 + 8, 0, true);
          if (!(e instanceof Error$1)) {
            throw new Error('Not a valid "Error" resource.');
          }
          const handle2 = handleCnt1++;
          handleTable1.set(handle2, { rep: e, own: true });
          dataView(memory0).setInt32(arg1 + 12, handle2, true);
          break;
        }
        case 'closed': {
          dataView(memory0).setInt8(arg1 + 8, 1, true);
          break;
        }
        default: {
          throw new TypeError(`invalid variant ${JSON.stringify(variant3.tag)} specified for StreamError`);
        }
      }
      break;
    }
    default: {
      throw new TypeError('invalid variant specified for result');
    }
  }
}

function trampoline19(arg0, arg1, arg2, arg3) {
  const handle1 = arg0;
  const rsc0 = handleTable0.get(handle1).rep;
  const ptr2 = arg1;
  const len2 = arg2;
  const result2 = new Uint8Array(memory0.buffer.slice(ptr2, ptr2 + len2 * 1));
  let ret;
  try {
    ret = { tag: 'ok', val: OutputStream.prototype.write.call(rsc0, result2) };
  } catch (e) {
    ret = { tag: 'err', val: getErrorPayload(e) };
  }
  const variant5 = ret;
  switch (variant5.tag) {
    case 'ok': {
      const e = variant5.val;
      dataView(memory0).setInt8(arg3 + 0, 0, true);
      break;
    }
    case 'err': {
      const e = variant5.val;
      dataView(memory0).setInt8(arg3 + 0, 1, true);
      const variant4 = e;
      switch (variant4.tag) {
        case 'last-operation-failed': {
          const e = variant4.val;
          dataView(memory0).setInt8(arg3 + 4, 0, true);
          if (!(e instanceof Error$1)) {
            throw new Error('Not a valid "Error" resource.');
          }
          const handle3 = handleCnt1++;
          handleTable1.set(handle3, { rep: e, own: true });
          dataView(memory0).setInt32(arg3 + 8, handle3, true);
          break;
        }
        case 'closed': {
          dataView(memory0).setInt8(arg3 + 4, 1, true);
          break;
        }
        default: {
          throw new TypeError(`invalid variant ${JSON.stringify(variant4.tag)} specified for StreamError`);
        }
      }
      break;
    }
    default: {
      throw new TypeError('invalid variant specified for result');
    }
  }
}

function trampoline20(arg0, arg1, arg2, arg3) {
  const handle1 = arg0;
  const rsc0 = handleTable0.get(handle1).rep;
  const ptr2 = arg1;
  const len2 = arg2;
  const result2 = new Uint8Array(memory0.buffer.slice(ptr2, ptr2 + len2 * 1));
  let ret;
  try {
    ret = { tag: 'ok', val: OutputStream.prototype.blockingWriteAndFlush.call(rsc0, result2) };
  } catch (e) {
    ret = { tag: 'err', val: getErrorPayload(e) };
  }
  const variant5 = ret;
  switch (variant5.tag) {
    case 'ok': {
      const e = variant5.val;
      dataView(memory0).setInt8(arg3 + 0, 0, true);
      break;
    }
    case 'err': {
      const e = variant5.val;
      dataView(memory0).setInt8(arg3 + 0, 1, true);
      const variant4 = e;
      switch (variant4.tag) {
        case 'last-operation-failed': {
          const e = variant4.val;
          dataView(memory0).setInt8(arg3 + 4, 0, true);
          if (!(e instanceof Error$1)) {
            throw new Error('Not a valid "Error" resource.');
          }
          const handle3 = handleCnt1++;
          handleTable1.set(handle3, { rep: e, own: true });
          dataView(memory0).setInt32(arg3 + 8, handle3, true);
          break;
        }
        case 'closed': {
          dataView(memory0).setInt8(arg3 + 4, 1, true);
          break;
        }
        default: {
          throw new TypeError(`invalid variant ${JSON.stringify(variant4.tag)} specified for StreamError`);
        }
      }
      break;
    }
    default: {
      throw new TypeError('invalid variant specified for result');
    }
  }
}

function trampoline21(arg0, arg1) {
  const handle1 = arg0;
  const rsc0 = handleTable0.get(handle1).rep;
  let ret;
  try {
    ret = { tag: 'ok', val: OutputStream.prototype.blockingFlush.call(rsc0) };
  } catch (e) {
    ret = { tag: 'err', val: getErrorPayload(e) };
  }
  const variant4 = ret;
  switch (variant4.tag) {
    case 'ok': {
      const e = variant4.val;
      dataView(memory0).setInt8(arg1 + 0, 0, true);
      break;
    }
    case 'err': {
      const e = variant4.val;
      dataView(memory0).setInt8(arg1 + 0, 1, true);
      const variant3 = e;
      switch (variant3.tag) {
        case 'last-operation-failed': {
          const e = variant3.val;
          dataView(memory0).setInt8(arg1 + 4, 0, true);
          if (!(e instanceof Error$1)) {
            throw new Error('Not a valid "Error" resource.');
          }
          const handle2 = handleCnt1++;
          handleTable1.set(handle2, { rep: e, own: true });
          dataView(memory0).setInt32(arg1 + 8, handle2, true);
          break;
        }
        case 'closed': {
          dataView(memory0).setInt8(arg1 + 4, 1, true);
          break;
        }
        default: {
          throw new TypeError(`invalid variant ${JSON.stringify(variant3.tag)} specified for StreamError`);
        }
      }
      break;
    }
    default: {
      throw new TypeError('invalid variant specified for result');
    }
  }
}

function trampoline22(arg0, arg1) {
  const ret = getRandomBytes(BigInt.asUintN(64, arg0));
  const val0 = ret;
  const len0 = val0.byteLength;
  const ptr0 = realloc0(0, 0, 1, len0 * 1);
  const src0 = new Uint8Array(val0.buffer || val0, val0.byteOffset, len0 * 1);
  (new Uint8Array(memory0.buffer, ptr0, len0 * 1)).set(src0);
  dataView(memory0).setInt32(arg1 + 4, len0, true);
  dataView(memory0).setInt32(arg1 + 0, ptr0, true);
}

function trampoline23(arg0) {
  const ret = getEnvironment();
  const vec3 = ret;
  const len3 = vec3.length;
  const result3 = realloc0(0, 0, 4, len3 * 16);
  for (let i = 0; i < vec3.length; i++) {
    const e = vec3[i];
    const base = result3 + i * 16;const [tuple0_0, tuple0_1] = e;
    const ptr1 = utf8Encode(tuple0_0, realloc0, memory0);
    const len1 = utf8EncodedLen;
    dataView(memory0).setInt32(base + 4, len1, true);
    dataView(memory0).setInt32(base + 0, ptr1, true);
    const ptr2 = utf8Encode(tuple0_1, realloc0, memory0);
    const len2 = utf8EncodedLen;
    dataView(memory0).setInt32(base + 12, len2, true);
    dataView(memory0).setInt32(base + 8, ptr2, true);
  }
  dataView(memory0).setInt32(arg0 + 4, len3, true);
  dataView(memory0).setInt32(arg0 + 0, result3, true);
}

function trampoline24(arg0) {
  const ret = getTerminalStdin();
  const variant1 = ret;
  if (variant1 === null || variant1=== undefined) {
    dataView(memory0).setInt8(arg0 + 0, 0, true);
  } else {
    const e = variant1;
    dataView(memory0).setInt8(arg0 + 0, 1, true);
    if (!(e instanceof TerminalInput)) {
      throw new Error('Not a valid "TerminalInput" resource.');
    }
    const handle0 = handleCnt6++;
    handleTable6.set(handle0, { rep: e, own: true });
    dataView(memory0).setInt32(arg0 + 4, handle0, true);
  }
}

function trampoline25(arg0) {
  const ret = getTerminalStdout();
  const variant1 = ret;
  if (variant1 === null || variant1=== undefined) {
    dataView(memory0).setInt8(arg0 + 0, 0, true);
  } else {
    const e = variant1;
    dataView(memory0).setInt8(arg0 + 0, 1, true);
    if (!(e instanceof TerminalOutput)) {
      throw new Error('Not a valid "TerminalOutput" resource.');
    }
    const handle0 = handleCnt7++;
    handleTable7.set(handle0, { rep: e, own: true });
    dataView(memory0).setInt32(arg0 + 4, handle0, true);
  }
}

function trampoline26(arg0) {
  const ret = getTerminalStderr();
  const variant1 = ret;
  if (variant1 === null || variant1=== undefined) {
    dataView(memory0).setInt8(arg0 + 0, 0, true);
  } else {
    const e = variant1;
    dataView(memory0).setInt8(arg0 + 0, 1, true);
    if (!(e instanceof TerminalOutput)) {
      throw new Error('Not a valid "TerminalOutput" resource.');
    }
    const handle0 = handleCnt7++;
    handleTable7.set(handle0, { rep: e, own: true });
    dataView(memory0).setInt32(arg0 + 4, handle0, true);
  }
}
let exports3;
let realloc1;
let postReturn0;
const handleTable4= new Map();
let handleCnt4 = 0;
function trampoline0(handle) {
  const handleEntry = handleTable4.get(handle);
  if (!handleEntry) {
    throw new Error(`Resource error: Invalid handle ${handle}`);
  }
  handleTable4.delete(handle);
  if (handleEntry.own && handleEntry.rep[symbolDispose]) {
    handleEntry.rep[symbolDispose]();
  }
}
const handleTable1= new Map();
let handleCnt1 = 0;
function trampoline1(handle) {
  const handleEntry = handleTable1.get(handle);
  if (!handleEntry) {
    throw new Error(`Resource error: Invalid handle ${handle}`);
  }
  handleTable1.delete(handle);
  if (handleEntry.own && handleEntry.rep[symbolDispose]) {
    handleEntry.rep[symbolDispose]();
  }
}
const handleTable2= new Map();
let handleCnt2 = 0;
function trampoline2(handle) {
  const handleEntry = handleTable2.get(handle);
  if (!handleEntry) {
    throw new Error(`Resource error: Invalid handle ${handle}`);
  }
  handleTable2.delete(handle);
  if (handleEntry.own && handleEntry.rep[symbolDispose]) {
    handleEntry.rep[symbolDispose]();
  }
}
const handleTable0= new Map();
let handleCnt0 = 0;
function trampoline3(handle) {
  const handleEntry = handleTable0.get(handle);
  if (!handleEntry) {
    throw new Error(`Resource error: Invalid handle ${handle}`);
  }
  handleTable0.delete(handle);
  if (handleEntry.own && handleEntry.rep[symbolDispose]) {
    handleEntry.rep[symbolDispose]();
  }
}
const handleTable3= new Map();
let handleCnt3 = 0;
function trampoline4(handle) {
  const handleEntry = handleTable3.get(handle);
  if (!handleEntry) {
    throw new Error(`Resource error: Invalid handle ${handle}`);
  }
  handleTable3.delete(handle);
  if (handleEntry.own && handleEntry.rep[symbolDispose]) {
    handleEntry.rep[symbolDispose]();
  }
}
const handleTable6= new Map();
let handleCnt6 = 0;
function trampoline5(handle) {
  const handleEntry = handleTable6.get(handle);
  if (!handleEntry) {
    throw new Error(`Resource error: Invalid handle ${handle}`);
  }
  handleTable6.delete(handle);
  if (handleEntry.own && handleEntry.rep[symbolDispose]) {
    handleEntry.rep[symbolDispose]();
  }
}
const handleTable7= new Map();
let handleCnt7 = 0;
function trampoline6(handle) {
  const handleEntry = handleTable7.get(handle);
  if (!handleEntry) {
    throw new Error(`Resource error: Invalid handle ${handle}`);
  }
  handleTable7.delete(handle);
  if (handleEntry.own && handleEntry.rep[symbolDispose]) {
    handleEntry.rep[symbolDispose]();
  }
}
const handleTable5= new Map();
let handleCnt5 = 0;
function trampoline7(handle) {
  const handleEntry = handleTable5.get(handle);
  if (!handleEntry) {
    throw new Error(`Resource error: Invalid handle ${handle}`);
  }
  handleTable5.delete(handle);
  if (handleEntry.own && handleEntry.rep[symbolDispose]) {
    handleEntry.rep[symbolDispose]();
  }
}

function diff(arg0, arg1, arg2) {
  const val0 = arg0;
  const len0 = val0.byteLength;
  const ptr0 = realloc1(0, 0, 1, len0 * 1);
  const src0 = new Uint8Array(val0.buffer || val0, val0.byteOffset, len0 * 1);
  (new Uint8Array(memory0.buffer, ptr0, len0 * 1)).set(src0);
  const val1 = arg1;
  const len1 = val1.byteLength;
  const ptr1 = realloc1(0, 0, 1, len1 * 1);
  const src1 = new Uint8Array(val1.buffer || val1, val1.byteOffset, len1 * 1);
  (new Uint8Array(memory0.buffer, ptr1, len1 * 1)).set(src1);
  const {threshold: v2_0, includeAntiAlias: v2_1 } = arg2;
  const variant3 = v2_0;
  let variant3_0;
  let variant3_1;
  if (variant3 === null || variant3=== undefined) {
    variant3_0 = 0;
    variant3_1 = 0;
  } else {
    const e = variant3;
    variant3_0 = 1;
    variant3_1 = +e;
  }
  const variant4 = v2_1;
  let variant4_0;
  let variant4_1;
  if (variant4 === null || variant4=== undefined) {
    variant4_0 = 0;
    variant4_1 = 0;
  } else {
    const e = variant4;
    variant4_0 = 1;
    variant4_1 = e ? 1 : 0;
  }
  const ret = exports1.diff(ptr0, len0, ptr1, len1, variant3_0, variant3_1, variant4_0, variant4_1);
  let variant10;
  switch (dataView(memory0).getUint8(ret + 0, true)) {
    case 0: {
      const ptr5 = dataView(memory0).getInt32(ret + 8, true);
      const len5 = dataView(memory0).getInt32(ret + 12, true);
      const result5 = new Uint8Array(memory0.buffer.slice(ptr5, ptr5 + len5 * 1));
      variant10= {
        tag: 'ok',
        val: {
          diffCount: dataView(memory0).getInt32(ret + 4, true) >>> 0,
          diffImage: result5,
          width: dataView(memory0).getInt32(ret + 16, true) >>> 0,
          height: dataView(memory0).getInt32(ret + 20, true) >>> 0,
        }
      };
      break;
    }
    case 1: {
      let variant9;
      switch (dataView(memory0).getUint8(ret + 4, true)) {
        case 0: {
          const ptr6 = dataView(memory0).getInt32(ret + 8, true);
          const len6 = dataView(memory0).getInt32(ret + 12, true);
          const result6 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr6, len6));
          variant9= {
            tag: 'decode',
            val: result6
          };
          break;
        }
        case 1: {
          const ptr7 = dataView(memory0).getInt32(ret + 8, true);
          const len7 = dataView(memory0).getInt32(ret + 12, true);
          const result7 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr7, len7));
          variant9= {
            tag: 'encode',
            val: result7
          };
          break;
        }
        case 2: {
          const ptr8 = dataView(memory0).getInt32(ret + 8, true);
          const len8 = dataView(memory0).getInt32(ret + 12, true);
          const result8 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr8, len8));
          variant9= {
            tag: 'image-length',
            val: result8
          };
          break;
        }
        default: {
          throw new TypeError('invalid variant discriminant for Error');
        }
      }
      variant10= {
        tag: 'err',
        val: variant9
      };
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for expected');
    }
  }
  postReturn0(ret);
  if (variant10.tag === 'err') {
    throw new ComponentError(variant10.val);
  }
  return variant10.val;
}

const $init = (async() => {
  const module0 = fetchCompile(new URL('./index.core.wasm', import.meta.url));
  const module1 = fetchCompile(new URL('./index.core2.wasm', import.meta.url));
  const module2 = base64Compile('AGFzbQEAAAABQgtgAX8AYAN/fn8AYAJ/fwBgBH9/f38AYAJ+fwBgBH9/f38Bf2ACf38Bf2ABfwF/YAN/f38Bf2AEf35/fwF/YAF/AAMZGAABAgICAgIDAwIEAAAAAAUGBgYHBggJCgQFAXABGBgHehkBMAAAATEAAQEyAAIBMwADATQABAE1AAUBNgAGATcABwE4AAgBOQAJAjEwAAoCMTEACwIxMgAMAjEzAA0CMTQADgIxNQAPAjE2ABACMTcAEQIxOAASAjE5ABMCMjAAFAIyMQAVAjIyABYCMjMAFwgkaW1wb3J0cwEACqcCGAkAIABBABEAAAsNACAAIAEgAkEBEQEACwsAIAAgAUECEQIACwsAIAAgAUEDEQIACwsAIAAgAUEEEQIACwsAIAAgAUEFEQIACwsAIAAgAUEGEQIACw8AIAAgASACIANBBxEDAAsPACAAIAEgAiADQQgRAwALCwAgACABQQkRAgALCwAgACABQQoRBAALCQAgAEELEQAACwkAIABBDBEAAAsJACAAQQ0RAAALCQAgAEEOEQAACw8AIAAgASACIANBDxEFAAsLACAAIAFBEBEGAAsLACAAIAFBEREGAAsLACAAIAFBEhEGAAsJACAAQRMRBwALCwAgACABQRQRBgALDQAgACABIAJBFREIAAsPACAAIAEgAiADQRYRCQALCQAgAEEXEQoACwAuCXByb2R1Y2VycwEMcHJvY2Vzc2VkLWJ5AQ13aXQtY29tcG9uZW50BjAuMTguMQCoDARuYW1lABMSd2l0LWNvbXBvbmVudDpzaGltAYsMGABFaW5kaXJlY3Qtd2FzaTpmaWxlc3lzdGVtL3ByZW9wZW5zQDAuMi4wLXJjLTIwMjMtMTAtMTgtZ2V0LWRpcmVjdG9yaWVzAVZpbmRpcmVjdC13YXNpOmZpbGVzeXN0ZW0vdHlwZXNAMC4yLjAtcmMtMjAyMy0xMC0xOC1bbWV0aG9kXWRlc2NyaXB0b3Iud3JpdGUtdmlhLXN0cmVhbQJXaW5kaXJlY3Qtd2FzaTpmaWxlc3lzdGVtL3R5cGVzQDAuMi4wLXJjLTIwMjMtMTAtMTgtW21ldGhvZF1kZXNjcmlwdG9yLmFwcGVuZC12aWEtc3RyZWFtA05pbmRpcmVjdC13YXNpOmZpbGVzeXN0ZW0vdHlwZXNAMC4yLjAtcmMtMjAyMy0xMC0xOC1bbWV0aG9kXWRlc2NyaXB0b3IuZ2V0LXR5cGUESmluZGlyZWN0LXdhc2k6ZmlsZXN5c3RlbS90eXBlc0AwLjIuMC1yYy0yMDIzLTEwLTE4LVttZXRob2RdZGVzY3JpcHRvci5zdGF0BUhpbmRpcmVjdC13YXNpOmZpbGVzeXN0ZW0vdHlwZXNAMC4yLjAtcmMtMjAyMy0xMC0xOC1maWxlc3lzdGVtLWVycm9yLWNvZGUGTmluZGlyZWN0LXdhc2k6aW8vc3RyZWFtc0AwLjIuMC1yYy0yMDIzLTEwLTE4LVttZXRob2Rdb3V0cHV0LXN0cmVhbS5jaGVjay13cml0ZQdIaW5kaXJlY3Qtd2FzaTppby9zdHJlYW1zQDAuMi4wLXJjLTIwMjMtMTAtMTgtW21ldGhvZF1vdXRwdXQtc3RyZWFtLndyaXRlCFtpbmRpcmVjdC13YXNpOmlvL3N0cmVhbXNAMC4yLjAtcmMtMjAyMy0xMC0xOC1bbWV0aG9kXW91dHB1dC1zdHJlYW0uYmxvY2tpbmctd3JpdGUtYW5kLWZsdXNoCVFpbmRpcmVjdC13YXNpOmlvL3N0cmVhbXNAMC4yLjAtcmMtMjAyMy0xMC0xOC1bbWV0aG9kXW91dHB1dC1zdHJlYW0uYmxvY2tpbmctZmx1c2gKQGluZGlyZWN0LXdhc2k6cmFuZG9tL3JhbmRvbUAwLjIuMC1yYy0yMDIzLTEwLTE4LWdldC1yYW5kb20tYnl0ZXMLQWluZGlyZWN0LXdhc2k6Y2xpL2Vudmlyb25tZW50QDAuMi4wLXJjLTIwMjMtMTAtMTgtZ2V0LWVudmlyb25tZW50DEdpbmRpcmVjdC13YXNpOmNsaS90ZXJtaW5hbC1zdGRpbkAwLjIuMC1yYy0yMDIzLTEwLTE4LWdldC10ZXJtaW5hbC1zdGRpbg1JaW5kaXJlY3Qtd2FzaTpjbGkvdGVybWluYWwtc3Rkb3V0QDAuMi4wLXJjLTIwMjMtMTAtMTgtZ2V0LXRlcm1pbmFsLXN0ZG91dA5JaW5kaXJlY3Qtd2FzaTpjbGkvdGVybWluYWwtc3RkZXJyQDAuMi4wLXJjLTIwMjMtMTAtMTgtZ2V0LXRlcm1pbmFsLXN0ZGVycg8lYWRhcHQtd2FzaV9zbmFwc2hvdF9wcmV2aWV3MS1mZF93cml0ZRAnYWRhcHQtd2FzaV9zbmFwc2hvdF9wcmV2aWV3MS1yYW5kb21fZ2V0EShhZGFwdC13YXNpX3NuYXBzaG90X3ByZXZpZXcxLWVudmlyb25fZ2V0Ei5hZGFwdC13YXNpX3NuYXBzaG90X3ByZXZpZXcxLWVudmlyb25fc2l6ZXNfZ2V0EyVhZGFwdC13YXNpX3NuYXBzaG90X3ByZXZpZXcxLWZkX2Nsb3NlFCthZGFwdC13YXNpX3NuYXBzaG90X3ByZXZpZXcxLWZkX3ByZXN0YXRfZ2V0FTBhZGFwdC13YXNpX3NuYXBzaG90X3ByZXZpZXcxLWZkX3ByZXN0YXRfZGlyX25hbWUWJGFkYXB0LXdhc2lfc25hcHNob3RfcHJldmlldzEtZmRfc2VlaxcmYWRhcHQtd2FzaV9zbmFwc2hvdF9wcmV2aWV3MS1wcm9jX2V4aXQ');
  const module3 = base64Compile('AGFzbQEAAAABQgtgAX8AYAN/fn8AYAJ/fwBgBH9/f38AYAJ+fwBgBH9/f38Bf2ACf38Bf2ABfwF/YAN/f38Bf2AEf35/fwF/YAF/AAKWARkAATAAAAABMQABAAEyAAIAATMAAgABNAACAAE1AAIAATYAAgABNwADAAE4AAMAATkAAgACMTAABAACMTEAAAACMTIAAAACMTMAAAACMTQAAAACMTUABQACMTYABgACMTcABgACMTgABgACMTkABwACMjAABgACMjEACAACMjIACQACMjMACgAIJGltcG9ydHMBcAEYGAkeAQBBAAsYAAECAwQFBgcICQoLDA0ODxAREhMUFRYXAC4JcHJvZHVjZXJzAQxwcm9jZXNzZWQtYnkBDXdpdC1jb21wb25lbnQGMC4xOC4xABwEbmFtZQAVFHdpdC1jb21wb25lbnQ6Zml4dXBz');
  const instanceFlags0 = new WebAssembly.Global({ value: "i32", mutable: true }, 3);
  Promise.all([module0, module1, module2, module3]).catch(() => {});
  ({ exports: exports0 } = await instantiateCore(await module2));
  ({ exports: exports1 } = await instantiateCore(await module0, {
    wasi_snapshot_preview1: {
      environ_get: exports0['17'],
      environ_sizes_get: exports0['18'],
      fd_close: exports0['19'],
      fd_prestat_dir_name: exports0['21'],
      fd_prestat_get: exports0['20'],
      fd_seek: exports0['22'],
      fd_write: exports0['15'],
      proc_exit: exports0['23'],
      random_get: exports0['16'],
    },
  }));
  ({ exports: exports2 } = await instantiateCore(await module1, {
    __main_module__: {
      cabi_realloc: exports1.cabi_realloc,
    },
    env: {
      memory: exports1.memory,
    },
    'wasi:cli/environment@0.2.0-rc-2023-10-18': {
      'get-environment': exports0['11'],
    },
    'wasi:cli/exit@0.2.0-rc-2023-10-18': {
      exit: trampoline9,
    },
    'wasi:cli/stderr@0.2.0-rc-2023-10-18': {
      'get-stderr': trampoline8,
    },
    'wasi:cli/stdin@0.2.0-rc-2023-10-18': {
      'get-stdin': trampoline10,
    },
    'wasi:cli/stdout@0.2.0-rc-2023-10-18': {
      'get-stdout': trampoline11,
    },
    'wasi:cli/terminal-input@0.2.0-rc-2023-10-18': {
      '[resource-drop]terminal-input': trampoline5,
    },
    'wasi:cli/terminal-output@0.2.0-rc-2023-10-18': {
      '[resource-drop]terminal-output': trampoline6,
    },
    'wasi:cli/terminal-stderr@0.2.0-rc-2023-10-18': {
      'get-terminal-stderr': exports0['14'],
    },
    'wasi:cli/terminal-stdin@0.2.0-rc-2023-10-18': {
      'get-terminal-stdin': exports0['12'],
    },
    'wasi:cli/terminal-stdout@0.2.0-rc-2023-10-18': {
      'get-terminal-stdout': exports0['13'],
    },
    'wasi:filesystem/preopens@0.2.0-rc-2023-10-18': {
      'get-directories': exports0['0'],
    },
    'wasi:filesystem/types@0.2.0-rc-2023-10-18': {
      '[method]descriptor.append-via-stream': exports0['2'],
      '[method]descriptor.get-type': exports0['3'],
      '[method]descriptor.stat': exports0['4'],
      '[method]descriptor.write-via-stream': exports0['1'],
      '[resource-drop]descriptor': trampoline4,
      '[resource-drop]directory-entry-stream': trampoline0,
      'filesystem-error-code': exports0['5'],
    },
    'wasi:io/streams@0.2.0-rc-2023-10-18': {
      '[method]output-stream.blocking-flush': exports0['9'],
      '[method]output-stream.blocking-write-and-flush': exports0['8'],
      '[method]output-stream.check-write': exports0['6'],
      '[method]output-stream.write': exports0['7'],
      '[resource-drop]error': trampoline1,
      '[resource-drop]input-stream': trampoline2,
      '[resource-drop]output-stream': trampoline3,
    },
    'wasi:random/random@0.2.0-rc-2023-10-18': {
      'get-random-bytes': exports0['10'],
    },
    'wasi:sockets/tcp@0.2.0-rc-2023-10-18': {
      '[resource-drop]tcp-socket': trampoline7,
    },
  }));
  memory0 = exports1.memory;
  realloc0 = exports2.cabi_import_realloc;
  ({ exports: exports3 } = await instantiateCore(await module3, {
    '': {
      $imports: exports0.$imports,
      '0': trampoline12,
      '1': trampoline13,
      '10': trampoline22,
      '11': trampoline23,
      '12': trampoline24,
      '13': trampoline25,
      '14': trampoline26,
      '15': exports2.fd_write,
      '16': exports2.random_get,
      '17': exports2.environ_get,
      '18': exports2.environ_sizes_get,
      '19': exports2.fd_close,
      '2': trampoline14,
      '20': exports2.fd_prestat_get,
      '21': exports2.fd_prestat_dir_name,
      '22': exports2.fd_seek,
      '23': exports2.proc_exit,
      '3': trampoline15,
      '4': trampoline16,
      '5': trampoline17,
      '6': trampoline18,
      '7': trampoline19,
      '8': trampoline20,
      '9': trampoline21,
    },
  }));
  realloc1 = exports1.cabi_realloc;
  postReturn0 = exports1.cabi_post_diff;
})();

await $init;

export { diff }