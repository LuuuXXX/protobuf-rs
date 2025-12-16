// Copyright (c) 2024 LuuuXXX
// Licensed under the BSD-3-Clause License.
// See LICENSE file in the project root for full license information.

/**
 * Base64 编码示例
 * Base64 Encoding Example
 * 
 * 演示如何使用 protobuf-rs 配合 Base64 编码进行数据传输。
 * Demonstrates how to use protobuf-rs with Base64 encoding for data transmission.
 */

const { Reader, Writer } = require('../integration/protobufjs-adapter');

console.log('='.repeat(80));
console.log('Base64 编码示例 | Base64 Encoding Example');
console.log('='.repeat(80));
console.log();

// =============================================================================
// 示例 1: 基本的 Base64 编码和解码 | Example 1: Basic Base64 Encode/Decode
// =============================================================================
console.log('示例 1: 将 Protobuf 消息编码为 Base64 字符串');
console.log('Example 1: Encoding Protobuf Messages to Base64 Strings');
console.log('-'.repeat(80));

// 创建一个简单的消息 | Create a simple message
const writer = Writer.create();
writer.uint32((1 << 3) | 0);    // 字段 1: id
writer.uint32(12345);
writer.uint32((2 << 3) | 2);    // 字段 2: name
writer.string('Alice');
writer.uint32((3 << 3) | 0);    // 字段 3: age
writer.uint32(30);

const buffer = writer.finish();

console.log('原始二进制数据 (hex) | Raw binary (hex):');
console.log(' ', buffer.toString('hex'));
console.log('原始大小 | Raw size:', buffer.length, '字节');
console.log();

// 转换为 Base64 | Convert to Base64
const base64String = buffer.toString('base64');
console.log('Base64 编码 | Base64 encoded:');
console.log(' ', base64String);
console.log('Base64 大小 | Base64 size:', base64String.length, '字符');
console.log('大小增长 | Size increase:', 
  ((base64String.length / buffer.length - 1) * 100).toFixed(1) + '%');
console.log();

// 从 Base64 解码 | Decode from Base64
const decodedBuffer = Buffer.from(base64String, 'base64');
console.log('从 Base64 解码 | Decoded from Base64:');

const reader = Reader.create(decodedBuffer);
const message = {};

while (reader.pos < reader.len) {
  const tag = reader.uint32();
  const fieldNumber = tag >>> 3;
  
  switch (fieldNumber) {
    case 1:
      message.id = reader.uint32();
      break;
    case 2:
      message.name = reader.string();
      break;
    case 3:
      message.age = reader.uint32();
      break;
    default:
      reader.skipType(tag & 7);
  }
}

console.log('解码的消息 | Decoded message:', message);
console.log();

// =============================================================================
// 示例 2: URL 安全的 Base64 编码 | Example 2: URL-Safe Base64 Encoding
// =============================================================================
console.log('示例 2: URL 安全的 Base64 编码（用于 URL 参数）');
console.log('Example 2: URL-Safe Base64 Encoding (for URL Parameters)');
console.log('-'.repeat(80));

function toUrlSafeBase64(buffer) {
  return buffer.toString('base64')
    .replace(/\+/g, '-')
    .replace(/\//g, '_')
    .replace(/=+$/, '');
}

function fromUrlSafeBase64(str) {
  // 恢复标准 Base64 格式 | Restore standard Base64 format
  str = str.replace(/-/g, '+').replace(/_/g, '/');
  
  // 添加必要的填充 | Add necessary padding
  while (str.length % 4) {
    str += '=';
  }
  
  return Buffer.from(str, 'base64');
}

const urlSafeBase64 = toUrlSafeBase64(buffer);
console.log('URL 安全的 Base64 | URL-safe Base64:');
console.log(' ', urlSafeBase64);
console.log();

// 示例 URL | Example URL
const exampleUrl = `https://api.example.com/data?msg=${urlSafeBase64}`;
console.log('示例 URL | Example URL:');
console.log(' ', exampleUrl);
console.log();

// 从 URL 参数解码 | Decode from URL parameter
const decodedFromUrl = fromUrlSafeBase64(urlSafeBase64);
console.log('从 URL 参数解码 | Decoded from URL parameter:');
console.log('  原始数据匹配 | Original data matches:', decodedFromUrl.equals(buffer));
console.log();

// =============================================================================
// 示例 3: JSON 中的二进制数据 | Example 3: Binary Data in JSON
// =============================================================================
console.log('示例 3: 在 JSON 中传输 Protobuf 数据');
console.log('Example 3: Transmitting Protobuf Data in JSON');
console.log('-'.repeat(80));

// 创建一个包含二进制数据的 JSON 对象 | Create JSON with binary data
const jsonMessage = {
  type: 'user-update',
  timestamp: Date.now(),
  data: buffer.toString('base64'),
  metadata: {
    version: '1.0',
    compressed: false
  }
};

const jsonString = JSON.stringify(jsonMessage, null, 2);
console.log('JSON 消息 | JSON message:');
console.log(jsonString);
console.log();

// 从 JSON 解析并解码 Protobuf 数据 | Parse JSON and decode Protobuf
const parsedJson = JSON.parse(jsonString);
const protobufData = Buffer.from(parsedJson.data, 'base64');

console.log('从 JSON 提取的 Protobuf 数据 | Protobuf data from JSON:');
console.log('  类型 | Type:', parsedJson.type);
console.log('  时间戳 | Timestamp:', new Date(parsedJson.timestamp).toISOString());
console.log('  数据大小 | Data size:', protobufData.length, '字节');
console.log();

// =============================================================================
// 示例 4: HTTP 传输示例 | Example 4: HTTP Transmission Example
// =============================================================================
console.log('示例 4: 模拟 HTTP API 请求/响应');
console.log('Example 4: Simulating HTTP API Request/Response');
console.log('-'.repeat(80));

// 定义消息类型 | Define message types
// message ApiRequest {
//   string method = 1;
//   uint32 user_id = 2;
//   bytes payload = 3;
// }

function encodeApiRequest(request) {
  const writer = Writer.create();
  
  if (request.method) {
    writer.uint32((1 << 3) | 2);
    writer.string(request.method);
  }
  
  if (request.user_id !== undefined) {
    writer.uint32((2 << 3) | 0);
    writer.uint32(request.user_id);
  }
  
  if (request.payload) {
    writer.uint32((3 << 3) | 2);
    writer.bytes(request.payload);
  }
  
  return writer.finish();
}

function decodeApiRequest(buffer) {
  const reader = Reader.create(buffer);
  const request = {};
  
  while (reader.pos < reader.len) {
    const tag = reader.uint32();
    const fieldNumber = tag >>> 3;
    
    switch (fieldNumber) {
      case 1:
        request.method = reader.string();
        break;
      case 2:
        request.user_id = reader.uint32();
        break;
      case 3:
        request.payload = reader.bytes();
        break;
      default:
        reader.skipType(tag & 7);
    }
  }
  
  return request;
}

// 客户端：创建请求 | Client: Create request
const apiRequest = {
  method: 'getUserProfile',
  user_id: 12345,
  payload: Buffer.from('additional data')
};

console.log('客户端请求 | Client request:', apiRequest);

const requestBuffer = encodeApiRequest(apiRequest);
const requestBase64 = requestBuffer.toString('base64');

console.log('编码为 Base64 | Encoded to Base64:', requestBase64);
console.log();

// 模拟 HTTP 传输 | Simulate HTTP transmission
console.log('HTTP 请求示例 | Example HTTP Request:');
console.log('POST /api/v1/rpc HTTP/1.1');
console.log('Content-Type: application/protobuf');
console.log('X-Protobuf-Encoding: base64');
console.log('Content-Length:', requestBase64.length);
console.log();
console.log(requestBase64);
console.log();

// 服务器：解码请求 | Server: Decode request
const receivedBuffer = Buffer.from(requestBase64, 'base64');
const decodedRequest = decodeApiRequest(receivedBuffer);

console.log('服务器接收的请求 | Server received request:', decodedRequest);
console.log();

// =============================================================================
// 示例 5: 大数据的 Base64 编码性能 | Example 5: Base64 Performance with Large Data
// =============================================================================
console.log('示例 5: 大数据的 Base64 编码性能测试');
console.log('Example 5: Base64 Encoding Performance with Large Data');
console.log('-'.repeat(80));

// 创建一个较大的消息 | Create a larger message
const largeWriter = Writer.create();
for (let i = 0; i < 1000; i++) {
  largeWriter.uint32((1 << 3) | 0);
  largeWriter.uint32(i);
  largeWriter.uint32((2 << 3) | 2);
  largeWriter.string(`Item ${i}`);
}

const largeBuffer = largeWriter.finish();

console.log('原始大小 | Raw size:', largeBuffer.length, '字节');

const start = Date.now();
let base64Result;
for (let i = 0; i < 1000; i++) {
  base64Result = largeBuffer.toString('base64');
}
const encodeTime = Date.now() - start;

console.log('Base64 大小 | Base64 size:', base64Result.length, '字符');
console.log('编码时间 (1000次) | Encoding time (1000x):', encodeTime, 'ms');

const decodeStart = Date.now();
for (let i = 0; i < 1000; i++) {
  Buffer.from(base64Result, 'base64');
}
const decodeTime = Date.now() - decodeStart;

console.log('解码时间 (1000次) | Decoding time (1000x):', decodeTime, 'ms');
console.log();

// =============================================================================
// 示例 6: Base64 vs 十六进制编码对比 | Example 6: Base64 vs Hex Encoding
// =============================================================================
console.log('示例 6: Base64 vs 十六进制编码对比');
console.log('Example 6: Base64 vs Hex Encoding Comparison');
console.log('-'.repeat(80));

const testBuffer = buffer; // 使用之前的 buffer

const hexString = testBuffer.toString('hex');
const base64StringCompare = testBuffer.toString('base64');

console.log('比较 | Comparison:');
console.log('  原始大小 | Raw size:', testBuffer.length, '字节');
console.log('  十六进制 | Hex:', hexString.length, '字符', 
  `(${(hexString.length / testBuffer.length).toFixed(1)}x)`);
console.log('  Base64:', base64StringCompare.length, '字符',
  `(${(base64StringCompare.length / testBuffer.length).toFixed(1)}x)`);
console.log();
console.log('结论 | Conclusion: Base64 比十六进制节省约',
  ((1 - base64StringCompare.length / hexString.length) * 100).toFixed(0) + '%',
  '的空间');
console.log('Conclusion: Base64 saves about',
  ((1 - base64StringCompare.length / hexString.length) * 100).toFixed(0) + '%',
  'space compared to hex');
console.log();

// =============================================================================
// 总结 | Summary
// =============================================================================
console.log('='.repeat(80));
console.log('✅ 所有 Base64 示例执行成功！');
console.log('✅ All Base64 examples executed successfully!');
console.log('='.repeat(80));
console.log();
console.log('最佳实践 | Best Practices:');
console.log('1. 在文本协议（HTTP、JSON）中使用 Base64 传输二进制数据');
console.log('   Use Base64 to transmit binary data in text protocols (HTTP, JSON)');
console.log('2. URL 参数使用 URL 安全的 Base64 编码');
console.log('   Use URL-safe Base64 for URL parameters');
console.log('3. Base64 比十六进制更节省空间（约 33%）');
console.log('   Base64 is more space-efficient than hex (about 33%)');
console.log('4. Base64 编码增加约 33% 的大小');
console.log('   Base64 encoding increases size by about 33%');
console.log();
