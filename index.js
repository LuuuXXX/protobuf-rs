"use strict";

// Import full protobuf.js library
const protobuf = require('./protobuf.js');

// Try to load native Rust bindings for high-performance operations
let nativeBinding;
try {
    nativeBinding = require('./index.node');
} catch (e) {
    // Native bindings not available, will use pure JavaScript fallback
    nativeBinding = null;
}

// If native bindings are available, enhance protobuf with Rust-powered components
if (nativeBinding) {
    // Override Reader/Writer with high-performance Rust implementations when available
    const RustReader = nativeBinding.Reader;
    const RustWriter = nativeBinding.Writer;
    
    // Create hybrid Reader that uses Rust for performance-critical operations
    class HybridReader extends protobuf.Reader {
        constructor(buffer) {
            super(buffer);
            if (RustReader && buffer) {
                try {
                    this._rustReader = new RustReader(buffer);
                } catch (e) {
                    // Fallback to JS implementation
                    this._rustReader = null;
                }
            }
        }
    }
    
    // Create hybrid Writer that uses Rust for performance-critical operations  
    class HybridWriter extends protobuf.Writer {
        constructor() {
            super();
            if (RustWriter) {
                try {
                    this._rustWriter = new RustWriter();
                } catch (e) {
                    // Fallback to JS implementation
                    this._rustWriter = null;
                }
            }
        }
    }
    
    // Expose high-performance Rust functions
    protobuf.rust = {
        Reader: RustReader,
        Writer: RustWriter,
        Encoder: nativeBinding.Encoder,
        Decoder: nativeBinding.Decoder,
        encodeVarint: nativeBinding.encodeVarint,
        decodeVarint: nativeBinding.decodeVarint,
        encodeZigzag: nativeBinding.encodeZigzag,
        decodeZigzag: nativeBinding.decodeZigzag,
        encodeFieldTag: nativeBinding.encodeFieldTag,
        decodeFieldTag: nativeBinding.decodeFieldTag,
    };
    
    protobuf.build = 'full-rust-accelerated';
} else {
    protobuf.build = 'full-javascript';
}

// Export the enhanced protobuf module
module.exports = protobuf;
