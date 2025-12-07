/**
 * Type definitions for @protobuf-rs/core
 */

export function init(wasmModule?: any): Promise<void>;

export const WireType: {
  readonly VARINT: 0;
  readonly FIXED64: 1;
  readonly LENGTH_DELIMITED: 2;
  readonly START_GROUP: 3;
  readonly END_GROUP: 4;
  readonly FIXED32: 5;
};

export class Writer {
  constructor();
  static withCapacity(capacity: number): Writer;
  
  readonly length: number;
  
  reset(): this;
  finish(): Uint8Array;
  asBytes(): Uint8Array;
  
  writeTag(fieldNumber: number, wireType: number): this;
  writeVarint32(value: number): this;
  writeVarint64(value: number | bigint): this;
  writeSint32(value: number): this;
  writeSint64(value: number | bigint): this;
  writeFixed32(value: number): this;
  writeFixed64(value: number | bigint): this;
  writeSfixed32(value: number): this;
  writeSfixed64(value: number | bigint): this;
  writeFloat(value: number): this;
  writeDouble(value: number): this;
  writeBool(value: boolean): this;
  writeString(value: string): this;
  writeBytes(value: Uint8Array): this;
  writeRawBytes(value: Uint8Array): this;
  
  uint32(fieldNumber: number, value: number): this;
  uint64(fieldNumber: number, value: number | bigint): this;
  int32(fieldNumber: number, value: number): this;
  int64(fieldNumber: number, value: number | bigint): this;
  sint32(fieldNumber: number, value: number): this;
  sint64(fieldNumber: number, value: number | bigint): this;
  string(fieldNumber: number, value: string): this;
  bytes(fieldNumber: number, value: Uint8Array): this;
  bool(fieldNumber: number, value: boolean): this;
  float(fieldNumber: number, value: number): this;
  double(fieldNumber: number, value: number): this;
  fixed32(fieldNumber: number, value: number): this;
  fixed64(fieldNumber: number, value: number | bigint): this;
  sfixed32(fieldNumber: number, value: number): this;
  sfixed64(fieldNumber: number, value: number | bigint): this;
}

export class Reader {
  constructor(buffer: Uint8Array | ArrayBuffer);
  
  readonly pos: number;
  readonly length: number;
  
  remaining(): number;
  isEof(): boolean;
  reset(): this;
  
  readTag(): [number, number];
  readVarint32(): number;
  readVarint64(): bigint;
  readSint32(): number;
  readSint64(): bigint;
  readInt32(): number;
  readInt64(): bigint;
  readFixed32(): number;
  readFixed64(): bigint;
  readSfixed32(): number;
  readSfixed64(): bigint;
  readFloat(): number;
  readDouble(): number;
  readBool(): boolean;
  readString(): string;
  readBytes(): Uint8Array;
  
  skip(wireType: number): this;
  skipBytes(n: number): this;
}

export const util: {
  makeTag(fieldNumber: number, wireType: number): number;
  parseTag(tag: number): [number, number];
  encodeZigZag32(n: number): number;
  decodeZigZag32(n: number): number;
  encodeZigZag64(n: number | bigint): bigint;
  decodeZigZag64(n: number | bigint): bigint;
};

export default Writer;
