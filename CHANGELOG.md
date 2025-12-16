# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - Phase 2: Core Simplification

### Added

#### Phase 2: Rust Core Simplification & JS Wrappers
- **Future Directory Structure**
  - Created `future/` directory for v1.1+ features
  - Moved `simd.rs` to `future/simd.rs` (SIMD batch operations)
  - Moved `parallel.rs` to `future/parallel.rs` (parallel processing)
  - Kept `pool.rs` in src/ with `#[allow(dead_code)]` annotation

- **Enhanced Reader API**
  - Added complete Protocol Buffer type support:
    - `int32()`, `sint32()` - Signed 32-bit integers
    - `uint64()`, `int64()`, `sint64()` - 64-bit integers
    - `bool()` - Boolean values
    - `fixed32()`, `sfixed32()` - Fixed 32-bit values
    - `fixed64()`, `sfixed64()` - Fixed 64-bit values
    - `float()`, `double()` - Floating point values
    - `skip_type(wire_type)` - Skip field by wire type
  - All existing methods retained: `uint32()`, `bytes()`, `string()`, `skip()`, `reset()`

- **Enhanced Writer API**
  - Added complete Protocol Buffer type support:
    - `int32()`, `sint32()` - Signed 32-bit integers
    - `uint64()`, `int64()`, `sint64()` - 64-bit integers
    - `bool()` - Boolean values
    - `fixed32()`, `sfixed32()` - Fixed 32-bit values
    - `fixed64()`, `sfixed64()` - Fixed 64-bit values
    - `float()`, `double()` - Floating point values
    - `fork()`, `ldelim()` - Length-delimited message support
  - All existing methods retained: `uint32()`, `bytes()`, `string()`, `reset()`, `finish()`

- **JavaScript Wrapper Layer**
  - Created `src/reader.js` - Smart wrapper with native/JS fallback
  - Created `src/writer.js` - Smart wrapper with native/JS fallback
  - New simplified `index.js` entry point
  - Automatic fallback to pure JS when native unavailable
  - `_useNative` flag for testing native availability

- **Documentation**
  - Created comprehensive `ROADMAP.md` with complete development plan
  - Updated README.md with Phase 2 status and current structure
  - Documented all new Reader/Writer methods

### Changed

- **Simplified Rust Core**
  - Removed Phase 3 exports from `src/lib.rs`
  - Now exports only: `Reader`, `Writer`, basic varint/zigzag/field_tag functions
  - Cleaner module structure focused on core functionality

- **Configuration Updates**
  - Updated `Cargo.toml`:
    - Package renamed to `protobuf-rs-ohos`
    - Version set to 1.0.0
    - Removed unused dependencies (`rayon`, `parking_lot`)
    - Optimized release profile (LTO, strip, opt-level 3)
  - Updated `package.json`:
    - Package renamed to `@protobuf-rs/ohos-core`
    - Simplified scripts (build, build:debug only)
    - Removed Node.js-specific tooling
    - Updated description for OpenHarmony target

### Removed

- **Node.js-Specific Files**
  - Deleted `protobufjs-compat.js` (Node.js compatibility layer)
  - Deleted `integration/` directory (Node.js adapters)
  - Deleted `benchmarks/` directory (will recreate in Phase 6)
  - Deleted `examples/` directory (will recreate in Phase 6)
  - Deleted `test/` directory (will recreate in Phase 4)

- **Dependencies**
  - Removed `rayon` (parallel processing - moved to future/)
  - Removed `parking_lot` (synchronization - moved to future/)

### Migration Notes

This is a major restructuring for OpenHarmony compatibility:

1. **For Users**: The core API remains the same. Reader and Writer now have complete Protocol Buffer support.
2. **For Developers**: Advanced features (SIMD, parallel) moved to `future/` directory for v1.1+
3. **Build Target**: Configured for OpenHarmony (`aarch64-linux-ohos` when toolchain available)

### Next Steps (Phase 3)

- Integrate protobufjs pure JavaScript implementation
- Add comprehensive fallback mechanism
- Ensure 100% API compatibility
- Create test suite

---

## [1.0.0] - 2024-12-14

### Added

#### Phase 3: Advanced Performance Optimization
- **SIMD Optimization Module** (`src/simd.rs`)
  - Batch varint encoding: `encodeVarintBatchSimd()`
  - Batch varint decoding: `decodeVarintBatchSimd()`
  - Runtime CPU feature detection with graceful fallback
  - Foundation for 40-60x batch speedup

- **Zero-Copy Optimizations**
  - New `Reader` class with zero-copy methods
  - New `Writer` class with buffer optimization
  - `bytes()` and `string()` methods with reduced allocations
  - Pre-allocation support via `Writer.withCapacity()`

- **Memory Pool Implementation** (`src/pool.rs`)
  - Thread-safe buffer pooling using parking_lot
  - Size-class based pools (powers of 2)
  - RAII pattern with automatic cleanup
  - Metrics tracking (hits, misses, utilization)

- **Parallel Processing Support** (`src/parallel.rs`)
  - `encodeVarintsParallel()` for parallel encoding
  - `decodeVarintsParallel()` for parallel decoding
  - `processU32BatchParallel()` for chunked processing
  - Work-stealing thread pool using rayon

- **Comprehensive Benchmarks**
  - Real-world scenarios: gRPC, batch export, streaming, low-memory
  - Memory profiling: allocation tracking, leak detection, GC monitoring
  - CPU profiling: utilization tracking, hotspot identification

- **Documentation**
  - Complete performance report with benchmark results
  - Methodology and reproducibility guidelines
  - Competitor comparison analysis
  - Real-world case studies
  - Best practices and optimization tips

#### Phase 2: Production-Ready Integration (Previously Released)
- **Hybrid Adapter** for drop-in protobuf.js compatibility
- Automatic fallback to JavaScript when native unavailable
- Performance monitoring tools
- Comprehensive integration guide
- Migration examples

#### Phase 1: Core Functionality (Previously Released)
- High-performance varint encoding/decoding
- ZigZag encoding for signed integers
- Field tag encoding/decoding
- Protocol Buffer message parsing
- Cross-platform NAPI-RS bindings
- TypeScript type definitions

### Performance

Measured improvements over pure JavaScript implementations:

#### Throughput
- **Single encode/decode:** 3-15x faster
- **Batch operations:** 1.85x faster (foundation for 40-60x with hardware SIMD)
- **gRPC scenario:** 3.14x faster (289K ops/sec)
- **Reader operations:** 621K ops/sec
- **Writer operations:** 397K ops/sec

#### Latency
- **P50 latency:** 1.53µs
- **P95 latency:** 2.48µs
- **P99 latency:** 23.63µs
- **CPU consistency:** 3.5% coefficient of variation

#### Memory
- **Heap efficiency:** 314% improvement (negative growth vs positive)
- **Per-allocation overhead:** 2 bytes average
- **External memory:** 0.12 MB per 1,000 buffers
- **Memory leak:** None detected (trend: -2.15%)

#### CPU
- **Utilization:** Consistent 112.3% (±3.88%)
- **Writer CPU:** 2.6µs per operation
- **Reader CPU:** 1.66µs per operation

### Dependencies

Added for Phase 3:
- `rayon = "1.11"` - Parallel processing
- `parking_lot = "0.12"` - High-performance synchronization
- `byteorder = "1.5"` - Endian-aware I/O

### Changed

- Updated `Cargo.toml` with Phase 3 dependencies
- Enhanced `index.js` to export new Phase 3 functions
- Updated `package.json` metadata for v1.0.0 release
- **License changed from MIT to BSD-3-Clause** to align with project requirements

### Testing

- ✅ All 74 existing tests passing
- ✅ Real-world benchmarks validated
- ✅ Memory profiling shows no leaks
- ✅ CPU profiling shows consistent performance
- ✅ Cross-platform builds verified

### Documentation

- Added `docs/PERFORMANCE_REPORT.md` with comprehensive analysis
- Added benchmark scripts in `benchmarks/` directory
- Updated README.md with performance highlights

## [0.2.0] - Phase 2 (Previously Released)

### Added
- protobuf.js compatibility adapter
- Performance monitoring utilities
- Integration guide
- Migration examples
- Automatic fallback mechanism

### Performance
- 10-20x faster than pure JavaScript
- 100% API compatibility

## [0.1.0] - Phase 1 (Previously Released)

### Added
- Core varint encoding/decoding
- ZigZag encoding for signed integers
- Field tag operations
- Protocol Buffer parser
- NAPI-RS bindings
- Basic test suite

### Performance
- 10x+ improvement over JavaScript implementations

---

## Upgrade Guide

### From 0.2.x to 1.0.0

Phase 3 adds new features while maintaining full backward compatibility:

```javascript
// Existing code continues to work
const { encodeVarint, decodeVarint } = require('protobuf-rs');

// New Phase 3 features (optional)
const { 
    Reader, 
    Writer,
    encodeVarintBatchSimd,
    processU32BatchParallel 
} = require('protobuf-rs');

// Use new Reader/Writer for better performance
const writer = new Writer();
writer.uint32(100);
const buffer = writer.finish();

const reader = new Reader(buffer);
const value = reader.uint32();
```

### From 0.1.x to 1.0.0

Update to use the hybrid adapter for protobuf.js compatibility:

```javascript
// Before (Phase 1)
const { encodeVarint } = require('protobuf-rs');

// After (Phase 3) - both old and new APIs work
const { encodeVarint, Reader, Writer } = require('protobuf-rs');
// Or use the adapter
const { Reader, Writer } = require('protobuf-rs/integration/protobufjs-adapter');
```

---

## Future Roadmap

### Planned for v1.1.0
- Hardware SIMD acceleration (AVX2/NEON)
- Buffer pooling API
- Enhanced parallel processing
- Streaming API
- Built-in metrics

### Under Consideration
- Custom allocators
- gRPC integration
- WebAssembly support
- ARM-specific optimizations

---

[1.0.0]: https://github.com/LuuuXXX/protobuf-rs/releases/tag/v1.0.0
[0.2.0]: https://github.com/LuuuXXX/protobuf-rs/releases/tag/v0.2.0
[0.1.0]: https://github.com/LuuuXXX/protobuf-rs/releases/tag/v0.1.0
