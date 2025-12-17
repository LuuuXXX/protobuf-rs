#!/usr/bin/env node
/**
 * Final comprehensive benchmark comparing:
 * 1. Old Rust implementation (direct FFI calls)
 * 2. New Rust implementation (batched FFI calls)
 * 3. Pure JavaScript implementation
 */

const Benchmark = require('benchmark');
const nativeBinding = require('../index.node');
const pb = require('..');

// Old implementation (direct FFI calls per method)
class OldRustWriter {
    constructor() {
        this._native = new nativeBinding.Writer();
    }
    static create() {
        return new OldRustWriter();
    }
    uint32(value) {
        this._native.uint32(value);
        return this;
    }
    string(value) {
        this._native.string(value);
        return this;
    }
    fork() {
        this._native.fork();
        return this;
    }
    ldelim() {
        this._native.ldelim();
        return this;
    }
    finish() {
        return this._native.finish();
    }
}

// Pure JavaScript implementation
const JsWriter = require('../src/writer');

console.log('\n' + '='.repeat(70));
console.log('  COMPREHENSIVE PERFORMANCE BENCHMARK');
console.log('='.repeat(70));
console.log('\nComparing three implementations:');
console.log('  1. OLD Rust (direct FFI calls per method)');
console.log('  2. NEW Rust (batched FFI - one call per message)');
console.log('  3. JavaScript (V8 JIT optimized)\n');
console.log('='.repeat(70) + '\n');

const tests = [
    {
        name: 'Small message (2 fields)',
        encode: (Writer) => {
            const w = Writer.create();
            w.uint32(150);
            w.string('hello');
            return w.finish();
        }
    },
    {
        name: 'Medium message (2 fields, longer string)',
        encode: (Writer) => {
            const w = Writer.create();
            w.uint32(99999);
            w.string('hello world'.repeat(10));
            return w.finish();
        }
    },
    {
        name: 'Complex message (fork/ldelim)',
        encode: (Writer) => {
            const w = Writer.create();
            w.uint32(1);
            w.fork();
            w.uint32(10);
            w.string('nested');
            w.ldelim();
            w.uint32(2);
            return w.finish();
        }
    }
];

const results = {};

tests.forEach(test => {
    console.log(`\n📊 Test: ${test.name}\n`);
    
    const suite = new Benchmark.Suite();
    
    suite
        .add('OLD Rust', function() {
            test.encode(OldRustWriter);
        })
        .add('NEW Rust', function() {
            test.encode(pb.Writer);
        })
        .add('JavaScript', function() {
            test.encode(JsWriter);
        })
        .on('cycle', function(event) {
            const bench = event.target;
            const opsPerSec = Math.round(bench.hz);
            console.log(`  ${bench.name.padEnd(12)} ${String(bench)}`);
            
            if (!results[test.name]) results[test.name] = {};
            results[test.name][bench.name] = opsPerSec;
        })
        .run({ async: false });
});

console.log('\n' + '='.repeat(70));
console.log('  SUMMARY');
console.log('='.repeat(70) + '\n');

tests.forEach(test => {
    const r = results[test.name];
    if (!r) return;
    
    const oldRust = r['OLD Rust'] || 0;
    const newRust = r['NEW Rust'] || 0;
    const js = r['JavaScript'] || 0;
    
    const improvement = ((newRust - oldRust) / oldRust * 100).toFixed(1);
    const vsJS = (newRust / js * 100).toFixed(1);
    
    console.log(`${test.name}:`);
    console.log(`  OLD Rust:    ${oldRust.toLocaleString().padStart(10)} ops/sec`);
    console.log(`  NEW Rust:    ${newRust.toLocaleString().padStart(10)} ops/sec  (+${improvement}%)`);
    console.log(`  JavaScript:  ${js.toLocaleString().padStart(10)} ops/sec`);
    console.log(`  NEW vs JS:   ${vsJS}% of JavaScript performance`);
    console.log();
});

console.log('='.repeat(70));
console.log('\n✅ Benchmark complete!\n');

// Calculate actual improvement range
const improvements = [];
tests.forEach(test => {
    const r = results[test.name];
    if (r && r['OLD Rust'] && r['NEW Rust']) {
        const improvement = ((r['NEW Rust'] - r['OLD Rust']) / r['OLD Rust'] * 100);
        improvements.push(improvement);
    }
});

const minImprovement = Math.min(...improvements).toFixed(0);
const maxImprovement = Math.max(...improvements).toFixed(0);

console.log('Key takeaways:');
console.log(`  • NEW Rust is ${minImprovement}-${maxImprovement}% faster than OLD Rust`);
console.log('  • Reduced FFI calls from N (per field) to 1 (per message)');
console.log('  • JavaScript remains fastest due to V8 JIT optimization');
console.log('  • Further improvements would require binary encoding or V8 Fast API\n');
