#!/usr/bin/env node
"use strict";

/**
 * Example: Using protobuf-rs with Rust acceleration
 * 
 * This example demonstrates how to use the library with both
 * the standard protobuf.js API and direct Rust acceleration.
 */

const protobuf = require('./index.js');

console.log("=".repeat(70));
console.log("protobuf-rs Example - Hybrid JavaScript + Rust Implementation");
console.log("=".repeat(70));
console.log();

// Show build type
console.log("Build Type:", protobuf.build);
console.log("Rust Acceleration:", protobuf.rust ? "✓ Available" : "✗ Not available");
console.log();

// ============================================================================
// Example 1: Using protobuf.js API (Standard)
// ============================================================================
console.log("─".repeat(70));
console.log("Example 1: Standard protobuf.js API");
console.log("─".repeat(70));

// Define a simple message type
const Root = protobuf.Root;
const Type = protobuf.Type;
const Field = protobuf.Field;

const root = new Root();

const Person = new Type("Person")
    .add(new Field("id", 1, "uint32"))
    .add(new Field("name", 2, "string"))
    .add(new Field("email", 3, "string"))
    .add(new Field("age", 4, "uint32"));

root.define("example").add(Person);

// Create and encode a message
const person = Person.create({
    id: 1,
    name: "Alice",
    email: "alice@example.com",
    age: 30
});

console.log("Original message:", person);

const encoded = Person.encode(person).finish();
console.log("Encoded size:", encoded.length, "bytes");
console.log("Encoded hex:", Buffer.from(encoded).toString('hex'));

// Decode the message
const decoded = Person.decode(encoded);
console.log("Decoded message:", decoded);
console.log();

// ============================================================================
// Example 2: Direct Rust Acceleration (Performance-Critical)
// ============================================================================
if (protobuf.rust) {
    console.log("─".repeat(70));
    console.log("Example 2: Direct Rust Acceleration");
    console.log("─".repeat(70));
    
    // Using high-performance Writer
    const writer = new protobuf.rust.Writer();
    
    // Write field 1: id (varint)
    writer.tag(1, 0).uint32(1);
    
    // Write field 2: name (string)
    writer.tag(2, 2).string("Bob");
    
    // Write field 3: email (string)
    writer.tag(3, 2).string("bob@example.com");
    
    // Write field 4: age (varint)
    writer.tag(4, 0).uint32(25);
    
    const buffer = writer.finish();
    console.log("Writer buffer size:", buffer.length, "bytes");
    console.log("Writer buffer hex:", Buffer.from(buffer).toString('hex'));
    
    // Using high-performance Reader
    const reader = new protobuf.rust.Reader(buffer);
    console.log("\nReading back with Reader:");
    
    while (reader.pos() < reader.len()) {
        const tag = reader.uint32();
        const fieldNumber = tag >>> 3;
        const wireType = tag & 7;
        
        console.log(`  Field ${fieldNumber} (wire type ${wireType}):`);
        
        switch (fieldNumber) {
            case 1: // id
                console.log(`    id = ${reader.uint32()}`);
                break;
            case 2: // name
                console.log(`    name = "${reader.string()}"`);
                break;
            case 3: // email
                console.log(`    email = "${reader.string()}"`);
                break;
            case 4: // age
                console.log(`    age = ${reader.uint32()}`);
                break;
            default:
                reader.skipType(wireType);
        }
    }
    console.log();
}

// ============================================================================
// Example 3: Performance-Critical Varint Operations
// ============================================================================
if (protobuf.rust) {
    console.log("─".repeat(70));
    console.log("Example 3: Performance-Critical Varint Operations");
    console.log("─".repeat(70));
    
    // Large number encoding
    const numbers = [1, 127, 128, 300, 16384, 2097151, 268435455];
    
    console.log("Varint encoding examples:");
    for (const num of numbers) {
        const encoded = protobuf.rust.encodeVarint(num);
        const decoded = protobuf.rust.decodeVarint(encoded);
        const hex = Buffer.from(encoded).toString('hex');
        console.log(`  ${num.toString().padStart(10)} -> [${hex}] (${encoded.length} bytes) -> ${decoded}`);
    }
    
    console.log("\nZigZag encoding examples (for signed numbers):");
    const signedNumbers = [-1, 1, -2, 2, -64, 64, -150];
    for (const num of signedNumbers) {
        const encoded = protobuf.rust.encodeZigzag(num);
        const decoded = protobuf.rust.decodeZigzag(encoded);
        console.log(`  ${num.toString().padStart(5)} -> ${encoded.toString().padStart(5)} -> ${decoded}`);
    }
    console.log();
}

// ============================================================================
// Example 4: Benchmark - Comparing Performance
// ============================================================================
if (protobuf.rust) {
    console.log("─".repeat(70));
    console.log("Example 4: Quick Performance Comparison");
    console.log("─".repeat(70));
    
    const iterations = 100000;
    const testValue = 300;
    
    // Benchmark Rust varint
    console.log(`Encoding ${iterations} varints...`);
    const rustStart = Date.now();
    for (let i = 0; i < iterations; i++) {
        const buf = protobuf.rust.encodeVarint(testValue);
        protobuf.rust.decodeVarint(buf);
    }
    const rustTime = Date.now() - rustStart;
    
    console.log(`  Rust implementation: ${rustTime}ms`);
    console.log(`  Throughput: ${Math.round(iterations / rustTime * 1000)} ops/sec`);
    console.log();
}

console.log("=".repeat(70));
console.log("Examples completed successfully!");
console.log("=".repeat(70));
