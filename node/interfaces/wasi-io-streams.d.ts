export namespace WasiIoStreams {
  export { OutputStream };
  export { Error };
  export { InputStream };
}
export type StreamError = StreamErrorLastOperationFailed | StreamErrorClosed;
export interface StreamErrorLastOperationFailed {
  tag: 'last-operation-failed',
  val: Error,
}
export interface StreamErrorClosed {
  tag: 'closed',
}

export class Error {
}

export class OutputStream {
  checkWrite(): bigint;
  write(contents: Uint8Array): void;
  blockingWriteAndFlush(contents: Uint8Array): void;
  blockingFlush(): void;
}

export class InputStream {
}
