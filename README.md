# protobufjs-rust

High-performance Protocol Buffers implementation for JavaScript & TypeScript, enhanced with Rust.

This is a fork of [protobuf.js](https://github.com/protobufjs/protobuf.js) with optional Rust WASM acceleration for improved performance.

## Features

- **100% protobuf.js API compatible** - Drop-in replacement for protobuf.js
- **Automatic Rust WASM acceleration** - When available, automatically uses high-performance Rust implementation
- **Seamless fallback** - Falls back to pure JavaScript when Rust/WASM is not available
- **Zero configuration** - Works out of the box with automatic optimization detection
- **All protobuf.js features** - Full support for reflection, static code generation, TypeScript, etc.

## Performance

When Rust WASM acceleration is available, you can expect:
- **Faster encoding/decoding** - Rust's zero-cost abstractions and efficient memory handling
- **Lower memory overhead** - Optimized buffer management
- **Better throughput** - Especially for large messages and batch operations

Performance benchmarks can be run with `npm run bench` (see Benchmarks section below).

## Installation

```bash
npm install protobufjs-rust
```

## Quick Start

Use exactly like protobuf.js - no code changes needed!

```javascript
const protobuf = require("protobufjs-rust");

// Load a .proto file
protobuf.load("awesome.proto", function(err, root) {
    if (err) throw err;

    // Obtain a message type
    const AwesomeMessage = root.lookupType("awesomepackage.AwesomeMessage");

    // Create a message
    const message = AwesomeMessage.create({ awesomeField: "hello" });

    // Encode
    const buffer = AwesomeMessage.encode(message).finish();

    // Decode
    const decoded = AwesomeMessage.decode(buffer);
});
```

The library automatically uses Rust acceleration when available, or falls back to JavaScript.

## Project Structure

```
protobufjs-rust/
├── src/
│   ├── protobuf-js/      # Core protobuf.js implementation (pure JS)
│   ├── rust/             # Rust source code
│   │   ├── lib.rs        # Rust library entry point
│   │   ├── reader.rs     # High-performance reader
│   │   ├── writer.rs     # High-performance writer
│   │   └── Cargo.toml    # Rust package configuration
│   ├── rust-wrapper/     # JS wrappers for Rust functions
│   └── rust-wasm/        # Generated WASM bindings (after build)
├── bench/                # Performance benchmarks
├── cli/                  # Command-line interface (pbjs, pbts)
├── tests/                # Test suite
├── lib/                  # Utility libraries
├── ext/                  # Extensions
├── google/               # Google protocol definitions
├── index.js              # Main entry point with Rust/JS fallback
└── package.json
```

## Building from Source

### Prerequisites

For full Rust WASM acceleration, you need:
- Node.js >= 12.0.0
- Rust toolchain (https://rustup.rs/)
- wasm-pack (optional, for WASM builds)

### Build Commands

```bash
# Install dependencies
npm install

# Build everything (JS bundle + types + Rust WASM)
npm run build

# Build only Rust WASM module
npm run build:rust

# Build only JavaScript bundle
npm run build:bundle

# Build only TypeScript definitions
npm run build:types
```

### Without Rust/WASM

The library works perfectly without Rust/WASM - it will use the pure JavaScript implementation:

```bash
npm install
# Use directly - no build needed for pure JS mode
```

## Benchmarks

Run performance benchmarks to compare encoding/decoding speed:

```bash
npm run bench
```

This will show performance comparisons between:
- protobuf.js (reflect mode)
- protobuf.js (static mode)
- JSON string encoding
- JSON buffer encoding
- google-protobuf

When Rust WASM is built, the benchmarks will use Rust-accelerated operations.

## Testing

```bash
# Run all tests
npm test

# Run only source tests
npm run test:sources

# Run only TypeScript tests
npm run test:types

# Run with coverage
npm run coverage
```

## API Reference

This library maintains 100% API compatibility with protobuf.js. See the full protobuf.js documentation:
- [protobuf.js Documentation](https://protobufjs.github.io/protobuf.js/)
- [API Reference](https://protobufjs.github.io/protobuf.js/)

### Additional Properties

```javascript
const protobuf = require("protobufjs-rust");

// Check if Rust acceleration is active
console.log(protobuf.rustAcceleration); // true or false

// Check build type
console.log(protobuf.build); // "rust-wasm-enhanced" or "javascript"
```

## Environment Variables

- `PROTOBUF_DEBUG` - Set to enable debug logging for Rust fallback behavior

```bash
PROTOBUF_DEBUG=1 node your-app.js
```

## CLI Tools

The CLI tools from protobuf.js are fully available:

```bash
# Generate static code
npx pbjs -t static-module -w commonjs -o compiled.js awesome.proto

# Generate TypeScript definitions
npx pbts -o compiled.d.ts compiled.js
```

See [CLI documentation](https://github.com/protobufjs/protobuf.js#command-line) for details.

## Rust Implementation

The Rust implementation provides high-performance encoding/decoding for:
- Varint encoding/decoding
- ZigZag encoding for signed integers
- Field tag encoding/decoding
- Reader operations (all protobuf types)
- Writer operations (all protobuf types)

### Future Rust Enhancements (Planned)

- SIMD batch operations (40-60x speedup)
- Parallel processing for large messages
- Memory pooling for reduced allocations
- Custom allocators for specific workloads

## Migration from protobuf.js

Simply replace `require("protobufjs")` with `require("protobufjs-rust")`:

```javascript
// Before
const protobuf = require("protobufjs");

// After  
const protobuf = require("protobufjs-rust");
```

Everything else stays the same!

## Contributing

Contributions are welcome! This project maintains the protobuf.js codebase with Rust enhancements.

Areas for contribution:
- Rust performance optimizations
- Additional Rust-accelerated operations
- Benchmark improvements
- Documentation
- Bug fixes

## License

This project maintains the original protobuf.js license:
- **BSD-3-Clause** for the protobuf.js code
- **MIT** for Rust enhancements

See [LICENSE](LICENSE) for details.

## Acknowledgments

- Original [protobuf.js](https://github.com/protobufjs/protobuf.js) by Daniel Wirtz
- Rust enhancements by LuuuXXX

## Links

- [protobuf.js original repository](https://github.com/protobufjs/protobuf.js)
- [Protocol Buffers](https://developers.google.com/protocol-buffers/)
- [Rust](https://www.rust-lang.org/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)
