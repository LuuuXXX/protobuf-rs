const Benchmark = require('benchmark');
const protobuf = require('..');

console.log(`\n🦀 Using: ${protobuf.__usingRust ? 'Rust' : 'JavaScript'} implementation\n`);

const suite = new Benchmark.Suite();

// Test data
const testData = {
    small: { uint32: 150, string: "hello" },
    medium: { uint32: 99999, string: "hello world".repeat(10) },
    large: { bytes: Buffer.alloc(10000).fill(42) }
};

// Pre-encode test data for decoding benchmarks
const smallBuffer = protobuf.Writer.create().uint32(testData.small.uint32).string(testData.small.string).finish();
const mediumBuffer = protobuf.Writer.create().uint32(testData.medium.uint32).string(testData.medium.string).finish();
const largeBuffer = protobuf.Writer.create().bytes(testData.large.bytes).finish();

suite
    .add('Writer#encode (small)', function() {
        const writer = protobuf.Writer.create();
        writer.uint32(testData.small.uint32);
        writer.string(testData.small.string);
        writer.finish();
    })
    .add('Writer#encode (medium)', function() {
        const writer = protobuf.Writer.create();
        writer.uint32(testData.medium.uint32);
        writer.string(testData.medium.string);
        writer.finish();
    })
    .add('Writer#encode (large)', function() {
        const writer = protobuf.Writer.create();
        writer.bytes(testData.large.bytes);
        writer.finish();
    })
    .add('Reader#decode (small)', function() {
        const reader = protobuf.Reader.create(smallBuffer);
        reader.uint32();
        reader.string();
    })
    .add('Reader#decode (medium)', function() {
        const reader = protobuf.Reader.create(mediumBuffer);
        reader.uint32();
        reader.string();
    })
    .add('Reader#decode (large)', function() {
        const reader = protobuf.Reader.create(largeBuffer);
        reader.bytes();
    })
    .add('Combined (small encode+decode)', function() {
        const writer = protobuf.Writer.create();
        writer.uint32(testData.small.uint32);
        writer.string(testData.small.string);
        const buffer = writer.finish();
        
        const reader = protobuf.Reader.create(buffer);
        reader.uint32();
        reader.string();
    })
    .add('Fork/Ldelim operations', function() {
        const writer = protobuf.Writer.create();
        writer.uint32(1);
        writer.fork();
        writer.uint32(10);
        writer.string("nested");
        writer.ldelim();
        writer.uint32(2);
        writer.finish();
    })
    .on('cycle', function(event) {
        console.log(String(event.target));
    })
    .on('complete', function() {
        console.log('\n✅ Benchmark complete!');
        console.log('\nTo compare with JavaScript implementation, run:');
        console.log('  PROTOBUF_NO_RUST=1 node bench/rust-comparison.js');
    })
    .run({ async: false });
