// full library entry point with Rust acceleration support

"use strict";

let nativeBinding = null;
let usingRust = false;

// Try to load native Rust module
try {
    nativeBinding = require('./index.node');
    usingRust = true;
} catch (err) {
    // Silent fallback to JavaScript implementation
}

const protobuf = require("./src/index");

if (usingRust) {
    // Save original classes for reference
    const OriginalReader = protobuf.Reader;
    const OriginalWriter = protobuf.Writer;
    const OriginalBufferReader = protobuf.BufferReader;
    const OriginalBufferWriter = protobuf.BufferWriter;
    
    // Wrap native Writer to ensure chainability and API compatibility
    class RustWriter {
        constructor() {
            this._native = new nativeBinding.Writer();
        }
        
        static create() {
            return new RustWriter();
        }
        
        uint32(value) {
            this._native.uint32(value);
            return this;
        }
        
        int32(value) {
            this._native.int32(value);
            return this;
        }
        
        sint32(value) {
            this._native.sint32(value);
            return this;
        }
        
        uint64(value) {
            // Handle Long objects from long.js
            if (value && typeof value === 'object' && value.low !== undefined && value.high !== undefined) {
                // Use the low and high parts directly
                const num = (value.low >>> 0) + (value.high >>> 0) * 0x100000000;
                this._native.uint64(num);
            } else {
                this._native.uint64(value);
            }
            return this;
        }
        
        int64(value) {
            // Handle Long objects from long.js
            if (value && typeof value === 'object' && value.low !== undefined && value.high !== undefined) {
                const num = (value.low >>> 0) + (value.high | 0) * 0x100000000;
                this._native.int64(num);
            } else {
                this._native.int64(value);
            }
            return this;
        }
        
        sint64(value) {
            // Handle Long objects from long.js
            if (value && typeof value === 'object' && value.low !== undefined && value.high !== undefined) {
                const num = (value.low >>> 0) + (value.high | 0) * 0x100000000;
                this._native.sint64(num);
            } else {
                this._native.sint64(value);
            }
            return this;
        }
        
        bool(value) {
            this._native.bool(value);
            return this;
        }
        
        fixed32(value) {
            this._native.fixed32(value);
            return this;
        }
        
        sfixed32(value) {
            this._native.sfixed32(value);
            return this;
        }
        
        fixed64(value) {
            // Handle Long objects from long.js
            if (value && typeof value === 'object' && value.low !== undefined && value.high !== undefined) {
                const num = (value.low >>> 0) + (value.high >>> 0) * 0x100000000;
                this._native.fixed64(num);
            } else {
                this._native.fixed64(value);
            }
            return this;
        }
        
        sfixed64(value) {
            // Handle Long objects from long.js
            if (value && typeof value === 'object' && value.low !== undefined && value.high !== undefined) {
                const num = (value.low >>> 0) + (value.high | 0) * 0x100000000;
                this._native.sfixed64(num);
            } else {
                this._native.sfixed64(value);
            }
            return this;
        }
        
        float(value) {
            this._native.float(value);
            return this;
        }
        
        double(value) {
            this._native.double(value);
            return this;
        }
        
        bytes(value) {
            // Handle different input types like the original Writer
            if (!value || value.length === 0) {
                this._native.uint32(0);
                return this;
            }
            
            // Convert to Buffer if needed
            let buffer;
            if (typeof value === 'string') {
                // Base64 encoded string
                buffer = Buffer.from(value, 'base64');
            } else if (Array.isArray(value)) {
                // Plain array
                buffer = Buffer.from(value);
            } else {
                // Already a Buffer or Uint8Array
                buffer = value;
            }
            
            // Write length prefix
            this._native.uint32(buffer.length);
            // Write bytes
            this._native.bytes(buffer);
            return this;
        }
        
        string(value) {
            this._native.string(value);
            return this;
        }
        
        fork() {
            this._native.fork();
            return this;
        }
        
        reset() {
            this._native.reset();
            return this;
        }
        
        ldelim() {
            this._native.ldelim();
            return this;
        }
        
        finish() {
            return this._native.finish();
        }
        
        get len() {
            return this._native.len;
        }
    }
    
    // Wrap native Reader to ensure API compatibility
    class RustReader {
        constructor(buffer) {
            // Convert array to Buffer if needed
            if (Array.isArray(buffer)) {
                buffer = Buffer.from(buffer);
            }
            this._native = new nativeBinding.Reader(buffer);
        }
        
        static create(buffer) {
            // If it's already a Reader instance, return it as-is
            if (buffer instanceof RustReader || buffer instanceof OriginalReader || buffer instanceof OriginalBufferReader) {
                return buffer;
            }
            return new RustReader(buffer);
        }
        
        uint32() {
            return this._native.uint32();
        }
        
        int32() {
            return this._native.int32();
        }
        
        sint32() {
            return this._native.sint32();
        }
        
        uint64() {
            const value = this._native.uint64();
            // Convert to Long if available
            if (protobuf.util.Long) {
                return protobuf.util.Long.fromBits(value & 0xFFFFFFFF, Math.floor(value / 0x100000000), true);
            }
            return value;
        }
        
        int64() {
            const value = this._native.int64();
            // Convert to Long if available
            if (protobuf.util.Long) {
                const low = value & 0xFFFFFFFF;
                const high = Math.floor(value / 0x100000000);
                return protobuf.util.Long.fromBits(low, high, false);
            }
            return value;
        }
        
        sint64() {
            const value = this._native.sint64();
            // Convert to Long if available
            if (protobuf.util.Long) {
                const low = value & 0xFFFFFFFF;
                const high = Math.floor(value / 0x100000000);
                return protobuf.util.Long.fromBits(low, high, false);
            }
            return value;
        }
        
        bool() {
            return this._native.bool();
        }
        
        fixed32() {
            return this._native.fixed32();
        }
        
        sfixed32() {
            return this._native.sfixed32();
        }
        
        fixed64() {
            const value = this._native.fixed64();
            // Convert to Long if available
            if (protobuf.util.Long) {
                const low = value & 0xFFFFFFFF;
                const high = Math.floor(value / 0x100000000);
                return protobuf.util.Long.fromBits(low, high, true);
            }
            return value;
        }
        
        sfixed64() {
            const value = this._native.sfixed64();
            // Convert to Long if available
            if (protobuf.util.Long) {
                const low = value & 0xFFFFFFFF;
                const high = Math.floor(value / 0x100000000);
                return protobuf.util.Long.fromBits(low, high, false);
            }
            return value;
        }
        
        float() {
            return this._native.float();
        }
        
        double() {
            return this._native.double();
        }
        
        bytes() {
            return this._native.bytes();
        }
        
        string() {
            return this._native.string();
        }
        
        skip(length) {
            this._native.skip(length);
            return this;
        }
        
        skipType(wireType) {
            this._native.skipType(wireType);
            return this;
        }
        
        get pos() {
            return this._native.pos;
        }
        
        get len() {
            return this._native.len;
        }
    }
    
    // Make RustReader inherit from OriginalReader for instanceof checks
    Object.setPrototypeOf(RustReader.prototype, OriginalReader.prototype);
    Object.setPrototypeOf(RustReader, OriginalReader);
    
    // Replace protobuf Writer and Reader with Rust versions
    protobuf.Writer = RustWriter;
    protobuf.Reader = RustReader;
    protobuf.BufferWriter = RustWriter;
    protobuf.BufferReader = RustReader;
}

// Export flag for testing/debugging
protobuf.__usingRust = usingRust;

module.exports = protobuf;
