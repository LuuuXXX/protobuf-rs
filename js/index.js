/**
 * @protobuf-rs/core - High-performance Protocol Buffers implementation
 * 
 * This is a JavaScript wrapper around the Rust/WASM core providing
 * a protobuf.js-compatible API for encoding and decoding protobuf messages.
 */

let wasm;

/**
 * Initialize the WASM module
 * @returns {Promise<void>}
 */
export async function init(wasmModule) {
  if (wasmModule) {
    wasm = wasmModule;
  } else {
    // Auto-load in Node.js environment
    if (typeof require !== 'undefined') {
      wasm = require('@protobuf-rs/core');
    } else {
      throw new Error('WASM module must be provided in browser environment');
    }
  }
}

/**
 * Wire type constants
 */
export const WireType = {
  VARINT: 0,
  FIXED64: 1,
  LENGTH_DELIMITED: 2,
  START_GROUP: 3,
  END_GROUP: 4,
  FIXED32: 5,
};

/**
 * Writer class for encoding protobuf messages
 */
export class Writer {
  constructor() {
    this._writer = new wasm.WasmWriter();
  }

  static withCapacity(capacity) {
    const writer = Object.create(Writer.prototype);
    writer._writer = wasm.WasmWriter.withCapacity(capacity);
    return writer;
  }

  get length() {
    return this._writer.length;
  }

  reset() {
    this._writer.reset();
    return this;
  }

  finish() {
    return this._writer.finish();
  }

  asBytes() {
    return this._writer.asBytes();
  }

  // Low-level methods
  writeTag(fieldNumber, wireType) {
    this._writer.writeTag(fieldNumber, wireType);
    return this;
  }

  writeVarint32(value) {
    this._writer.writeVarint32(value);
    return this;
  }

  writeVarint64(value) {
    this._writer.writeVarint64(BigInt(value));
    return this;
  }

  writeSint32(value) {
    this._writer.writeSint32(value);
    return this;
  }

  writeSint64(value) {
    this._writer.writeSint64(BigInt(value));
    return this;
  }

  writeFixed32(value) {
    this._writer.writeFixed32(value);
    return this;
  }

  writeFixed64(value) {
    this._writer.writeFixed64(BigInt(value));
    return this;
  }

  writeSfixed32(value) {
    this._writer.writeSfixed32(value);
    return this;
  }

  writeSfixed64(value) {
    this._writer.writeSfixed64(BigInt(value));
    return this;
  }

  writeFloat(value) {
    this._writer.writeFloat(value);
    return this;
  }

  writeDouble(value) {
    this._writer.writeDouble(value);
    return this;
  }

  writeBool(value) {
    this._writer.writeBool(value);
    return this;
  }

  writeString(value) {
    this._writer.writeString(value);
    return this;
  }

  writeBytes(value) {
    this._writer.writeBytes(value);
    return this;
  }

  writeRawBytes(value) {
    this._writer.writeRawBytes(value);
    return this;
  }

  // High-level field methods
  uint32(fieldNumber, value) {
    this._writer.writeUint32Field(fieldNumber, value);
    return this;
  }

  uint64(fieldNumber, value) {
    this._writer.writeUint64Field(fieldNumber, BigInt(value));
    return this;
  }

  int32(fieldNumber, value) {
    this._writer.writeInt32Field(fieldNumber, value);
    return this;
  }

  int64(fieldNumber, value) {
    this._writer.writeInt64Field(fieldNumber, BigInt(value));
    return this;
  }

  sint32(fieldNumber, value) {
    this._writer.writeSint32Field(fieldNumber, value);
    return this;
  }

  sint64(fieldNumber, value) {
    this._writer.writeSint64Field(fieldNumber, BigInt(value));
    return this;
  }

  string(fieldNumber, value) {
    this._writer.writeStringField(fieldNumber, value);
    return this;
  }

  bytes(fieldNumber, value) {
    this._writer.writeBytesField(fieldNumber, value);
    return this;
  }

  bool(fieldNumber, value) {
    this._writer.writeBoolField(fieldNumber, value);
    return this;
  }

  float(fieldNumber, value) {
    this._writer.writeFloatField(fieldNumber, value);
    return this;
  }

  double(fieldNumber, value) {
    this._writer.writeDoubleField(fieldNumber, value);
    return this;
  }

  fixed32(fieldNumber, value) {
    this._writer.writeFixed32Field(fieldNumber, value);
    return this;
  }

  fixed64(fieldNumber, value) {
    this._writer.writeFixed64Field(fieldNumber, BigInt(value));
    return this;
  }

  sfixed32(fieldNumber, value) {
    this._writer.writeSfixed32Field(fieldNumber, value);
    return this;
  }

  sfixed64(fieldNumber, value) {
    this._writer.writeSfixed64Field(fieldNumber, BigInt(value));
    return this;
  }
}

/**
 * Reader class for decoding protobuf messages
 */
export class Reader {
  constructor(buffer) {
    this._reader = new wasm.WasmReader(new Uint8Array(buffer));
  }

  get pos() {
    return this._reader.pos;
  }

  get length() {
    return this._reader.length;
  }

  remaining() {
    return this._reader.remaining();
  }

  isEof() {
    return this._reader.isEof();
  }

  reset() {
    this._reader.reset();
    return this;
  }

  readTag() {
    const result = this._reader.readTag();
    return [result[0], result[1]];
  }

  readVarint32() {
    return this._reader.readVarint32();
  }

  readVarint64() {
    return this._reader.readVarint64();
  }

  readSint32() {
    return this._reader.readSint32();
  }

  readSint64() {
    return this._reader.readSint64();
  }

  readInt32() {
    return this._reader.readInt32();
  }

  readInt64() {
    return this._reader.readInt64();
  }

  readFixed32() {
    return this._reader.readFixed32();
  }

  readFixed64() {
    return this._reader.readFixed64();
  }

  readSfixed32() {
    return this._reader.readSfixed32();
  }

  readSfixed64() {
    return this._reader.readSfixed64();
  }

  readFloat() {
    return this._reader.readFloat();
  }

  readDouble() {
    return this._reader.readDouble();
  }

  readBool() {
    return this._reader.readBool();
  }

  readString() {
    return this._reader.readString();
  }

  readBytes() {
    return this._reader.readBytes();
  }

  skip(wireType) {
    this._reader.skip(wireType);
    return this;
  }

  skipBytes(n) {
    this._reader.skipBytes(n);
    return this;
  }
}

/**
 * Utility functions
 */
export const util = {
  makeTag(fieldNumber, wireType) {
    return wasm.makeTag_util(fieldNumber, wireType);
  },

  parseTag(tag) {
    const result = wasm.parseTag_util(tag);
    return [result[0], result[1]];
  },

  encodeZigZag32(n) {
    return wasm.encodeZigZag32_util(n);
  },

  decodeZigZag32(n) {
    return wasm.decodeZigZag32_util(n);
  },

  encodeZigZag64(n) {
    return wasm.encodeZigZag64_util(BigInt(n));
  },

  decodeZigZag64(n) {
    return wasm.decodeZigZag64_util(BigInt(n));
  },
};

export { Writer as default };
