// protobufjs-rust: High-performance Rust enhancement for protobuf.js
// This file provides automatic Rust WASM acceleration with seamless fallback to pure JavaScript

"use strict";

// Load the base protobuf.js implementation
const protobufjs = require("./src/protobuf-js/index");

// Attempt to load Rust WASM implementation
let rustImpl = null;
let hasRustAcceleration = false;

try {
  // Try to load Rust WASM module
  rustImpl = require('./src/rust-wasm');
  hasRustAcceleration = true;
  console.info('protobufjs-rust: Using high-performance Rust WASM implementation');
} catch (e) {
  // Rust WASM not available, will use pure JS implementation
  if (process.env.PROTOBUF_DEBUG) {
    console.warn('protobufjs-rust: Rust WASM not available, using pure JavaScript implementation');
    console.warn('To build Rust WASM: npm run build:rust');
  }
}

// Create wrapper for Reader if Rust implementation is available
if (hasRustAcceleration && rustImpl.Reader) {
  const OriginalReader = protobufjs.Reader;
  const RustReader = rustImpl.Reader;
  
  // Wrapper that tries Rust first, falls back to JS
  protobufjs.Reader = function Reader(buffer) {
    try {
      return new RustReader(buffer);
    } catch (e) {
      if (process.env.PROTOBUF_DEBUG) {
        console.warn('Rust Reader failed, falling back to JS:', e.message);
      }
      return new OriginalReader(buffer);
    }
  };
  
  // Copy static methods
  Object.setPrototypeOf(protobufjs.Reader, OriginalReader);
  Object.assign(protobufjs.Reader, OriginalReader);
}

// Create wrapper for Writer if Rust implementation is available
if (hasRustAcceleration && rustImpl.Writer) {
  const OriginalWriter = protobufjs.Writer;
  const RustWriter = rustImpl.Writer;
  
  // Wrapper that tries Rust first, falls back to JS
  protobufjs.Writer = function Writer() {
    try {
      return new RustWriter();
    } catch (e) {
      if (process.env.PROTOBUF_DEBUG) {
        console.warn('Rust Writer failed, falling back to JS:', e.message);
      }
      return new OriginalWriter();
    }
  };
  
  // Copy static methods
  Object.setPrototypeOf(protobufjs.Writer, OriginalWriter);
  Object.assign(protobufjs.Writer, OriginalWriter);
}

// Add build information
protobufjs.build = hasRustAcceleration ? 'rust-wasm-enhanced' : 'javascript';
protobufjs.rustAcceleration = hasRustAcceleration;

// Export the enhanced protobufjs
module.exports = protobufjs;
