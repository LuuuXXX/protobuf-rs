"use strict";

// Export native bindings
let nativeBinding;
try {
    nativeBinding = require('./index.node');
} catch (e) {
    console.warn('Native binding not available, some features will be unavailable');
    nativeBinding = {};
}

// Export basic functions
module.exports = {
    // Native functions
    encodeVarint: nativeBinding.encodeVarint,
    decodeVarint: nativeBinding.decodeVarint,
    encodeZigzag: nativeBinding.encodeZigzag,
    decodeZigzag: nativeBinding.decodeZigzag,
    encodeFieldTag: nativeBinding.encodeFieldTag,
    decodeFieldTag: nativeBinding.decodeFieldTag,
    
    // Wrapped Reader/Writer
    Reader: require('./src/reader'),
    Writer: require('./src/writer'),
    
    // Info
    build: 'rust-ohos',
    version: '1.0.0',
};
