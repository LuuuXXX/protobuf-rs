// Compare old vs new implementation
const Benchmark = require('benchmark');

const nativeBinding = require('../index.node');
const pb = require('..');

// Old implementation (direct FFI calls)
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

console.log('\n📊 Performance Comparison:\n');

const suite = new Benchmark.Suite();

suite
    .add('OLD (direct FFI) - small', function() {
        const writer = OldRustWriter.create();
        writer.uint32(150);
        writer.string('hello');
        writer.finish();
    })
    .add('NEW (batched FFI) - small', function() {
        const writer = pb.Writer.create();
        writer.uint32(150);
        writer.string('hello');
        writer.finish();
    })
    .add('OLD (direct FFI) - fork/ldelim', function() {
        const writer = OldRustWriter.create();
        writer.uint32(1);
        writer.fork();
        writer.uint32(10);
        writer.string('nested');
        writer.ldelim();
        writer.uint32(2);
        writer.finish();
    })
    .add('NEW (batched FFI) - fork/ldelim', function() {
        const writer = pb.Writer.create();
        writer.uint32(1);
        writer.fork();
        writer.uint32(10);
        writer.string('nested');
        writer.ldelim();
        writer.uint32(2);
        writer.finish();
    })
    .on('cycle', function(event) {
        console.log(String(event.target));
    })
    .on('complete', function() {
        console.log('\n✅ Comparison complete!\n');
        console.log('Note: NEW implementation should be faster due to reduced FFI calls');
    })
    .run({ async: false });
