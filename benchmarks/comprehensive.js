/**
 * Comprehensive Benchmark Suite
 * 
 * Complete performance comparison between Rust and JavaScript implementations
 * across various real-world scenarios.
 */

const { 
  encodeVarint, 
  decodeVarint
} = require('../index.js');

const { 
  Reader: RustReader, 
  Writer: RustWriter 
} = require('../integration/protobufjs-adapter');

const protobufjs = require('protobufjs');

// Pure JS implementations for comparison
function encodeVarintJS(value) {
  const result = [];
  let n = value >>> 0;
  
  while (true) {
    let byte = n & 0x7F;
    n >>>= 7;
    
    if (n !== 0) {
      byte |= 0x80;
    }
    
    result.push(byte);
    
    if (n === 0) {
      break;
    }
  }
  
  return Buffer.from(result);
}

function decodeVarintJS(buffer) {
  let result = 0;
  let shift = 0;
  
  for (let i = 0; i < buffer.length; i++) {
    const byte = buffer[i];
    result |= (byte & 0x7F) << shift;
    
    if ((byte & 0x80) === 0) {
      return result >>> 0;
    }
    
    shift += 7;
  }
  
  throw new Error('Incomplete varint');
}

// Benchmark helper with latency tracking
function benchmark(name, fn, iterations = 10000) {
  // Warmup
  for (let i = 0; i < Math.min(1000, iterations / 10); i++) {
    fn();
  }
  
  // Collect latencies
  const latencies = [];
  
  for (let i = 0; i < iterations; i++) {
    const start = process.hrtime.bigint();
    fn();
    const end = process.hrtime.bigint();
    latencies.push(Number(end - start) / 1000); // microseconds
  }
  
  // Calculate statistics
  latencies.sort((a, b) => a - b);
  const totalUs = latencies.reduce((a, b) => a + b, 0);
  const durationMs = totalUs / 1000;
  const opsPerSec = (iterations / durationMs) * 1000;
  
  return {
    name,
    iterations,
    throughput: Math.round(opsPerSec),
    latency: {
      min: latencies[0].toFixed(2),
      max: latencies[latencies.length - 1].toFixed(2),
      p50: latencies[Math.floor(latencies.length * 0.5)].toFixed(2),
      p95: latencies[Math.floor(latencies.length * 0.95)].toFixed(2),
      p99: latencies[Math.floor(latencies.length * 0.99)].toFixed(2),
    }
  };
}

// Memory measurement helper
function measureMemory(fn) {
  if (global.gc) {
    global.gc();
  }
  
  const before = process.memoryUsage();
  fn();
  
  if (global.gc) {
    global.gc();
  }
  
  const after = process.memoryUsage();
  
  return {
    heap: ((after.heapUsed - before.heapUsed) / 1024 / 1024).toFixed(2),
    rss: (after.rss / 1024 / 1024).toFixed(2)
  };
}

console.log('='.repeat(80));
console.log('COMPREHENSIVE BENCHMARK SUITE');
console.log('='.repeat(80));
console.log();

// Test environment
console.log('Test Environment:');
console.log(`  Node.js: ${process.version}`);
console.log(`  Platform: ${process.platform} ${process.arch}`);
console.log(`  CPU Cores: ${require('os').cpus().length}`);
console.log(`  Memory: ${(require('os').totalmem() / 1024 / 1024 / 1024).toFixed(1)} GB`);
console.log();

const results = {};

// ============================================================================
// 1. Basic Operations
// ============================================================================

console.log('1. BASIC OPERATIONS');
console.log('-'.repeat(80));

// Varint encode single
const rustVarintEncode = benchmark('Rust - Varint encode single', () => {
  encodeVarint(12345);
}, 100000);

const jsVarintEncode = benchmark('JS - Varint encode single', () => {
  encodeVarintJS(12345);
}, 100000);

console.log(`  Varint Encode:`);
console.log(`    Rust: ${rustVarintEncode.throughput.toLocaleString()} ops/sec (P50: ${rustVarintEncode.latency.p50}µs, P99: ${rustVarintEncode.latency.p99}µs)`);
console.log(`    JS:   ${jsVarintEncode.throughput.toLocaleString()} ops/sec (P50: ${jsVarintEncode.latency.p50}µs, P99: ${jsVarintEncode.latency.p99}µs)`);
console.log(`    Speedup: ${(rustVarintEncode.throughput / jsVarintEncode.throughput).toFixed(2)}x`);
console.log();

results.varint_encode = {
  rust: rustVarintEncode,
  javascript: jsVarintEncode,
  speedup: (rustVarintEncode.throughput / jsVarintEncode.throughput).toFixed(2)
};

// Varint decode single
const encoded = encodeVarint(12345);
const rustVarintDecode = benchmark('Rust - Varint decode single', () => {
  decodeVarint(encoded);
}, 100000);

const jsVarintDecode = benchmark('JS - Varint decode single', () => {
  decodeVarintJS(encoded);
}, 100000);

console.log(`  Varint Decode:`);
console.log(`    Rust: ${rustVarintDecode.throughput.toLocaleString()} ops/sec (P50: ${rustVarintDecode.latency.p50}µs, P99: ${rustVarintDecode.latency.p99}µs)`);
console.log(`    JS:   ${jsVarintDecode.throughput.toLocaleString()} ops/sec (P50: ${jsVarintDecode.latency.p50}µs, P99: ${jsVarintDecode.latency.p99}µs)`);
console.log(`    Speedup: ${(rustVarintDecode.throughput / jsVarintDecode.throughput).toFixed(2)}x`);
console.log();

results.varint_decode = {
  rust: rustVarintDecode,
  javascript: jsVarintDecode,
  speedup: (rustVarintDecode.throughput / jsVarintDecode.throughput).toFixed(2)
};

// ============================================================================
// 2. Message Operations
// ============================================================================

console.log('2. MESSAGE OPERATIONS');
console.log('-'.repeat(80));

// Simple message (3 fields)
const rustSimpleMsg = benchmark('Rust - Simple message encode', () => {
  const writer = new RustWriter();
  writer.uint32((1 << 3) | 0); writer.uint32(100);  // id
  writer.uint32((2 << 3) | 2); writer.string('test'); // name
  writer.uint32((3 << 3) | 0); writer.bool(true);   // active
  writer.finish();
}, 50000);

const jsSimpleMsg = benchmark('JS - Simple message encode', () => {
  const writer = protobufjs.Writer.create();
  writer.uint32((1 << 3) | 0); writer.uint32(100);
  writer.uint32((2 << 3) | 2); writer.string('test');
  writer.uint32((3 << 3) | 0); writer.bool(true);
  writer.finish();
}, 50000);

console.log(`  Simple Message (3 fields):`);
console.log(`    Rust: ${rustSimpleMsg.throughput.toLocaleString()} ops/sec (P50: ${rustSimpleMsg.latency.p50}µs, P99: ${rustSimpleMsg.latency.p99}µs)`);
console.log(`    JS:   ${jsSimpleMsg.throughput.toLocaleString()} ops/sec (P50: ${jsSimpleMsg.latency.p50}µs, P99: ${jsSimpleMsg.latency.p99}µs)`);
console.log(`    Speedup: ${(rustSimpleMsg.throughput / jsSimpleMsg.throughput).toFixed(2)}x`);
console.log();

results.simple_message = {
  rust: rustSimpleMsg,
  javascript: jsSimpleMsg,
  speedup: (rustSimpleMsg.throughput / jsSimpleMsg.throughput).toFixed(2)
};

// Complex message (20 fields)
const rustComplexMsg = benchmark('Rust - Complex message encode', () => {
  const writer = new RustWriter();
  for (let i = 1; i <= 20; i++) {
    writer.uint32((i << 3) | 0);
    writer.uint32(i * 100);
  }
  writer.finish();
}, 20000);

const jsComplexMsg = benchmark('JS - Complex message encode', () => {
  const writer = protobufjs.Writer.create();
  for (let i = 1; i <= 20; i++) {
    writer.uint32((i << 3) | 0);
    writer.uint32(i * 100);
  }
  writer.finish();
}, 20000);

console.log(`  Complex Message (20 fields):`);
console.log(`    Rust: ${rustComplexMsg.throughput.toLocaleString()} ops/sec (P50: ${rustComplexMsg.latency.p50}µs, P99: ${rustComplexMsg.latency.p99}µs)`);
console.log(`    JS:   ${jsComplexMsg.throughput.toLocaleString()} ops/sec (P50: ${jsComplexMsg.latency.p50}µs, P99: ${jsComplexMsg.latency.p99}µs)`);
console.log(`    Speedup: ${(rustComplexMsg.throughput / jsComplexMsg.throughput).toFixed(2)}x`);
console.log();

results.complex_message = {
  rust: rustComplexMsg,
  javascript: jsComplexMsg,
  speedup: (rustComplexMsg.throughput / jsComplexMsg.throughput).toFixed(2)
};

// ============================================================================
// 3. Real-world Scenarios
// ============================================================================

console.log('3. REAL-WORLD SCENARIOS');
console.log('-'.repeat(80));

// gRPC Request/Response simulation
const rustGrpc = benchmark('Rust - gRPC request/response', () => {
  // Encode request
  const reqWriter = new RustWriter();
  reqWriter.uint32((1 << 3) | 2); reqWriter.string('getUserById');
  reqWriter.uint32((2 << 3) | 0); reqWriter.uint32(12345);
  const reqBuf = reqWriter.finish();
  
  // Decode request
  const reqReader = new RustReader(reqBuf);
  reqReader.uint32(); reqReader.string();
  reqReader.uint32(); reqReader.uint32();
  
  // Encode response
  const resWriter = new RustWriter();
  resWriter.uint32((1 << 3) | 0); resWriter.uint32(12345);
  resWriter.uint32((2 << 3) | 2); resWriter.string('John Doe');
  resWriter.uint32((3 << 3) | 2); resWriter.string('john@example.com');
  resWriter.finish();
}, 10000);

const jsGrpc = benchmark('JS - gRPC request/response', () => {
  // Encode request
  const reqWriter = protobufjs.Writer.create();
  reqWriter.uint32((1 << 3) | 2); reqWriter.string('getUserById');
  reqWriter.uint32((2 << 3) | 0); reqWriter.uint32(12345);
  const reqBuf = reqWriter.finish();
  
  // Decode request
  const reqReader = protobufjs.Reader.create(reqBuf);
  reqReader.uint32(); reqReader.string();
  reqReader.uint32(); reqReader.uint32();
  
  // Encode response
  const resWriter = protobufjs.Writer.create();
  resWriter.uint32((1 << 3) | 0); resWriter.uint32(12345);
  resWriter.uint32((2 << 3) | 2); resWriter.string('John Doe');
  resWriter.uint32((3 << 3) | 2); resWriter.string('john@example.com');
  resWriter.finish();
}, 10000);

console.log(`  gRPC Request/Response:`);
console.log(`    Rust: ${rustGrpc.throughput.toLocaleString()} ops/sec (P50: ${rustGrpc.latency.p50}µs, P99: ${rustGrpc.latency.p99}µs)`);
console.log(`    JS:   ${jsGrpc.throughput.toLocaleString()} ops/sec (P50: ${jsGrpc.latency.p50}µs, P99: ${jsGrpc.latency.p99}µs)`);
console.log(`    Speedup: ${(rustGrpc.throughput / jsGrpc.throughput).toFixed(2)}x`);
console.log();

results.grpc_request_response = {
  rust: rustGrpc,
  javascript: jsGrpc,
  speedup: (rustGrpc.throughput / jsGrpc.throughput).toFixed(2)
};

// Batch processing
const batchData = Array.from({ length: 100 }, (_, i) => i);
const rustBatch = benchmark('Rust - Batch processing (100 msgs)', () => {
  for (const value of batchData) {
    const writer = new RustWriter();
    writer.uint32((1 << 3) | 0); writer.uint32(value);
    writer.finish();
  }
}, 1000);

const jsBatch = benchmark('JS - Batch processing (100 msgs)', () => {
  for (const value of batchData) {
    const writer = protobufjs.Writer.create();
    writer.uint32((1 << 3) | 0); writer.uint32(value);
    writer.finish();
  }
}, 1000);

console.log(`  Batch Processing (100 messages):`);
console.log(`    Rust: ${rustBatch.throughput.toLocaleString()} ops/sec (P50: ${rustBatch.latency.p50}µs, P99: ${rustBatch.latency.p99}µs)`);
console.log(`    JS:   ${jsBatch.throughput.toLocaleString()} ops/sec (P50: ${jsBatch.latency.p50}µs, P99: ${jsBatch.latency.p99}µs)`);
console.log(`    Speedup: ${(rustBatch.throughput / jsBatch.throughput).toFixed(2)}x`);
console.log();

results.batch_processing = {
  rust: rustBatch,
  javascript: jsBatch,
  speedup: (rustBatch.throughput / jsBatch.throughput).toFixed(2)
};

// ============================================================================
// 4. Memory Analysis
// ============================================================================

console.log('4. MEMORY ANALYSIS');
console.log('-'.repeat(80));

const rustMemory = measureMemory(() => {
  for (let i = 0; i < 10000; i++) {
    const writer = new RustWriter();
    writer.uint32(i);
    writer.string('test message ' + i);
    writer.finish();
  }
});

const jsMemory = measureMemory(() => {
  for (let i = 0; i < 10000; i++) {
    const writer = protobufjs.Writer.create();
    writer.uint32(i);
    writer.string('test message ' + i);
    writer.finish();
  }
});

console.log(`  10,000 message encode operations:`);
console.log(`    Rust - Heap: ${rustMemory.heap} MB, RSS: ${rustMemory.rss} MB`);
console.log(`    JS   - Heap: ${jsMemory.heap} MB, RSS: ${jsMemory.rss} MB`);
const heapReduction = ((1 - parseFloat(rustMemory.heap) / parseFloat(jsMemory.heap)) * 100).toFixed(1);
console.log(`    Heap Reduction: ${heapReduction}%`);
console.log();

results.memory = {
  rust: rustMemory,
  javascript: jsMemory,
  reduction: heapReduction
};

// ============================================================================
// Summary
// ============================================================================

console.log('='.repeat(80));
console.log('SUMMARY');
console.log('='.repeat(80));
console.log();

console.log('Operation                  | Rust (ops/s) | JS (ops/s)  | Speedup');
console.log('-'.repeat(80));
console.log(`Varint Encode              | ${rustVarintEncode.throughput.toLocaleString().padEnd(12)} | ${jsVarintEncode.throughput.toLocaleString().padEnd(11)} | ${results.varint_encode.speedup}x`);
console.log(`Varint Decode              | ${rustVarintDecode.throughput.toLocaleString().padEnd(12)} | ${jsVarintDecode.throughput.toLocaleString().padEnd(11)} | ${results.varint_decode.speedup}x`);
console.log(`Simple Message             | ${rustSimpleMsg.throughput.toLocaleString().padEnd(12)} | ${jsSimpleMsg.throughput.toLocaleString().padEnd(11)} | ${results.simple_message.speedup}x`);
console.log(`Complex Message            | ${rustComplexMsg.throughput.toLocaleString().padEnd(12)} | ${jsComplexMsg.throughput.toLocaleString().padEnd(11)} | ${results.complex_message.speedup}x`);
console.log(`gRPC Request/Response      | ${rustGrpc.throughput.toLocaleString().padEnd(12)} | ${jsGrpc.throughput.toLocaleString().padEnd(11)} | ${results.grpc_request_response.speedup}x`);
console.log(`Batch Processing           | ${rustBatch.throughput.toLocaleString().padEnd(12)} | ${jsBatch.throughput.toLocaleString().padEnd(11)} | ${results.batch_processing.speedup}x`);
console.log();

// Calculate overall average speedup
const speedups = [
  parseFloat(results.varint_encode.speedup),
  parseFloat(results.varint_decode.speedup),
  parseFloat(results.simple_message.speedup),
  parseFloat(results.complex_message.speedup),
  parseFloat(results.grpc_request_response.speedup),
  parseFloat(results.batch_processing.speedup)
];
const avgSpeedup = (speedups.reduce((a, b) => a + b, 0) / speedups.length).toFixed(2);

console.log(`Average Speedup: ${avgSpeedup}x faster`);
console.log(`Memory Reduction: ${results.memory.reduction}%`);
console.log();

console.log('✅ All benchmarks completed successfully!');
console.log();

// Export results for use in report generation
module.exports = results;
