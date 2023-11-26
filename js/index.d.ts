export interface Opts {
  threshold?: number;
  includeAntiAlias?: boolean;
}
export interface Output {
  diffCount: number;
  diffImage: Uint8Array;
  width: number;
  height: number;
}
export type Error = ErrorDecode | ErrorEncode;
export interface ErrorDecode {
  tag: "decode";
  val: string;
}
export interface ErrorEncode {
  tag: "encode";
  val: string;
}

export function diff(imga: Uint8Array, imgb: Uint8Array, opts: Opts): Output;
