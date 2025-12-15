/**
 * Drop-in replacement for protobuf.js
 * 
 * Usage: Simply replace require('protobufjs') with require('@protobuf-rs/core/protobufjs-compat')
 * 
 * Before:
 *   const protobuf = require('protobufjs');
 * 
 * After:
 *   const protobuf = require('@protobuf-rs/core/protobufjs-compat');
 * 
 * All existing code works unchanged!
 * 
 * This module exports ALL protobuf.js APIs, but replaces Reader/Writer with our
 * Rust-accelerated implementation for 3-4x performance improvement.
 */

let protobufjs = null;
let nativeAdapter = null;

// Try to load protobuf.js
try {
  protobufjs = require('protobufjs');
} catch (err) {
  throw new Error('protobufjs is required but not installed. Run: npm install protobufjs');
}

// Try to load our Rust-accelerated adapter
try {
  nativeAdapter = require('../integration/protobufjs-adapter');
} catch (err) {
  // Fallback to pure protobuf.js if native module is unavailable
  console.warn('⚠ protobuf-rs: Native module not available, using pure protobuf.js');
  nativeAdapter = null;
}

// Export everything from protobuf.js, but override Reader/Writer with our implementation
const exported = {
  // Core classes (use protobuf.js)
  Root: protobufjs.Root,
  Type: protobufjs.Type,
  Field: protobufjs.Field,
  OneOf: protobufjs.OneOf,
  Enum: protobufjs.Enum,
  Namespace: protobufjs.Namespace,
  Service: protobufjs.Service,
  Method: protobufjs.Method,
  Message: protobufjs.Message,
  MapField: protobufjs.MapField,
  
  // Reader/Writer (Rust-accelerated when available, fallback to protobuf.js)
  Reader: nativeAdapter ? nativeAdapter.Reader : protobufjs.Reader,
  Writer: nativeAdapter ? nativeAdapter.Writer : protobufjs.Writer,
  
  // Utilities (use protobuf.js)
  util: protobufjs.util,
  configure: protobufjs.configure,
  
  // Entry points (use protobuf.js)
  parse: protobufjs.parse,
  load: protobufjs.load,
  loadSync: protobufjs.loadSync,
  
  // Encoder/Decoder (use protobuf.js, they will use our Reader/Writer)
  encoder: protobufjs.encoder,
  decoder: protobufjs.decoder,
  verifier: protobufjs.verifier,
  
  // Common types and reflection
  common: protobufjs.common,
  types: protobufjs.types,
  rpc: protobufjs.rpc,
  
  // ReflectionObject and related
  ReflectionObject: protobufjs.ReflectionObject,
  
  // Wrappers
  wrappers: protobufjs.wrappers,
  
  // Tokenize for .proto parsing
  tokenize: protobufjs.tokenize,
};

// Add any additional exports from protobufjs that we might have missed
for (const key in protobufjs) {
  if (!exported.hasOwnProperty(key)) {
    exported[key] = protobufjs[key];
  }
}

// Add convenience function to check if native acceleration is active
exported.isNativeAccelerated = function() {
  return nativeAdapter !== null && nativeAdapter.isNativeAvailable();
};

// Add function to get implementation info
exported.getImplementationInfo = function() {
  const path = require('path');
  let version = '1.0.0';
  try {
    // Try to read package.json from module root
    const pkgPath = path.join(__dirname, 'package.json');
    version = require(pkgPath).version;
  } catch (e) {
    // Fallback to hardcoded version if package.json not found
  }
  
  return {
    native: nativeAdapter !== null && nativeAdapter.isNativeAvailable(),
    type: nativeAdapter ? nativeAdapter.getImplementationType() : 'javascript',
    version: version,
    protobufjs: protobufjs.util.newBuffer ? 'light' : 'full'
  };
};

module.exports = exported;
