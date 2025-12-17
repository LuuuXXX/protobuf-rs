# protobuf-rs

High-performance Protocol Buffers implementation combining the complete [protobuf.js](https://github.com/protobufjs/protobuf.js) library with Rust-accelerated core operations.

## Architecture

This project provides a hybrid approach that combines:

- **Full protobuf.js source code** (96% of codebase) - Complete Protocol Buffer implementation with all features
- **Rust high-performance modules** (4% of codebase) - Performance-critical operations accelerated with Rust

### Project Structure

```
protobuf-rs/
├── protobuf.js/          # Complete protobuf.js source code (187 JS files)
│   ├── src/              # Core protobuf.js implementation
│   ├── lib/              # Utility libraries
│   ├── cli/              # Command-line tools
│   └── ...
├── src/rust/             # High-performance Rust modules
│   ├── lib.rs            # Module entry point & core functions
│   ├── encoder.rs        # Protocol Buffer encoder
│   ├── decoder.rs        # Protocol Buffer decoder
│   ├── reader.rs         # High-performance reader
│   └── writer.rs         # High-performance writer
├── index.js              # Main entry point (integrates protobuf.js + Rust)
├── Cargo.toml            # Rust build configuration
└── package.json          # NPM package configuration
```

## Features

### From protobuf.js (Full JavaScript Implementation)
- Complete Protocol Buffer support (proto2 & proto3)
- .proto file parsing
- Message encoding/decoding
- Reflection API
- JSON conversion
- TypeScript definitions
- CLI tools (pbjs, pbts)

### Rust Acceleration (High-Performance Core)
- ⚡ Fast varint encoding/decoding
- ⚡ ZigZag encoding for signed integers
- ⚡ Field tag operations
- ⚡ High-performance Reader with zero-copy optimizations
- ⚡ High-performance Writer with buffer management
- ⚡ Encoder/Decoder for all Protocol Buffer types

## Installation

```bash
npm install protobuf-rs
```

## Building from Source

### Prerequisites
- Node.js >= 12.0.0
- Rust toolchain (cargo, rustc)

### Build Steps

```bash
# Install dependencies
npm install

# Build Rust modules (creates index.node)
npm run build

# Or build in debug mode (faster compilation)
npm run build:debug

# Run tests
npm test
```

## Usage

### Basic Usage (protobuf.js API)

The library provides the complete protobuf.js API:

```javascript
const protobuf = require('protobuf-rs');

// Use standard protobuf.js API
const Root = protobuf.Root;
const Type = protobuf.Type;

// Define a message
const root = new Root();
const MyMessage = new Type("MyMessage")
    .add(new protobuf.Field("id", 1, "uint32"))
    .add(new protobuf.Field("name", 2, "string"));

root.define("mypackage").add(MyMessage);

// Encode/decode messages (automatically uses Rust acceleration when available)
const message = MyMessage.create({ id: 1, name: "Test" });
const buffer = MyMessage.encode(message).finish();
const decoded = MyMessage.decode(buffer);
```

### Using Rust Acceleration Directly

When native bindings are available, you can access high-performance Rust functions:

```javascript
const protobuf = require('protobuf-rs');

if (protobuf.rust) {
    console.log('Rust acceleration available!');
    
    // Fast varint operations
    const encoded = protobuf.rust.encodeVarint(300);
    const decoded = protobuf.rust.decodeVarint(encoded);
    
    // High-performance Writer
    const writer = new protobuf.rust.Writer();
    writer.uint32(100);
    writer.string("Hello, World!");
    const buffer = writer.finish();
    
    // High-performance Reader
    const reader = new protobuf.rust.Reader(buffer);
    const num = reader.uint32();
    const str = reader.string();
}
```

## Performance

Rust-accelerated operations provide significant performance improvements:

- **3-15x faster** varint encoding/decoding
- **Zero-copy** reading operations
- **Optimized buffer management** for writing
- Automatic fallback to JavaScript when native modules unavailable

## Build Types

The library supports different build modes:

- `full-rust-accelerated` - protobuf.js + Rust acceleration (best performance)
- `full-javascript` - Pure JavaScript (fallback when native modules not available)

Check build type:
```javascript
const protobuf = require('protobuf-rs');
console.log(protobuf.build); // Shows build type
```

## API Reference

### Core Functions (Rust-accelerated)

- `encodeVarint(value)` - Encode 64-bit integer as varint
- `decodeVarint(buffer)` - Decode varint from buffer
- `encodeZigzag(value)` - ZigZag encode signed integer
- `decodeZigzag(value)` - ZigZag decode signed integer
- `encodeFieldTag(fieldNumber, wireType)` - Encode field tag
- `decodeFieldTag(buffer)` - Decode field tag

### Reader Class

High-performance reader with methods:
- `uint32()`, `int32()`, `sint32()`
- `uint64()`, `int64()`, `sint64()`
- `bool()`, `fixed32()`, `sfixed32()`, `fixed64()`, `sfixed64()`
- `float()`, `double()`
- `bytes()`, `string()`
- `skip(count)`, `skipType(wireType)`, `reset()`

### Writer Class

High-performance writer with methods:
- `uint32(v)`, `int32(v)`, `sint32(v)`
- `uint64(v)`, `int64(v)`, `sint64(v)`
- `bool(v)`, `fixed32(v)`, `sfixed32(v)`, `fixed64(v)`, `sfixed64(v)`
- `float(v)`, `double(v)`
- `bytes(v)`, `string(v)`
- `fork()`, `ldelim()`, `reset()`, `finish()`

### Encoder/Decoder Classes

For advanced use cases, direct Encoder and Decoder classes are available with similar APIs.

## Testing

```bash
# Run integration tests
npm test

# Run protobuf.js tests
node protobuf.js/tests/node.js
```

## Language Statistics

- **JavaScript**: 95.9% (~45,000 lines) - Full protobuf.js implementation
- **Rust**: 4.1% (~1,900 lines) - High-performance core modules

## Dependencies

### Runtime Dependencies
- `@protobufjs/*` packages - Core protobuf.js utilities
- `long` - 64-bit integer support

### Build Dependencies
- Rust toolchain (cargo, rustc)
- NAPI-RS for Rust-Node.js bindings

## License

BSD-3-Clause - See [LICENSE](LICENSE) file for details

## Acknowledgments

- Built on top of [protobuf.js](https://github.com/protobufjs/protobuf.js) - the excellent JavaScript Protocol Buffers implementation
- Uses [NAPI-RS](https://napi.rs/) for seamless Rust-Node.js integration
- Rust modules leverage `serde` for safe serialization

## Contributing

Contributions are welcome! The project maintains a careful balance between JavaScript (protobuf.js features) and Rust (performance optimizations).

## Support

- **Issues**: [GitHub Issues](https://github.com/LuuuXXX/protobuf-rs/issues)
- **protobuf.js Documentation**: https://protobufjs.github.io/protobuf.js/

---

**Made with ❤️ combining JavaScript completeness and Rust performance**
