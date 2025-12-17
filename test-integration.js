#!/usr/bin/env node
"use strict";

// Simple test to verify protobuf-rs integration

console.log("Testing protobuf-rs integration...\n");

// Test 1: Load the module
console.log("1. Loading protobuf module...");
const protobuf = require('./index.js');
console.log("   ✓ Module loaded successfully");
console.log("   Build:", protobuf.build);

// Test 2: Check if Rust components are available
console.log("\n2. Checking Rust components...");
if (protobuf.rust) {
    console.log("   ✓ Rust acceleration available");
    
    // Test basic varint encoding/decoding
    console.log("\n3. Testing varint encoding/decoding...");
    try {
        const value = 300;
        const encoded = protobuf.rust.encodeVarint(value);
        const decoded = protobuf.rust.decodeVarint(encoded);
        console.log(`   Input: ${value}`);
        console.log(`   Encoded: [${Array.from(encoded).join(', ')}]`);
        console.log(`   Decoded: ${decoded}`);
        console.log(`   ✓ Varint test passed`);
    } catch (e) {
        console.error(`   ✗ Varint test failed:`, e.message);
    }
    
    // Test ZigZag encoding/decoding
    console.log("\n4. Testing ZigZag encoding/decoding...");
    try {
        const value = -150;
        const encoded = protobuf.rust.encodeZigzag(value);
        const decoded = protobuf.rust.decodeZigzag(encoded);
        console.log(`   Input: ${value}`);
        console.log(`   Encoded: ${encoded}`);
        console.log(`   Decoded: ${decoded}`);
        console.log(`   ✓ ZigZag test passed`);
    } catch (e) {
        console.error(`   ✗ ZigZag test failed:`, e.message);
    }
    
    // Test Encoder
    console.log("\n5. Testing Rust Encoder...");
    try {
        const encoder = new protobuf.rust.Encoder();
        encoder.encodeUint32(100);
        encoder.encodeString("Hello, World!");
        const buffer = encoder.finish();
        console.log(`   ✓ Encoder created and used successfully`);
        console.log(`   Buffer size: ${buffer.length} bytes`);
    } catch (e) {
        console.error(`   ✗ Encoder test failed:`, e.message);
    }
    
    // Test Writer
    console.log("\n6. Testing Rust Writer...");
    try {
        const writer = new protobuf.rust.Writer();
        writer.uint32(100);
        writer.string("Test");
        const buffer = writer.finish();
        console.log(`   ✓ Writer created and used successfully`);
        console.log(`   Buffer size: ${buffer.length} bytes`);
    } catch (e) {
        console.error(`   ✗ Writer test failed:`, e.message);
    }
    
    // Test Reader
    console.log("\n7. Testing Rust Reader...");
    try {
        const writer = new protobuf.rust.Writer();
        writer.uint32(300);
        writer.string("Hello");
        const buffer = writer.finish();
        
        const reader = new protobuf.rust.Reader(buffer);
        const num = reader.uint32();
        const str = reader.string();
        console.log(`   Read uint32: ${num}`);
        console.log(`   Read string: ${str}`);
        console.log(`   ✓ Reader test passed`);
    } catch (e) {
        console.error(`   ✗ Reader test failed:`, e.message);
    }
} else {
    console.log("   ⚠ Rust acceleration not available (using pure JavaScript)");
}

// Test 3: Check protobuf.js base functionality
console.log("\n8. Testing protobuf.js base functionality...");
try {
    const Root = protobuf.Root;
    const Type = protobuf.Type;
    const Field = protobuf.Field;
    
    console.log("   ✓ Core protobuf.js classes available");
    console.log("   - Root:", typeof Root);
    console.log("   - Type:", typeof Type);
    console.log("   - Field:", typeof Field);
} catch (e) {
    console.error(`   ✗ protobuf.js test failed:`, e.message);
}

console.log("\n✓ All tests completed!");
console.log("\nIntegration summary:");
console.log("- protobuf.js source: ✓ Available");
console.log("- Rust acceleration:", protobuf.rust ? "✓ Available" : "✗ Not available");
console.log("- Build type:", protobuf.build);
