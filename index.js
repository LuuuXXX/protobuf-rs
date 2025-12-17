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
    // Optimized batch operation approach: accumulate all operations in JS and send to Rust once
    class RustWriter {
        constructor() {
            // Accumulate operations instead of calling Rust immediately
            this._operations = [];
            this._len = 0;
            this._states = [];  // Support fork/ldelim
        }
        
        static create() {
            return new RustWriter();
        }
        
        uint32(value) {
            value = value >>> 0;
            this._operations.push(['u32', value]);
            // Pre-calculate length (same as original JS implementation)
            this._len += value < 128 ? 1
                      : value < 16384 ? 2
                      : value < 2097152 ? 3
                      : value < 268435456 ? 4 : 5;
            return this;
        }
        
        int32(value) {
            if (value < 0) {
                // Negative numbers encode to 10 bytes
                this._operations.push(['i32', value]);
                this._len += 10;
            } else {
                return this.uint32(value);
            }
            return this;
        }
        
        sint32(value) {
            const encoded = ((value << 1) ^ (value >> 31)) >>> 0;
            return this.uint32(encoded);
        }
        
        uint64(value) {
            this._operations.push(['u64', value]);
            // Simplified: max 10 bytes
            this._len += 10;
            return this;
        }
        
        int64(value) {
            this._operations.push(['i64', value]);
            this._len += 10;
            return this;
        }
        
        sint64(value) {
            this._operations.push(['s64', value]);
            this._len += 10;
            return this;
        }
        
        bool(value) {
            this._operations.push(['bool', value]);
            this._len += 1;
            return this;
        }
        
        fixed32(value) {
            this._operations.push(['f32', value >>> 0]);
            this._len += 4;
            return this;
        }
        
        sfixed32(value) {
            return this.fixed32(value);
        }
        
        fixed64(value) {
            this._operations.push(['f64', value]);
            this._len += 8;
            return this;
        }
        
        sfixed64(value) {
            return this.fixed64(value);
        }
        
        float(value) {
            this._operations.push(['float', value]);
            this._len += 4;
            return this;
        }
        
        double(value) {
            this._operations.push(['double', value]);
            this._len += 8;
            return this;
        }
        
        bytes(value) {
            if (!value || value.length === 0) {
                return this.uint32(0);
            }
            
            let buffer;
            if (typeof value === 'string') {
                buffer = Buffer.from(value, 'base64');
            } else if (Array.isArray(value)) {
                buffer = Buffer.from(value);
            } else {
                buffer = value;
            }
            
            this._operations.push(['bytes', buffer]);
            const len = buffer.length;
            this._len += (len < 128 ? 1 : len < 16384 ? 2 : len < 2097152 ? 3 : len < 268435456 ? 4 : 5) + len;
            return this;
        }
        
        string(value) {
            const len = Buffer.byteLength(value, 'utf8');
            if (len === 0) {
                return this.uint32(0);
            }
            this._operations.push(['string', value]);
            this._len += (len < 128 ? 1 : len < 16384 ? 2 : len < 2097152 ? 3 : len < 268435456 ? 4 : 5) + len;
            return this;
        }
        
        fork() {
            this._states.push({
                operations: this._operations.slice(),
                len: this._len
            });
            this._operations = [];
            this._len = 0;
            return this;
        }
        
        reset() {
            if (this._states.length > 0) {
                const state = this._states.pop();
                this._operations = state.operations;
                this._len = state.len;
            } else {
                this._operations = [];
                this._len = 0;
            }
            return this;
        }
        
        ldelim() {
            const forkOps = this._operations;
            const forkLen = this._len;
            
            this.reset();
            this.uint32(forkLen);
            
            // Merge fork operations
            this._operations.push(...forkOps);
            this._len += forkLen;
            
            return this;
        }
        
        finish() {
            // 💥 Key optimization: only cross FFI boundary once!
            return nativeBinding.Writer.encodeAll(this._operations);
        }
        
        get len() {
            return this._len;
        }
        
        static alloc(size) {
            return Buffer.allocUnsafe(size);
        }
        
        static _configure() {
            // Compatibility placeholder
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
        
        static _configure(BufferReader_) {
            // For compatibility with original Reader._configure
            // The Rust implementation doesn't need this, but we keep it for compatibility
            return;
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
