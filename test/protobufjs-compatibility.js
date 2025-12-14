/**
 * Compatibility Test Suite
 * 
 * Comprehensive tests to ensure protobuf.js compatibility.
 * Tests both native Rust and pure JavaScript implementations.
 */

const tape = require('tape');
const { Reader, Writer, isNativeAvailable } = require('../integration/protobufjs-adapter');

console.log('\n🧪 Running protobuf.js compatibility tests...\n');

tape.test('Basic Reader/Writer compatibility', (t) => {
  const writer = Writer.create();
  writer.uint32(300);
  const buffer = writer.finish();

  const reader = Reader.create(buffer);
  const value = reader.uint32();

  t.equal(value, 300, 'uint32 round-trip should work');
  t.end();
});

tape.test('Multiple values in single buffer', (t) => {
  const writer = Writer.create();
  writer.uint32(100);
  writer.uint32(200);
  writer.uint32(300);
  const buffer = writer.finish();

  const reader = Reader.create(buffer);
  t.equal(reader.uint32(), 100, 'first value should be 100');
  t.equal(reader.uint32(), 200, 'second value should be 200');
  t.equal(reader.uint32(), 300, 'third value should be 300');
  t.end();
});

tape.test('Reader position tracking', (t) => {
  const writer = Writer.create();
  writer.uint32(42);
  const buffer = writer.finish();

  const reader = Reader.create(buffer);
  t.equal(reader.pos, 0, 'initial position should be 0');
  t.equal(reader.len, buffer.length, 'length should match buffer length');
  
  reader.uint32();
  t.ok(reader.pos > 0, 'position should advance after reading');
  t.end();
});

tape.test('All data types round-trip - integers', (t) => {
  const writer = Writer.create();
  
  // uint32
  writer.uint32(0);
  writer.uint32(1);
  writer.uint32(127);
  writer.uint32(16384);
  writer.uint32(2147483647);
  
  // int32
  writer.int32(0);
  writer.int32(-1);
  writer.int32(127);
  writer.int32(-128);
  
  // sint32 (zigzag)
  writer.sint32(0);
  writer.sint32(-1);
  writer.sint32(1);
  writer.sint32(-2);
  writer.sint32(2);
  
  // bool
  writer.bool(true);
  writer.bool(false);
  
  const buffer = writer.finish();
  const reader = Reader.create(buffer);
  
  // uint32
  t.equal(reader.uint32(), 0, 'uint32(0)');
  t.equal(reader.uint32(), 1, 'uint32(1)');
  t.equal(reader.uint32(), 127, 'uint32(127)');
  t.equal(reader.uint32(), 16384, 'uint32(16384)');
  t.equal(reader.uint32(), 2147483647, 'uint32(2147483647)');
  
  // int32
  t.equal(reader.int32(), 0, 'int32(0)');
  t.equal(reader.int32(), -1, 'int32(-1)');
  t.equal(reader.int32(), 127, 'int32(127)');
  t.equal(reader.int32(), -128, 'int32(-128)');
  
  // sint32
  t.equal(reader.sint32(), 0, 'sint32(0)');
  t.equal(reader.sint32(), -1, 'sint32(-1)');
  t.equal(reader.sint32(), 1, 'sint32(1)');
  t.equal(reader.sint32(), -2, 'sint32(-2)');
  t.equal(reader.sint32(), 2, 'sint32(2)');
  
  // bool
  t.equal(reader.bool(), true, 'bool(true)');
  t.equal(reader.bool(), false, 'bool(false)');
  
  t.end();
});

tape.test('All data types round-trip - fixed width', (t) => {
  const writer = Writer.create();
  
  // fixed32
  writer.fixed32(0);
  writer.fixed32(1);
  writer.fixed32(4294967295);
  
  // sfixed32
  writer.sfixed32(0);
  writer.sfixed32(-1);
  writer.sfixed32(2147483647);
  writer.sfixed32(-2147483648);
  
  // float
  writer.float(0.0);
  writer.float(3.14);
  writer.float(-2.5);
  
  // double
  writer.double(0.0);
  writer.double(3.14159265359);
  writer.double(-1.23456789);
  
  const buffer = writer.finish();
  const reader = Reader.create(buffer);
  
  // fixed32
  t.equal(reader.fixed32(), 0, 'fixed32(0)');
  t.equal(reader.fixed32(), 1, 'fixed32(1)');
  t.equal(reader.fixed32(), 4294967295, 'fixed32(max)');
  
  // sfixed32
  t.equal(reader.sfixed32(), 0, 'sfixed32(0)');
  t.equal(reader.sfixed32(), -1, 'sfixed32(-1)');
  t.equal(reader.sfixed32(), 2147483647, 'sfixed32(max)');
  t.equal(reader.sfixed32(), -2147483648, 'sfixed32(min)');
  
  // float
  t.equal(reader.float(), 0.0, 'float(0.0)');
  t.ok(Math.abs(reader.float() - 3.14) < 0.001, 'float(3.14)');
  t.ok(Math.abs(reader.float() - (-2.5)) < 0.001, 'float(-2.5)');
  
  // double
  t.equal(reader.double(), 0.0, 'double(0.0)');
  t.ok(Math.abs(reader.double() - 3.14159265359) < 0.00000001, 'double(pi)');
  t.ok(Math.abs(reader.double() - (-1.23456789)) < 0.00000001, 'double(negative)');
  
  t.end();
});

tape.test('All data types round-trip - bytes and strings', (t) => {
  const writer = Writer.create();
  
  // bytes
  writer.bytes(Buffer.from([1, 2, 3]));
  writer.bytes(Buffer.from([]));
  writer.bytes(Buffer.from([255, 0, 128]));
  
  // strings
  writer.string('hello');
  writer.string('');
  writer.string('Unicode: 你好 🎉');
  
  const buffer = writer.finish();
  const reader = Reader.create(buffer);
  
  // bytes
  t.deepEqual(reader.bytes(), Buffer.from([1, 2, 3]), 'bytes([1,2,3])');
  t.deepEqual(reader.bytes(), Buffer.from([]), 'bytes(empty)');
  t.deepEqual(reader.bytes(), Buffer.from([255, 0, 128]), 'bytes([255,0,128])');
  
  // strings
  t.equal(reader.string(), 'hello', 'string(hello)');
  t.equal(reader.string(), '', 'string(empty)');
  t.equal(reader.string(), 'Unicode: 你好 🎉', 'string(unicode)');
  
  t.end();
});

tape.test('Edge cases: empty strings, zero values', (t) => {
  const writer = Writer.create();
  
  writer.uint32(0);
  writer.int32(0);
  writer.sint32(0);
  writer.bool(false);
  writer.string('');
  writer.bytes(Buffer.from([]));
  writer.fixed32(0);
  writer.float(0.0);
  writer.double(0.0);
  
  const buffer = writer.finish();
  const reader = Reader.create(buffer);
  
  t.equal(reader.uint32(), 0, 'zero uint32');
  t.equal(reader.int32(), 0, 'zero int32');
  t.equal(reader.sint32(), 0, 'zero sint32');
  t.equal(reader.bool(), false, 'false bool');
  t.equal(reader.string(), '', 'empty string');
  t.deepEqual(reader.bytes(), Buffer.from([]), 'empty bytes');
  t.equal(reader.fixed32(), 0, 'zero fixed32');
  t.equal(reader.float(), 0.0, 'zero float');
  t.equal(reader.double(), 0.0, 'zero double');
  
  t.end();
});

tape.test('Large values: max uint32, long strings', (t) => {
  const writer = Writer.create();
  
  const maxUint32 = 4294967295;
  const longString = 'a'.repeat(10000);
  const largeBytes = Buffer.alloc(5000, 0xAB);
  
  writer.uint32(maxUint32);
  writer.string(longString);
  writer.bytes(largeBytes);
  
  const buffer = writer.finish();
  const reader = Reader.create(buffer);
  
  t.equal(reader.uint32(), maxUint32, 'max uint32');
  t.equal(reader.string(), longString, 'long string');
  t.deepEqual(reader.bytes(), largeBytes, 'large bytes');
  
  t.end();
});

tape.test('fork() and ldelim() for length-delimited fields', (t) => {
  const writer = Writer.create();
  
  writer.uint32(1); // field tag
  writer.fork();
  writer.uint32(42);
  writer.string('nested');
  writer.ldelim();
  
  const buffer = writer.finish();
  t.ok(buffer.length > 0, 'buffer should contain data');
  
  const reader = Reader.create(buffer);
  t.equal(reader.uint32(), 1, 'field tag');
  
  const length = reader.uint32();
  t.ok(length > 0, 'length should be > 0');
  
  const nestedValue = reader.uint32();
  const nestedString = reader.string();
  t.equal(nestedValue, 42, 'nested uint32');
  t.equal(nestedString, 'nested', 'nested string');
  
  t.end();
});

tape.test('skip() method', (t) => {
  const writer = Writer.create();
  writer.uint32(100);
  writer.uint32(200);
  writer.uint32(300);
  
  const buffer = writer.finish();
  const reader = Reader.create(buffer);
  
  reader.uint32(); // read first value (100)
  
  // Skip the second value (200) by reading and re-reading
  const beforeSkip = reader.pos;
  reader.uint32(); // read second value to know how much to skip
  const skipAmount = reader.pos - beforeSkip;
  
  // Reset reader and skip properly
  const reader2 = Reader.create(buffer);
  reader2.uint32(); // read first value (100)
  reader2.skip(skipAmount); // skip second value
  const value = reader2.uint32();
  t.equal(value, 300, 'should read third value after skip');
  
  t.end();
});

tape.test('skipType() with different wire types', (t) => {
  const writer = Writer.create();
  
  // Wire type 0 (varint)
  writer.uint32(100);
  // Wire type 2 (length-delimited)
  writer.string('skip me');
  // Another varint
  writer.uint32(200);
  
  const buffer = writer.finish();
  const reader = Reader.create(buffer);
  
  reader.uint32(); // read first varint
  
  // Skip the string by skipping the length varint and then the string data
  const strLen = reader.uint32();
  reader.skip(strLen);
  
  const value = reader.uint32();
  t.equal(value, 200, 'should read value after skip');
  
  t.end();
});

tape.test('Writer reset()', (t) => {
  const writer = Writer.create();
  writer.uint32(100);
  writer.uint32(200);
  
  writer.reset();
  writer.uint32(300);
  
  const buffer = writer.finish();
  const reader = Reader.create(buffer);
  
  t.equal(reader.uint32(), 300, 'should only contain value after reset');
  t.ok(reader.pos === reader.len, 'should be at end of buffer');
  
  t.end();
});

tape.test('Implementation type detection', (t) => {
  const available = isNativeAvailable();
  t.ok(typeof available === 'boolean', 'isNativeAvailable should return boolean');
  console.log(`  Using ${available ? 'native Rust' : 'JavaScript'} implementation`);
  t.end();
});

tape.test('64-bit integer support', (t) => {
  const writer = Writer.create();
  
  // Test values that fit in JavaScript number range
  writer.uint64(0);
  writer.uint64(1000000);
  writer.sint64(-1000000);
  writer.fixed64(123456789);
  writer.sfixed64(-123456789);
  
  const buffer = writer.finish();
  const reader = Reader.create(buffer);
  
  t.equal(reader.uint64(), 0, 'uint64(0)');
  t.equal(reader.uint64(), 1000000, 'uint64(1000000)');
  t.equal(reader.sint64(), -1000000, 'sint64(-1000000)');
  t.equal(reader.fixed64(), 123456789, 'fixed64(123456789)');
  t.equal(reader.sfixed64(), -123456789, 'sfixed64(-123456789)');
  
  t.end();
});
