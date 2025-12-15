/**
 * Hybrid Adapter for protobuf.js
 * 
 * Provides automatic fallback between native Rust implementation and pure JavaScript.
 * Drop-in replacement for protobuf.js Reader/Writer with identical API.
 */

let nativeModule = null;
let isNative = false;
let protobufjs = null;

// Set to false to suppress initialization messages
const VERBOSE = process.env.PROTOBUF_RS_VERBOSE === 'true';

// Try to load native Rust module
try {
  nativeModule = require('../library/index');
  isNative = true;
  if (VERBOSE) {
    console.log('✓ protobuf-rs: Using native Rust implementation');
  }
} catch (err) {
  if (VERBOSE) {
    console.log('⚠ protobuf-rs: Native module not available, falling back to protobuf.js');
  }
  try {
    protobufjs = require('protobufjs');
  } catch (pbErr) {
    throw new Error('Neither native module nor protobuf.js is available');
  }
}

/**
 * Check if native Rust module is available
 * @returns {boolean} true if native module is loaded
 */
function isNativeAvailable() {
  return isNative;
}

/**
 * Get current implementation type for debugging
 * @returns {string} "native" or "javascript"
 */
function getImplementationType() {
  return isNative ? 'native' : 'javascript';
}

/**
 * Reader wrapper that proxies to native or JS implementation
 */
class Reader {
  constructor(buffer) {
    if (isNative) {
      // For native implementation, we'll track position and buffer manually
      this.buf = buffer;
      this.pos = 0;
      this.len = buffer.length;
    } else {
      // Use protobuf.js Reader
      this._reader = protobufjs.Reader.create(buffer);
      this.buf = this._reader.buf;
      this.pos = this._reader.pos;
      this.len = this._reader.len;
    }
  }

  /**
   * Create a new Reader instance
   * @param {Buffer} buffer - The buffer to read from
   * @returns {Reader} Reader instance
   */
  static create(buffer) {
    return new Reader(buffer);
  }

  /**
   * Read uint32 value
   * @returns {number} uint32 value
   */
  uint32() {
    if (isNative) {
      // Calculate varint length first to avoid inefficient slicing
      let len = 0;
      let startPos = this.pos;
      while (startPos + len < this.len) {
        if ((this.buf[startPos + len] & 0x80) === 0) {
          len++;
          break;
        }
        len++;
        if (len > 10) {
          throw new Error('Varint too long');
        }
      }
      
      if (len === 0 || (this.buf[startPos + len - 1] & 0x80) !== 0) {
        throw new Error('Incomplete varint');
      }
      
      // Now decode with the calculated length
      const slice = this.buf.slice(this.pos, this.pos + len);
      const value = nativeModule.decodeVarint(slice);
      this.pos += len;
      return value >>> 0; // Ensure unsigned 32-bit
    } else {
      const val = this._reader.uint32();
      this.pos = this._reader.pos;
      return val;
    }
  }

  /**
   * Read int32 value
   * @returns {number} int32 value
   */
  int32() {
    if (isNative) {
      return this.uint32() | 0; // Convert to signed 32-bit
    } else {
      const val = this._reader.int32();
      this.pos = this._reader.pos;
      return val;
    }
  }

  /**
   * Read sint32 value (zigzag encoded)
   * @returns {number} sint32 value
   */
  sint32() {
    if (isNative) {
      const n = this.uint32();
      return nativeModule.decodeZigzag(n);
    } else {
      const val = this._reader.sint32();
      this.pos = this._reader.pos;
      return val;
    }
  }

  /**
   * Read uint64 value as number
   * @returns {number} uint64 value
   */
  uint64() {
    if (isNative) {
      return this.uint32(); // For simplicity, using uint32 logic
    } else {
      const val = this._reader.uint64();
      this.pos = this._reader.pos;
      return typeof val === 'number' ? val : val.toNumber();
    }
  }

  /**
   * Read int64 value as number
   * @returns {number} int64 value
   */
  sint64() {
    if (isNative) {
      const n = this.uint64();
      return nativeModule.decodeZigzag(n);
    } else {
      const val = this._reader.sint64();
      this.pos = this._reader.pos;
      return typeof val === 'number' ? val : val.toNumber();
    }
  }

  /**
   * Read bool value
   * @returns {boolean} bool value
   */
  bool() {
    if (isNative) {
      return this.uint32() !== 0;
    } else {
      const val = this._reader.bool();
      this.pos = this._reader.pos;
      return val;
    }
  }

  /**
   * Read fixed32 value
   * @returns {number} fixed32 value
   */
  fixed32() {
    if (isNative) {
      if (this.pos + 4 > this.len) {
        throw new Error('Buffer overflow');
      }
      const value = this.buf.readUInt32LE(this.pos);
      this.pos += 4;
      return value;
    } else {
      const val = this._reader.fixed32();
      this.pos = this._reader.pos;
      return val;
    }
  }

  /**
   * Read sfixed32 value
   * @returns {number} sfixed32 value
   */
  sfixed32() {
    if (isNative) {
      if (this.pos + 4 > this.len) {
        throw new Error('Buffer overflow');
      }
      const value = this.buf.readInt32LE(this.pos);
      this.pos += 4;
      return value;
    } else {
      const val = this._reader.sfixed32();
      this.pos = this._reader.pos;
      return val;
    }
  }

  /**
   * Read fixed64 value
   * @returns {number} fixed64 value
   */
  fixed64() {
    if (isNative) {
      if (this.pos + 8 > this.len) {
        throw new Error('Buffer overflow');
      }
      // Read as two 32-bit values for simplicity
      const lo = this.buf.readUInt32LE(this.pos);
      const hi = this.buf.readUInt32LE(this.pos + 4);
      this.pos += 8;
      return lo + hi * 0x100000000;
    } else {
      const val = this._reader.fixed64();
      this.pos = this._reader.pos;
      return typeof val === 'number' ? val : val.toNumber();
    }
  }

  /**
   * Read sfixed64 value
   * @returns {number} sfixed64 value
   */
  sfixed64() {
    if (isNative) {
      if (this.pos + 8 > this.len) {
        throw new Error('Buffer overflow');
      }
      const lo = this.buf.readUInt32LE(this.pos);
      const hi = this.buf.readInt32LE(this.pos + 4);
      this.pos += 8;
      return lo + hi * 0x100000000;
    } else {
      const val = this._reader.sfixed64();
      this.pos = this._reader.pos;
      return typeof val === 'number' ? val : val.toNumber();
    }
  }

  /**
   * Read float value
   * @returns {number} float value
   */
  float() {
    if (isNative) {
      if (this.pos + 4 > this.len) {
        throw new Error('Buffer overflow');
      }
      const value = this.buf.readFloatLE(this.pos);
      this.pos += 4;
      return value;
    } else {
      const val = this._reader.float();
      this.pos = this._reader.pos;
      return val;
    }
  }

  /**
   * Read double value
   * @returns {number} double value
   */
  double() {
    if (isNative) {
      if (this.pos + 8 > this.len) {
        throw new Error('Buffer overflow');
      }
      const value = this.buf.readDoubleLE(this.pos);
      this.pos += 8;
      return value;
    } else {
      const val = this._reader.double();
      this.pos = this._reader.pos;
      return val;
    }
  }

  /**
   * Read bytes value
   * @returns {Buffer} bytes value
   */
  bytes() {
    if (isNative) {
      const length = this.uint32();
      if (this.pos + length > this.len) {
        throw new Error('Buffer overflow');
      }
      const value = this.buf.slice(this.pos, this.pos + length);
      this.pos += length;
      return value;
    } else {
      const val = this._reader.bytes();
      this.pos = this._reader.pos;
      return Buffer.from(val);
    }
  }

  /**
   * Read string value
   * @returns {string} string value
   */
  string() {
    if (isNative) {
      const bytes = this.bytes();
      return bytes.toString('utf8');
    } else {
      const val = this._reader.string();
      this.pos = this._reader.pos;
      return val;
    }
  }

  /**
   * Skip specified number of bytes
   * @param {number} length - Number of bytes to skip
   * @returns {Reader} this
   */
  skip(length) {
    if (isNative) {
      if (this.pos + length > this.len) {
        throw new Error('Buffer overflow');
      }
      this.pos += length;
      return this;
    } else {
      this._reader.skip(length);
      this.pos = this._reader.pos;
      return this;
    }
  }

  /**
   * Skip a field by wire type
   * @param {number} wireType - Wire type (0-5)
   * @returns {Reader} this
   */
  skipType(wireType) {
    if (isNative) {
      switch (wireType) {
        case 0: // Varint
          while (this.pos < this.len && (this.buf[this.pos++] & 0x80) !== 0);
          // Check if we exited because we ran out of buffer with continuation bit still set
          if (this.pos === this.len && this.pos > 0 && (this.buf[this.pos - 1] & 0x80) !== 0) {
            throw new Error('Malformed varint: buffer ended before end of varint');
          }
          return this;
        case 1: // 64-bit
          this.skip(8);
          return this;
        case 2: // Length-delimited
          this.skip(this.uint32());
          return this;
        case 3: // Start group (deprecated)
          let nestedWireType;
          while ((nestedWireType = this.uint32() & 7) !== 4) {
            this.skipType(nestedWireType);
          }
          return this;
        case 5: // 32-bit
          this.skip(4);
          return this;
        default:
          throw new Error('Invalid wire type ' + wireType);
      }
    } else {
      this._reader.skipType(wireType);
      this.pos = this._reader.pos;
      return this;
    }
  }
}

/**
 * Writer wrapper that proxies to native or JS implementation
 */
class Writer {
  constructor() {
    if (isNative) {
      this.buf = [];
      this.len = 0;
      this.head = { next: null };
      this.tail = this.head;
      this._states = null;
    } else {
      this._writer = protobufjs.Writer.create();
    }
  }

  /**
   * Create a new Writer instance
   * @returns {Writer} Writer instance
   */
  static create() {
    return new Writer();
  }

  /**
   * Write uint32 value
   * @param {number} value - uint32 value
   * @returns {Writer} this
   */
  uint32(value) {
    if (isNative) {
      const encoded = nativeModule.encodeVarint(value >>> 0);
      this.buf.push(encoded);
      this.len += encoded.length;
      return this;
    } else {
      this._writer.uint32(value);
      return this;
    }
  }

  /**
   * Write int32 value
   * @param {number} value - int32 value
   * @returns {Writer} this
   */
  int32(value) {
    if (isNative) {
      return this.uint32(value);
    } else {
      this._writer.int32(value);
      return this;
    }
  }

  /**
   * Write sint32 value (zigzag encoded)
   * @param {number} value - sint32 value
   * @returns {Writer} this
   */
  sint32(value) {
    if (isNative) {
      const encoded = nativeModule.encodeZigzag(value);
      return this.uint32(encoded);
    } else {
      this._writer.sint32(value);
      return this;
    }
  }

  /**
   * Write uint64 value
   * @param {number} value - uint64 value
   * @returns {Writer} this
   */
  uint64(value) {
    if (isNative) {
      return this.uint32(value);
    } else {
      this._writer.uint64(value);
      return this;
    }
  }

  /**
   * Write sint64 value (zigzag encoded)
   * @param {number} value - sint64 value
   * @returns {Writer} this
   */
  sint64(value) {
    if (isNative) {
      const encoded = nativeModule.encodeZigzag(value);
      return this.uint64(encoded);
    } else {
      this._writer.sint64(value);
      return this;
    }
  }

  /**
   * Write bool value
   * @param {boolean} value - bool value
   * @returns {Writer} this
   */
  bool(value) {
    if (isNative) {
      return this.uint32(value ? 1 : 0);
    } else {
      this._writer.bool(value);
      return this;
    }
  }

  /**
   * Write fixed32 value
   * @param {number} value - fixed32 value
   * @returns {Writer} this
   */
  fixed32(value) {
    if (isNative) {
      const buf = Buffer.allocUnsafe(4);
      buf.writeUInt32LE(value >>> 0, 0);
      this.buf.push(buf);
      this.len += 4;
      return this;
    } else {
      this._writer.fixed32(value);
      return this;
    }
  }

  /**
   * Write sfixed32 value
   * @param {number} value - sfixed32 value
   * @returns {Writer} this
   */
  sfixed32(value) {
    if (isNative) {
      const buf = Buffer.allocUnsafe(4);
      buf.writeInt32LE(value | 0, 0);
      this.buf.push(buf);
      this.len += 4;
      return this;
    } else {
      this._writer.sfixed32(value);
      return this;
    }
  }

  /**
   * Write fixed64 value
   * @param {number} value - fixed64 value
   * @returns {Writer} this
   */
  fixed64(value) {
    if (isNative) {
      const buf = Buffer.allocUnsafe(8);
      const lo = value >>> 0;
      const hi = (value / 0x100000000) >>> 0;
      buf.writeUInt32LE(lo, 0);
      buf.writeUInt32LE(hi, 4);
      this.buf.push(buf);
      this.len += 8;
      return this;
    } else {
      this._writer.fixed64(value);
      return this;
    }
  }

  /**
   * Write sfixed64 value
   * @param {number} value - sfixed64 value
   * @returns {Writer} this
   */
  sfixed64(value) {
    if (isNative) {
      const buf = Buffer.allocUnsafe(8);
      const lo = value >>> 0;
      const hi = Math.floor(value / 0x100000000);
      buf.writeUInt32LE(lo, 0);
      buf.writeInt32LE(hi, 4);
      this.buf.push(buf);
      this.len += 8;
      return this;
    } else {
      this._writer.sfixed64(value);
      return this;
    }
  }

  /**
   * Write float value
   * @param {number} value - float value
   * @returns {Writer} this
   */
  float(value) {
    if (isNative) {
      const buf = Buffer.allocUnsafe(4);
      buf.writeFloatLE(value, 0);
      this.buf.push(buf);
      this.len += 4;
      return this;
    } else {
      this._writer.float(value);
      return this;
    }
  }

  /**
   * Write double value
   * @param {number} value - double value
   * @returns {Writer} this
   */
  double(value) {
    if (isNative) {
      const buf = Buffer.allocUnsafe(8);
      buf.writeDoubleLE(value, 0);
      this.buf.push(buf);
      this.len += 8;
      return this;
    } else {
      this._writer.double(value);
      return this;
    }
  }

  /**
   * Write bytes value
   * @param {Buffer} value - bytes value
   * @returns {Writer} this
   */
  bytes(value) {
    if (isNative) {
      const len = value.length;
      this.uint32(len);
      if (len > 0) {
        this.buf.push(Buffer.from(value));
        this.len += len;
      }
      return this;
    } else {
      this._writer.bytes(value);
      return this;
    }
  }

  /**
   * Write string value
   * @param {string} value - string value
   * @returns {Writer} this
   */
  string(value) {
    if (isNative) {
      const buf = Buffer.from(value, 'utf8');
      return this.bytes(buf);
    } else {
      this._writer.string(value);
      return this;
    }
  }

  /**
   * Fork for length-delimited operations
   * @returns {Writer} this
   */
  fork() {
    if (isNative) {
      if (!this._states) {
        this._states = [];
      }
      this._states.push({
        buf: this.buf,
        len: this.len
      });
      this.buf = [];
      this.len = 0;
      return this;
    } else {
      this._writer.fork();
      return this;
    }
  }

  /**
   * Complete length-delimited operation
   * @returns {Writer} this
   */
  ldelim() {
    if (isNative) {
      if (!this._states || this._states.length === 0) {
        throw new Error('No fork to delimit');
      }
      const state = this._states.pop();
      const content = Buffer.concat(this.buf);
      this.buf = state.buf;
      this.len = state.len;
      this.uint32(content.length);
      this.buf.push(content);
      this.len += content.length;
      return this;
    } else {
      this._writer.ldelim();
      return this;
    }
  }

  /**
   * Reset writer
   * @returns {Writer} this
   */
  reset() {
    if (isNative) {
      this.buf = [];
      this.len = 0;
      this._states = null;
      return this;
    } else {
      this._writer.reset();
      return this;
    }
  }

  /**
   * Finish writing and return buffer
   * @returns {Buffer} The complete buffer
   */
  finish() {
    if (isNative) {
      return Buffer.concat(this.buf);
    } else {
      return Buffer.from(this._writer.finish());
    }
  }
}

module.exports = {
  Reader,
  Writer,
  isNativeAvailable,
  getImplementationType
};
