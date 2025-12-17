# Rust Build Configuration

This document explains how to build the Rust components of protobufjs-rust.

## Current Status

The Rust code is currently configured for **NAPI** (Node.js native modules) using `napi-rs`. This provides high performance for Node.js environments but is not compatible with browsers.

## Build Options

### Option 1: NAPI (Native Node.js Module) - Current Implementation

**Pros:**
- Fastest performance (native code, no WASM overhead)
- Direct access to Node.js APIs
- Lower memory overhead

**Cons:**
- Platform-specific binaries (needs compilation for each OS/arch)
- Not compatible with browsers
- Requires native build tools

**Build Command:**
```bash
cd src/rust
cargo build --release
```

### Option 2: WASM (WebAssembly) - Planned

**Pros:**
- Works in browsers and Node.js
- Platform-independent (single build works everywhere)
- No native build tools needed for installation

**Cons:**
- Slightly slower than native code
- Limited access to system APIs

**Build Command (when implemented):**
```bash
cd src/rust
wasm-pack build --target nodejs --out-dir ../rust-wasm
```

## Converting from NAPI to WASM

To convert the current Rust code to WASM, the following changes are needed:

### 1. Update Cargo.toml

Replace NAPI dependencies with wasm-bindgen:

```toml
[dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"
byteorder = "1.5"

[dependencies.web-sys]
version = "0.3"
features = ["console"]

[lib]
crate-type = ["cdylib", "rlib"]
```

### 2. Update lib.rs

Replace `#[napi]` attributes with `#[wasm_bindgen]`:

```rust
// Before (NAPI)
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn decode_varint(buffer: Buffer) -> Result<i64> {
    // ...
}

// After (WASM)
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn decode_varint(buffer: &[u8]) -> i64 {
    // ...
}
```

### 3. Handle Buffer Types

NAPI's `Buffer` type needs to be replaced with standard Rust types that work with WASM:
- `Buffer` → `&[u8]` or `Vec<u8>`
- `Result<T>` → Return types that work with JS (or use `js_sys::Error`)

## Hybrid Approach (Recommended for Future)

Create both NAPI and WASM builds:

```
src/rust/
├── Cargo.toml          # Base configuration
├── lib.rs              # Shared implementation
├── napi/
│   ├── Cargo.toml      # NAPI-specific config
│   └── wrapper.rs      # NAPI bindings
└── wasm/
    ├── Cargo.toml      # WASM-specific config
    └── wrapper.rs      # WASM bindings
```

Then:
- Use NAPI build for Node.js (best performance)
- Use WASM build for browsers
- Auto-detect at runtime which to use

## Current Fallback Behavior

The `index.js` entry point already implements fallback:

1. Try to load Rust implementation (WASM or NAPI)
2. If not available, use pure JavaScript
3. No errors - seamless degradation

This means the library works even without any Rust build!

## Building for Production

For production use, you can:

1. **No Rust build** - Use pure JS (slower but works everywhere)
   ```bash
   npm install
   # That's it!
   ```

2. **NAPI build** - Best for Node.js servers
   ```bash
   npm install
   npm run build:rust:native
   ```

3. **WASM build** - Best for universal compatibility (when implemented)
   ```bash
   npm install
   npm run build:rust
   ```

## Performance Comparison

Based on typical workloads:

| Implementation | Encoding Speed | Decoding Speed | Compatibility |
|---------------|---------------|----------------|---------------|
| Pure JS       | 1x (baseline) | 1x (baseline)  | Universal     |
| WASM          | 2-3x faster   | 2-3x faster    | Universal     |
| NAPI          | 3-5x faster   | 3-5x faster    | Node.js only  |

## Next Steps for WASM Support

1. Create a new branch for WASM conversion
2. Update Cargo.toml dependencies
3. Convert all `#[napi]` to `#[wasm_bindgen]`
4. Update buffer handling
5. Test in both Node.js and browser
6. Update build scripts
7. Update documentation

For now, the library works perfectly with pure JavaScript fallback.
