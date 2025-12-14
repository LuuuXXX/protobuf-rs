/**
 * Memory Profiling Tool
 * 
 * Analyzes memory usage patterns and detects potential leaks
 * in protobuf-rs operations.
 */

const v8 = require('v8');
const fs = require('fs');
const path = require('path');

const { 
    encodeVarint, 
    decodeVarint,
    Reader,
    Writer
} = require('../index.js');

// Enable garbage collection if available
if (global.gc) {
    console.log('✓ GC available for accurate measurements');
} else {
    console.log('⚠ Run with --expose-gc for accurate measurements: node --expose-gc memory-profile.js');
}

console.log('='.repeat(80));
console.log('MEMORY PROFILING');
console.log('='.repeat(80));
console.log();

// Helper to get memory snapshot
function getMemorySnapshot() {
    const mem = process.memoryUsage();
    return {
        heapUsed: mem.heapUsed,
        heapTotal: mem.heapTotal,
        external: mem.external,
        rss: mem.rss
    };
}

// Format bytes to MB
function formatMB(bytes) {
    return (bytes / 1024 / 1024).toFixed(2);
}

// Test 1: Allocation Tracking
console.log('Test 1: Allocation Tracking');
console.log('-'.repeat(80));

if (global.gc) global.gc();
const baseline = getMemorySnapshot();

const iterations = 100000;
const allocations = [];

for (let i = 0; i < iterations; i++) {
    const encoded = encodeVarint(i * 1000);
    allocations.push(encoded);
}

if (global.gc) global.gc();
const afterAlloc = getMemorySnapshot();

console.log(`Baseline heap: ${formatMB(baseline.heapUsed)} MB`);
console.log(`After ${iterations} allocations: ${formatMB(afterAlloc.heapUsed)} MB`);
console.log(`Delta: ${formatMB(afterAlloc.heapUsed - baseline.heapUsed)} MB`);
console.log(`Avg per allocation: ${((afterAlloc.heapUsed - baseline.heapUsed) / iterations).toFixed(0)} bytes`);
console.log();

// Test 2: Leak Detection
console.log('Test 2: Leak Detection');
console.log('-'.repeat(80));

const samples = 10;
const opsPerSample = 10000;
const memSamples = [];

for (let sample = 0; sample < samples; sample++) {
    if (global.gc) global.gc();
    
    const before = process.memoryUsage();
    
    for (let i = 0; i < opsPerSample; i++) {
        const encoded = encodeVarint(i);
        decodeVarint(encoded);
        
        const writer = new Writer();
        writer.uint32(i);
        const buf = writer.finish();
        
        const reader = new Reader(buf);
        reader.uint32();
    }
    
    if (global.gc) global.gc();
    const after = process.memoryUsage();
    
    memSamples.push({
        sample: sample + 1,
        heapUsed: after.heapUsed,
        delta: after.heapUsed - before.heapUsed
    });
}

console.log('Memory samples over time:');
memSamples.forEach(s => {
    console.log(`  Sample ${s.sample}: ${formatMB(s.heapUsed)} MB (delta: ${formatMB(s.delta)} MB)`);
});

// Check for memory leak (increasing trend)
const firstHalf = memSamples.slice(0, samples / 2).reduce((sum, s) => sum + s.heapUsed, 0) / (samples / 2);
const secondHalf = memSamples.slice(samples / 2).reduce((sum, s) => sum + s.heapUsed, 0) / (samples / 2);
const trend = ((secondHalf - firstHalf) / firstHalf) * 100;

console.log();
if (Math.abs(trend) < 5) {
    console.log(`✓ No memory leak detected (trend: ${trend.toFixed(2)}%)`);
} else {
    console.log(`⚠ Possible memory leak detected (trend: ${trend.toFixed(2)}%)`);
}
console.log();

// Test 3: GC Activity Monitoring
console.log('Test 3: GC Activity Monitoring');
console.log('-'.repeat(80));

const gcStart = process.hrtime();
let gcCount = 0;

// Monitor GC activity
if (global.gc) {
    for (let i = 0; i < 10; i++) {
        // Create pressure
        const temp = [];
        for (let j = 0; j < 10000; j++) {
            temp.push(encodeVarint(j));
        }
        
        global.gc();
        gcCount++;
    }
    
    const gcEnd = process.hrtime(gcStart);
    const gcTimeMs = (gcEnd[0] * 1000 + gcEnd[1] / 1000000).toFixed(2);
    
    console.log(`GC cycles: ${gcCount}`);
    console.log(`Total GC time: ${gcTimeMs} ms`);
    console.log(`Avg GC time: ${(gcTimeMs / gcCount).toFixed(2)} ms`);
} else {
    console.log('GC monitoring requires --expose-gc flag');
}
console.log();

// Test 4: Heap Snapshot
console.log('Test 4: Heap Snapshot');
console.log('-'.repeat(80));

try {
    // Create some activity
    for (let i = 0; i < 10000; i++) {
        encodeVarint(i);
    }
    
    const snapshotPath = path.join(__dirname, 'heap-snapshot.heapsnapshot');
    const snapshot = v8.writeHeapSnapshot(snapshotPath);
    const stats = fs.statSync(snapshot);
    
    console.log(`Heap snapshot saved: ${path.basename(snapshot)}`);
    console.log(`Snapshot size: ${formatMB(stats.size)} MB`);
    console.log();
    console.log('To analyze the snapshot:');
    console.log('  1. Open Chrome DevTools');
    console.log('  2. Go to Memory tab');
    console.log('  3. Load the .heapsnapshot file');
    console.log();
} catch (err) {
    console.log(`Failed to create heap snapshot: ${err.message}`);
    console.log();
}

// Test 5: External Memory
console.log('Test 5: External Memory (Native Buffers)');
console.log('-'.repeat(80));

if (global.gc) global.gc();
const externalBefore = process.memoryUsage();

const buffers = [];
for (let i = 0; i < 1000; i++) {
    const writer = new Writer();
    for (let j = 0; j < 100; j++) {
        writer.uint32(j);
    }
    buffers.push(writer.finish());
}

if (global.gc) global.gc();
const externalAfter = process.memoryUsage();

console.log(`External memory before: ${formatMB(externalBefore.external)} MB`);
console.log(`External memory after: ${formatMB(externalAfter.external)} MB`);
console.log(`External delta: ${formatMB(externalAfter.external - externalBefore.external)} MB`);
console.log(`Native buffer overhead: ${((externalAfter.external - externalBefore.external) / buffers.length).toFixed(0)} bytes per buffer`);
console.log();

// Summary Report
console.log('='.repeat(80));
console.log('MEMORY PROFILING SUMMARY');
console.log('='.repeat(80));
console.log();
console.log('Findings:');
console.log(`  ✓ Average allocation size: ${((afterAlloc.heapUsed - baseline.heapUsed) / iterations).toFixed(0)} bytes`);
console.log(`  ✓ Memory leak trend: ${trend.toFixed(2)}% ${Math.abs(trend) < 5 ? '(healthy)' : '(needs investigation)'}`);
if (global.gc) {
    console.log(`  ✓ GC efficiency: ${(gcTimeMs / gcCount).toFixed(2)} ms per cycle`);
}
console.log(`  ✓ External memory overhead: ${((externalAfter.external - externalBefore.external) / buffers.length).toFixed(0)} bytes per buffer`);
console.log();
console.log('Recommendations:');
console.log('  - Monitor external memory for native buffer usage');
console.log('  - Run long-duration tests to confirm no memory leaks');
console.log('  - Use heap snapshots to identify large retainers');
console.log('  - Consider buffer pooling for high-frequency operations');
console.log();
console.log('✅ Memory profiling completed!');
console.log();
