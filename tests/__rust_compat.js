var tape = require("tape");
var protobuf = require("..");

tape.test("Rust implementation", function(test) {
    test.ok(protobuf.__usingRust, "should be using Rust implementation");
    
    test.test(test.name + " - basic Writer/Reader operations", function(test) {
        var writer = protobuf.Writer.create();
        
        // Test all data types
        writer.uint32(42);
        writer.int32(-100);
        writer.sint32(-200);
        writer.bool(true);
        writer.string("Hello, Rust!");
        writer.bytes(Buffer.from([1, 2, 3, 4, 5]));
        writer.float(3.14);
        writer.double(2.718);
        
        var buffer = writer.finish();
        test.ok(Buffer.isBuffer(buffer), "finish() should return a Buffer");
        
        var reader = protobuf.Reader.create(buffer);
        test.equal(reader.uint32(), 42, "should read uint32");
        test.equal(reader.int32(), -100, "should read int32");
        test.equal(reader.sint32(), -200, "should read sint32");
        test.equal(reader.bool(), true, "should read bool");
        test.equal(reader.string(), "Hello, Rust!", "should read string");
        
        var bytes = reader.bytes();
        test.ok(Buffer.isBuffer(bytes), "should read bytes as Buffer");
        test.deepEqual(Array.from(bytes), [1, 2, 3, 4, 5], "bytes content should match");
        
        var floatVal = reader.float();
        test.ok(Math.abs(floatVal - 3.14) < 0.01, "should read float");
        
        var doubleVal = reader.double();
        test.ok(Math.abs(doubleVal - 2.718) < 0.001, "should read double");
        
        test.end();
    });
    
    test.test(test.name + " - fork/ldelim operations", function(test) {
        var writer = protobuf.Writer.create();
        
        writer.uint32(1);
        writer.fork();
        writer.uint32(10);
        writer.string("nested");
        writer.ldelim();
        writer.uint32(2);
        
        var buffer = writer.finish();
        
        var reader = protobuf.Reader.create(buffer);
        test.equal(reader.uint32(), 1, "should read first field");
        
        var len = reader.uint32();
        test.ok(len > 0, "should have length prefix");
        
        test.equal(reader.uint32(), 10, "should read nested uint32");
        test.equal(reader.string(), "nested", "should read nested string");
        
        test.equal(reader.uint32(), 2, "should read last field");
        
        test.end();
    });
    
    test.test(test.name + " - 64-bit integers with Long", function(test) {
        if (!protobuf.util.Long) {
            test.skip("Long.js not available");
            test.end();
            return;
        }
        
        var Long = protobuf.util.Long;
        var writer = protobuf.Writer.create();
        
        var largeValue = Long.fromNumber(549755813887, false);
        writer.uint64(largeValue);
        writer.int64(largeValue);
        writer.sint64(largeValue);
        
        var buffer = writer.finish();
        
        var reader = protobuf.Reader.create(buffer);
        var val1 = reader.uint64();
        var val2 = reader.int64();
        var val3 = reader.sint64();
        
        test.ok(val1.equals(largeValue), "should read uint64 as Long");
        test.ok(val2.equals(largeValue), "should read int64 as Long");
        test.ok(val3.equals(largeValue), "should read sint64 as Long");
        
        test.end();
    });
    
    test.test(test.name + " - array input for bytes", function(test) {
        var writer = protobuf.Writer.create();
        writer.bytes([1, 2, 3, 4, 5]);
        
        var buffer = writer.finish();
        var reader = protobuf.Reader.create(buffer);
        var bytes = reader.bytes();
        
        test.deepEqual(Array.from(bytes), [1, 2, 3, 4, 5], "should handle array input for bytes");
        
        test.end();
    });
    
    test.test(test.name + " - base64 string input for bytes", function(test) {
        var writer = protobuf.Writer.create();
        var base64Data = "SGVsbG8="; // "Hello" in base64
        writer.bytes(base64Data);
        
        var buffer = writer.finish();
        var reader = protobuf.Reader.create(buffer);
        var bytes = reader.bytes();
        
        test.equal(bytes.toString(), "Hello", "should handle base64 string input for bytes");
        
        test.end();
    });
    
    test.end();
});
