# protobuf-rs OpenHarmony Port - Development Roadmap

This document outlines the complete development roadmap for porting protobuf-rs to OpenHarmony.

## Overview

The project is structured in multiple phases to ensure a clean, maintainable, and high-performance Protocol Buffers implementation for OpenHarmony.

---

## Phase 1: Migration to ohos-rs ✅ COMPLETED

**Goal**: Migrate from standard NAPI-RS to ohos-rs bindings

### Tasks Completed
- ✅ Updated dependencies to use ohos-rs ecosystem
- ✅ Verified core functionality works with new bindings
- ✅ Ensured cross-platform compatibility

---

## Phase 2: Core Simplification 🚧 IN PROGRESS

**Goal**: Simplify Rust core and create JavaScript wrapper layer

### Task 1: Clean Up Rust Code
- ✅ Create `future/` directory for v1.1 features
- ✅ Move `src/simd.rs` → `future/simd.rs`
- ✅ Move `src/parallel.rs` → `future/parallel.rs`
- ✅ Keep `src/pool.rs` with `#[allow(dead_code)]`
- ✅ Update `src/lib.rs` to remove Phase 3 exports
- ✅ Enhance Reader with complete API:
  - uint32, int32, sint32
  - uint64, int64, sint64
  - bool
  - fixed32, sfixed32, fixed64, sfixed64
  - float, double
  - bytes, string
  - skip, skip_type, reset
- ✅ Enhance Writer with complete API:
  - uint32, int32, sint32
  - uint64, int64, sint64
  - bool
  - fixed32, sfixed32, fixed64, sfixed64
  - float, double
  - bytes, string
  - fork, ldelim
  - reset, finish

### Task 2: Create JavaScript Wrapper Layer
- ✅ Create `src/reader.js` - wrapper with native fallback
- ✅ Create `src/writer.js` - wrapper with native fallback
- ✅ Create simplified `index.js` entry point

### Task 3: Clean Up Old Files
- ✅ Delete `protobufjs-compat.js`
- ✅ Delete `integration/` directory
- ✅ Delete `benchmarks/` directory
- ✅ Delete `examples/` directory
- ✅ Delete `test/` directory

### Task 4: Update Configuration
- ✅ Update `Cargo.toml` for OpenHarmony target
- ✅ Update `package.json` with simplified scripts
- ⏳ Update `README.md` with Phase 2 status
- ⏳ Update `CHANGELOG.md` with Phase 2 entry

### Task 5: Documentation
- ✅ Create `ROADMAP.md` (this file)

---

## Phase 3: Integrate protobufjs Code ⏳ PLANNED

**Goal**: Integrate protobufjs pure JavaScript implementation as fallback

### Tasks
- [ ] Copy protobufjs Reader implementation
- [ ] Copy protobufjs Writer implementation
- [ ] Update wrappers to use protobufjs fallback
- [ ] Add automatic detection and switching
- [ ] Ensure 100% API compatibility
- [ ] Test hybrid operation (Rust + JS)

### Expected Structure
```
src/
  reader.js         - Smart wrapper
  writer.js         - Smart wrapper
  protobuf/
    reader.js       - protobufjs Reader (pure JS)
    writer.js       - protobufjs Writer (pure JS)
    util/           - protobufjs utilities
```

---

## Phase 4: Testing ⏳ PLANNED

**Goal**: Comprehensive test suite for all functionality

### Tasks
- [ ] Unit tests for Rust core
  - [ ] Varint encoding/decoding
  - [ ] ZigZag encoding
  - [ ] Field tag operations
  - [ ] Reader methods
  - [ ] Writer methods
- [ ] Integration tests
  - [ ] Rust-only mode tests
  - [ ] JS-only mode tests
  - [ ] Hybrid mode tests
  - [ ] Fallback mechanism tests
- [ ] Compatibility tests
  - [ ] protobufjs compatibility
  - [ ] Cross-platform tests
- [ ] Performance tests
  - [ ] Benchmarks vs pure JS
  - [ ] Memory usage tests
  - [ ] Stress tests

---

## Phase 5: Documentation ⏳ PLANNED

**Goal**: Complete documentation for users and developers

### Tasks
- [ ] API Reference
  - [ ] Reader API
  - [ ] Writer API
  - [ ] Encoding functions
  - [ ] Decoding functions
- [ ] User Guide
  - [ ] Installation instructions
  - [ ] Basic usage examples
  - [ ] Advanced usage patterns
  - [ ] Migration guide
- [ ] Developer Guide
  - [ ] Building from source
  - [ ] Architecture overview
  - [ ] Contributing guidelines
  - [ ] Testing guide
- [ ] Performance Guide
  - [ ] Benchmark results
  - [ ] Optimization tips
  - [ ] Best practices

---

## Phase 6: Examples and Benchmarks ⏳ PLANNED

**Goal**: Practical examples and performance benchmarks

### Tasks
- [ ] Create examples/
  - [ ] Basic encode/decode
  - [ ] Message serialization
  - [ ] Streaming processing
  - [ ] Real-world use cases
- [ ] Create benchmarks/
  - [ ] vs protobufjs
  - [ ] vs other implementations
  - [ ] Memory usage
  - [ ] CPU profiling

---

## Future Enhancements (v1.1+) 💡 IDEAS

### SIMD Optimization (from future/simd.rs)
- Hardware SIMD support (AVX2/NEON)
- Batch encoding/decoding
- 40-60x performance improvement for batch operations

### Parallel Processing (from future/parallel.rs)
- Multi-threaded encoding/decoding
- Work-stealing thread pool
- Near-linear scaling on multi-core systems

### Memory Pool (from src/pool.rs)
- Buffer pooling API
- Reduced allocations
- Better memory efficiency

### Additional Features
- Custom allocators
- Streaming API
- Built-in metrics
- WebAssembly support
- ARM-specific optimizations

---

## Current Status Summary

| Phase | Status | Progress |
|-------|--------|----------|
| Phase 1: ohos-rs Migration | ✅ Complete | 100% |
| Phase 2: Core Simplification | 🚧 In Progress | 90% |
| Phase 3: protobufjs Integration | ⏳ Planned | 0% |
| Phase 4: Testing | ⏳ Planned | 0% |
| Phase 5: Documentation | ⏳ Planned | 0% |
| Phase 6: Examples & Benchmarks | ⏳ Planned | 0% |

---

## Version Plan

- **v1.0.0** - Phase 1-3 complete, basic functionality
- **v1.0.1** - Phase 4 complete, tested and stable
- **v1.1.0** - SIMD optimization
- **v1.2.0** - Parallel processing
- **v1.3.0** - Memory pool API
- **v2.0.0** - Full feature set with all enhancements

---

## Contributing

This project is under active development. For questions or contributions, please see the main README.md or open an issue on GitHub.

## License

MIT License - See LICENSE file for details.
