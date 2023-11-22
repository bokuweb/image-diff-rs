export namespace BokuwebImageDiffTypes {
}
export interface Opts {
  threshold?: number,
  includeAntiAlias?: boolean,
}
export interface Output {
  diffCount: number,
  diffImage: Uint8Array,
  width: number,
  height: number,
}
export type Error = ErrorDecode | ErrorEncode | ErrorImageLength;
export interface ErrorDecode {
  tag: 'decode',
  val: string,
}
export interface ErrorEncode {
  tag: 'encode',
  val: string,
}
export interface ErrorImageLength {
  tag: 'image-length',
  val: string,
}
