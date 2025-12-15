/**
 * Tests for protobufjs-compat.js
 * 
 * Verifies that the drop-in replacement works correctly
 */

const tape = require('tape');

console.log('\n🧪 Testing protobufjs-compat drop-in replacement...\n');

tape.test('protobufjs-compat loads successfully', (t) => {
  const protobuf = require('../protobufjs-compat');
  t.ok(protobuf, 'Module should load');
  t.end();
});

tape.test('All core APIs are exported', (t) => {
  const protobuf = require('../protobufjs-compat');
  
  // Core classes
  t.ok(protobuf.Root, 'Root should be exported');
  t.ok(protobuf.Type, 'Type should be exported');
  t.ok(protobuf.Field, 'Field should be exported');
  t.ok(protobuf.Enum, 'Enum should be exported');
  t.ok(protobuf.Service, 'Service should be exported');
  t.ok(protobuf.Method, 'Method should be exported');
  
  // Reader/Writer
  t.ok(protobuf.Reader, 'Reader should be exported');
  t.ok(protobuf.Writer, 'Writer should be exported');
  
  // Entry points
  t.ok(protobuf.parse, 'parse should be exported');
  t.ok(protobuf.load, 'load should be exported');
  t.ok(protobuf.loadSync, 'loadSync should be exported');
  
  // Utilities
  t.ok(protobuf.util, 'util should be exported');
  t.ok(protobuf.configure, 'configure should be exported');
  
  t.end();
});

tape.test('Reader/Writer work correctly', (t) => {
  const protobuf = require('../protobufjs-compat');
  
  const writer = protobuf.Writer.create();
  writer.uint32(12345);
  writer.string('Hello, protobuf-rs!');
  const buffer = writer.finish();
  
  const reader = protobuf.Reader.create(buffer);
  t.equal(reader.uint32(), 12345, 'Should read uint32');
  t.equal(reader.string(), 'Hello, protobuf-rs!', 'Should read string');
  
  t.end();
});

tape.test('Implementation info functions work', (t) => {
  const protobuf = require('../protobufjs-compat');
  
  t.equal(typeof protobuf.isNativeAccelerated, 'function', 'isNativeAccelerated should be a function');
  t.equal(typeof protobuf.getImplementationInfo, 'function', 'getImplementationInfo should be a function');
  
  const isNative = protobuf.isNativeAccelerated();
  t.equal(typeof isNative, 'boolean', 'isNativeAccelerated should return boolean');
  
  const info = protobuf.getImplementationInfo();
  t.ok(info, 'getImplementationInfo should return object');
  t.equal(typeof info.native, 'boolean', 'info.native should be boolean');
  t.ok(info.type, 'info.type should exist');
  t.ok(info.version, 'info.version should exist');
  
  console.log(`  Implementation: ${info.type} (native: ${info.native})`);
  console.log(`  Version: ${info.version}`);
  
  t.end();
});

tape.test('Can create and use Root with Type', (t) => {
  const protobuf = require('../protobufjs-compat');
  
  const root = new protobuf.Root();
  const testType = new protobuf.Type('Test');
  
  testType.add(new protobuf.Field('id', 1, 'uint32'));
  testType.add(new protobuf.Field('name', 2, 'string'));
  
  root.add(testType);
  
  // Create a message
  const message = testType.create({ id: 42, name: 'test' });
  
  // Encode it
  const buffer = testType.encode(message).finish();
  t.ok(buffer.length > 0, 'Encoded buffer should have content');
  
  // Decode it
  const decoded = testType.decode(buffer);
  t.equal(decoded.id, 42, 'Decoded id should match');
  t.equal(decoded.name, 'test', 'Decoded name should match');
  
  t.end();
});

tape.test('Compatibility with protobuf.js load/loadSync', (t) => {
  const protobuf = require('../protobufjs-compat');
  const fs = require('fs');
  const path = require('path');
  const os = require('os');
  
  // Create a temporary .proto file
  const protoContent = `
syntax = "proto3";

message TestMessage {
  uint32 id = 1;
  string text = 2;
}
`;
  
  const protoPath = path.join(os.tmpdir(), 'test.proto');
  fs.writeFileSync(protoPath, protoContent);
  
  // Test loadSync
  try {
    const root = protobuf.loadSync(protoPath);
    t.ok(root, 'loadSync should return root');
    
    const TestMessage = root.lookupType('TestMessage');
    t.ok(TestMessage, 'Should find TestMessage type');
    
    // Test encoding/decoding
    const message = TestMessage.create({ id: 123, text: 'hello' });
    const buffer = TestMessage.encode(message).finish();
    const decoded = TestMessage.decode(buffer);
    
    t.equal(decoded.id, 123, 'Decoded id should match');
    t.equal(decoded.text, 'hello', 'Decoded text should match');
    
    // Clean up
    fs.unlinkSync(protoPath);
    
    t.end();
  } catch (err) {
    t.fail('loadSync should not throw: ' + err.message);
    t.end();
  }
});

console.log('\n✅ protobufjs-compat tests complete!\n');
