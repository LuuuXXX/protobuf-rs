"use strict";
module.exports = Reader;

// Try to load native Rust implementation
let NativeReader;
try {
    const native = require('../index.node');
    NativeReader = native.Reader;
} catch (e) {
    // Native module not available, will use JS fallback
    NativeReader = null;
}

function Reader(buffer) {
    if (NativeReader) {
        // Use Rust acceleration
        return new NativeReader(buffer);
    } else {
        // Pure JS fallback (will be replaced in Phase 3)
        this.buf = buffer;
        this.pos = 0;
        this.len = buffer.length;
    }
}

Reader.create = function(buffer) {
    return new Reader(buffer);
};

// Export flag for testing
Reader._useNative = !!NativeReader;
