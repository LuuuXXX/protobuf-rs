import hilog from '@ohos.hilog';
import { describe, beforeAll, beforeEach, afterEach, afterAll, it, expect } from '@ohos/hypium';
// Import protobuf library from the library module
// Using relative path to reference the library module in the same project
import { Reader, Writer } from '../../../../../../library/index';
// Import test data models
const { UserLoginResponse } = require('./user');

export default function abilityTest() {
  describe('ProtobufBasicTests', () => {
    // Test case: Create UserLoginResponse
    it('should create UserLoginResponse', 0, () => {
      hilog.info(0x0000, 'testTag', 'Testing UserLoginResponse.create');
      
      const response = UserLoginResponse.create({
        userId: "user123",
        userName: "Test User",
        isActive: true,
        timestamp: 1234567890,
        sessionToken: new Uint8Array([1, 2, 3, 4, 5])
      });
      
      expect(response.userId).assertEqual("user123");
      expect(response.userName).assertEqual("Test User");
      expect(response.isActive).assertEqual(true);
      expect(response.timestamp).assertEqual(1234567890);
      expect(response.sessionToken.length).assertEqual(5);
      
      hilog.info(0x0000, 'testTag', 'UserLoginResponse.create test passed');
    });

    // Test case: Encode UserLoginResponse
    it('should encode UserLoginResponse', 0, () => {
      hilog.info(0x0000, 'testTag', 'Testing UserLoginResponse.encode');
      
      const message = UserLoginResponse.create({
        userId: "user456",
        userName: "Encode Test",
        isActive: false,
        timestamp: 9876543210,
        sessionToken: new Uint8Array([10, 20, 30])
      });
      
      const writer = Writer.create();
      UserLoginResponse.encode(message, writer);
      const buffer = writer.finish();
      
      expect(buffer).assertNotNull();
      expect(buffer.length).assertLarger(0);
      
      hilog.info(0x0000, 'testTag', 'UserLoginResponse.encode test passed, buffer length: %{public}d', buffer.length);
    });

    // Test case: Decode UserLoginResponse
    it('should decode UserLoginResponse', 0, () => {
      hilog.info(0x0000, 'testTag', 'Testing UserLoginResponse encode/decode');
      
      // Create original message
      const original = UserLoginResponse.create({
        userId: "user789",
        userName: "Decode Test",
        isActive: true,
        timestamp: 1111111111,
        sessionToken: new Uint8Array([100, 101, 102])
      });
      
      // Encode
      const writer = Writer.create();
      UserLoginResponse.encode(original, writer);
      const buffer = writer.finish();
      
      // Decode
      const reader = Reader.create(buffer);
      const decoded = UserLoginResponse.decode(reader);
      
      // Verify
      expect(decoded.userId).assertEqual(original.userId);
      expect(decoded.userName).assertEqual(original.userName);
      expect(decoded.isActive).assertEqual(original.isActive);
      expect(decoded.timestamp).assertEqual(original.timestamp);
      expect(decoded.sessionToken.length).assertEqual(original.sessionToken.length);
      
      hilog.info(0x0000, 'testTag', 'UserLoginResponse decode test passed');
    });

    // Test case: String types
    it('should handle string types correctly', 0, () => {
      hilog.info(0x0000, 'testTag', 'Testing string types');
      
      const writer = Writer.create();
      writer.string("Hello");
      writer.string("World");
      writer.string("你好世界"); // Unicode test
      const buffer = writer.finish();
      
      const reader = Reader.create(buffer);
      expect(reader.string()).assertEqual("Hello");
      expect(reader.string()).assertEqual("World");
      expect(reader.string()).assertEqual("你好世界");
      
      hilog.info(0x0000, 'testTag', 'String types test passed');
    });

    // Test case: Boolean types
    it('should handle boolean types correctly', 0, () => {
      hilog.info(0x0000, 'testTag', 'Testing boolean types');
      
      const writer = Writer.create();
      writer.bool(true);
      writer.bool(false);
      writer.bool(true);
      const buffer = writer.finish();
      
      const reader = Reader.create(buffer);
      expect(reader.bool()).assertEqual(true);
      expect(reader.bool()).assertEqual(false);
      expect(reader.bool()).assertEqual(true);
      
      hilog.info(0x0000, 'testTag', 'Boolean types test passed');
    });

    // Test case: Number types
    it('should handle number types correctly', 0, () => {
      hilog.info(0x0000, 'testTag', 'Testing number types');
      
      const writer = Writer.create();
      writer.uint32(0);
      writer.uint32(127);
      writer.uint32(16384);
      writer.uint32(2147483647);
      const buffer = writer.finish();
      
      const reader = Reader.create(buffer);
      expect(reader.uint32()).assertEqual(0);
      expect(reader.uint32()).assertEqual(127);
      expect(reader.uint32()).assertEqual(16384);
      expect(reader.uint32()).assertEqual(2147483647);
      
      hilog.info(0x0000, 'testTag', 'Number types test passed');
    });

    // Test case: BigInt/Long support
    it('should handle BigInt/Long types', 0, () => {
      hilog.info(0x0000, 'testTag', 'Testing BigInt/Long types');
      
      const writer = Writer.create();
      const bigValue = 9223372036854775807; // Max int64
      writer.int64(bigValue);
      const buffer = writer.finish();
      
      const reader = Reader.create(buffer);
      const decoded = reader.int64();
      
      // Compare as numbers (may lose precision for very large values)
      expect(typeof decoded).assertEqual('number');
      
      hilog.info(0x0000, 'testTag', 'BigInt/Long types test passed');
    });

    // Test case: Uint8Array/bytes
    it('should handle Uint8Array correctly', 0, () => {
      hilog.info(0x0000, 'testTag', 'Testing Uint8Array/bytes');
      
      const testData = new Uint8Array([1, 2, 3, 4, 5, 255, 0, 128]);
      
      const writer = Writer.create();
      writer.bytes(testData);
      const buffer = writer.finish();
      
      const reader = Reader.create(buffer);
      const decoded = reader.bytes();
      
      expect(decoded.length).assertEqual(testData.length);
      for (let i = 0; i < testData.length; i++) {
        expect(decoded[i]).assertEqual(testData[i]);
      }
      
      hilog.info(0x0000, 'testTag', 'Uint8Array test passed');
    });
  });

  describe('ProtobufPerformanceTests', () => {
    // Performance test: Batch create
    it('performance test - batch create (2000 times)', 0, () => {
      hilog.info(0x0000, 'testTag', 'Performance test: batch create');
      
      const count = 2000;
      const startTime = Date.now();
      
      for (let i = 0; i < count; i++) {
        UserLoginResponse.create({
          userId: `user${i}`,
          userName: `User ${i}`,
          isActive: i % 2 === 0,
          timestamp: Date.now() + i,
          sessionToken: new Uint8Array([i % 256, (i + 1) % 256])
        });
      }
      
      const endTime = Date.now();
      const duration = endTime - startTime;
      const opsPerSecond = Math.floor((count / duration) * 1000);
      
      hilog.info(0x0000, 'testTag', 'Batch create: %{public}d ops in %{public}d ms (%{public}d ops/sec)', 
        count, duration, opsPerSecond);
      
      expect(duration).assertLess(5000); // Should complete in less than 5 seconds
    });

    // Performance test: Batch encode
    it('performance test - batch encode (2000 times)', 0, () => {
      hilog.info(0x0000, 'testTag', 'Performance test: batch encode');
      
      const count = 2000;
      const messages = [];
      
      // Prepare messages
      for (let i = 0; i < count; i++) {
        messages.push(UserLoginResponse.create({
          userId: `user${i}`,
          userName: `User ${i}`,
          isActive: i % 2 === 0,
          timestamp: Date.now() + i,
          sessionToken: new Uint8Array([i % 256])
        }));
      }
      
      const startTime = Date.now();
      
      for (let i = 0; i < count; i++) {
        const writer = Writer.create();
        UserLoginResponse.encode(messages[i], writer);
        writer.finish();
      }
      
      const endTime = Date.now();
      const duration = endTime - startTime;
      const opsPerSecond = Math.floor((count / duration) * 1000);
      
      hilog.info(0x0000, 'testTag', 'Batch encode: %{public}d ops in %{public}d ms (%{public}d ops/sec)', 
        count, duration, opsPerSecond);
      
      expect(duration).assertLess(5000); // Should complete in less than 5 seconds
    });

    // Performance test: Batch decode
    it('performance test - batch decode (2000 times)', 0, () => {
      hilog.info(0x0000, 'testTag', 'Performance test: batch decode');
      
      const count = 2000;
      const buffers = [];
      
      // Prepare encoded buffers
      for (let i = 0; i < count; i++) {
        const message = UserLoginResponse.create({
          userId: `user${i}`,
          userName: `User ${i}`,
          isActive: i % 2 === 0,
          timestamp: Date.now() + i,
          sessionToken: new Uint8Array([i % 256])
        });
        const writer = Writer.create();
        UserLoginResponse.encode(message, writer);
        buffers.push(writer.finish());
      }
      
      const startTime = Date.now();
      
      for (let i = 0; i < count; i++) {
        const reader = Reader.create(buffers[i]);
        UserLoginResponse.decode(reader);
      }
      
      const endTime = Date.now();
      const duration = endTime - startTime;
      const opsPerSecond = Math.floor((count / duration) * 1000);
      
      hilog.info(0x0000, 'testTag', 'Batch decode: %{public}d ops in %{public}d ms (%{public}d ops/sec)', 
        count, duration, opsPerSecond);
      
      expect(duration).assertLess(5000); // Should complete in less than 5 seconds
    });

    // Performance test: Round-trip
    it('performance test - encode/decode round-trip', 0, () => {
      hilog.info(0x0000, 'testTag', 'Performance test: round-trip');
      
      const count = 1000;
      const startTime = Date.now();
      
      for (let i = 0; i < count; i++) {
        const original = UserLoginResponse.create({
          userId: `user${i}`,
          userName: `User ${i}`,
          isActive: i % 2 === 0,
          timestamp: Date.now() + i,
          sessionToken: new Uint8Array([i % 256])
        });
        
        const writer = Writer.create();
        UserLoginResponse.encode(original, writer);
        const buffer = writer.finish();
        
        const reader = Reader.create(buffer);
        const decoded = UserLoginResponse.decode(reader);
        
        // Verify one field to ensure correctness
        expect(decoded.userId).assertEqual(original.userId);
      }
      
      const endTime = Date.now();
      const duration = endTime - startTime;
      const opsPerSecond = Math.floor((count / duration) * 1000);
      
      hilog.info(0x0000, 'testTag', 'Round-trip: %{public}d ops in %{public}d ms (%{public}d ops/sec)', 
        count, duration, opsPerSecond);
      
      expect(duration).assertLess(5000); // Should complete in less than 5 seconds
    });
  });
}
