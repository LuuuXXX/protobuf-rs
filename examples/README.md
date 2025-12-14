# Examples / 示例

This directory contains comprehensive examples demonstrating how to use protobuf-rs.

本目录包含全面的示例，演示如何使用 protobuf-rs。

## Running Examples / 运行示例

All examples can be run with Node.js:

所有示例都可以使用 Node.js 运行：

```bash
node examples/basic-encode-decode.js
node examples/base64-encoding.js
node examples/long-type-handling.js
node examples/streaming-processing.js
node examples/protobufjs-migration.js
node examples/drop-in-replacement.js
```

## Example List / 示例列表

### 1. Basic Encode/Decode / 基本编码解码

**File:** `basic-encode-decode.js`

Demonstrates the fundamentals of Protocol Buffer encoding and decoding:
- Simple data types (integers, floats, strings, bytes)
- Messages with field tags
- Nested messages
- Repeated fields (arrays)
- Packed vs unpacked encoding

演示 Protocol Buffer 编码和解码的基础知识：
- 简单数据类型（整数、浮点数、字符串、字节）
- 带字段标签的消息
- 嵌套消息
- 重复字段（数组）
- 打包与未打包编码

**Key Concepts:**
- Field tags = (field_number << 3) | wire_type
- Wire types (0=varint, 1=64-bit, 2=length-delimited, 5=32-bit)
- Length-delimited encoding for strings, bytes, and nested messages

**关键概念：**
- 字段标签 = (字段编号 << 3) | 线路类型
- 线路类型（0=varint, 1=64位, 2=长度限定, 5=32位）
- 字符串、字节和嵌套消息的长度限定编码

---

### 2. Base64 Encoding / Base64 编码

**File:** `base64-encoding.js`

Shows how to use Base64 encoding with Protocol Buffers:
- Converting binary protobuf data to Base64 strings
- URL-safe Base64 encoding
- Transmitting protobuf data in JSON
- HTTP request/response examples
- Performance comparison (Base64 vs Hex)

展示如何在 Protocol Buffers 中使用 Base64 编码：
- 将二进制 protobuf 数据转换为 Base64 字符串
- URL 安全的 Base64 编码
- 在 JSON 中传输 protobuf 数据
- HTTP 请求/响应示例
- 性能对比（Base64 vs 十六进制）

**Use Cases:**
- Sending binary data over text-based protocols (HTTP, JSON, XML)
- URL query parameters
- Web APIs
- REST APIs with protobuf payloads

**使用场景：**
- 通过基于文本的协议发送二进制数据（HTTP、JSON、XML）
- URL 查询参数
- Web API
- 带有 protobuf 负载的 REST API

---

### 3. Long Type Handling / Long 类型处理

**File:** `long-type-handling.js`

Explains JavaScript's limitations with 64-bit integers and best practices:
- JavaScript safe integer range (±2^53)
- uint64, sint64 encoding/decoding
- fixed64, sfixed64 types
- Common use cases (timestamps, IDs)
- Precision issues with large integers
- Recommendations and workarounds

解释 JavaScript 在 64 位整数方面的限制和最佳实践：
- JavaScript 安全整数范围（±2^53）
- uint64、sint64 编码/解码
- fixed64、sfixed64 类型
- 常见用例（时间戳、ID）
- 大整数的精度问题
- 建议和解决方法

**Important Notes:**
- JavaScript can only safely represent integers up to ±2^53
- Most applications don't need values larger than uint32 (4 billion)
- For timestamps, use uint32 (seconds) or uint64 (milliseconds within safe range)
- For very large integers, use string representation or Long library

**重要说明：**
- JavaScript 只能安全表示最多 ±2^53 的整数
- 大多数应用不需要大于 uint32（40 亿）的值
- 对于时间戳，使用 uint32（秒）或 uint64（安全范围内的毫秒）
- 对于非常大的整数，使用字符串表示或 Long 库

---

### 4. Streaming Processing / 流式处理

**File:** `streaming-processing.js`

Demonstrates streaming patterns for processing large datasets:
- Length-prefixed message encoding for streams
- Decoding multiple messages from a stream
- Node.js Transform Streams
- Batch processing optimization
- Real-time log streaming simulation

演示处理大数据集的流式模式：
- 流的长度前缀消息编码
- 从流中解码多个消息
- Node.js Transform Streams
- 批量处理优化
- 实时日志流模拟

**Use Cases:**
- Processing large log files
- Real-time data streaming
- Batch data export/import
- Network protocol implementations
- gRPC-style streaming

**使用场景：**
- 处理大型日志文件
- 实时数据流
- 批量数据导出/导入
- 网络协议实现
- gRPC 风格的流式传输

**Key Techniques:**
- Length prefixing: Prepend message length for delimiting in streams
- Writer reuse: Reuse Writer objects for better performance
- Transform Streams: Use Node.js streams for elegant data processing

**关键技术：**
- 长度前缀：在流中为分隔添加消息长度前缀
- Writer 重用：重用 Writer 对象以获得更好的性能
- Transform Streams：使用 Node.js 流进行优雅的数据处理

---

### 5. protobuf.js Migration / protobuf.js 迁移

**File:** `protobufjs-migration.js`

Complete guide for migrating from protobuf.js to protobuf-rs:
- Performance comparison benchmarks
- API compatibility examples
- Error handling
- Before/after code comparisons

从 protobuf.js 迁移到 protobuf-rs 的完整指南：
- 性能对比基准测试
- API 兼容性示例
- 错误处理
- 前后代码对比

**Migration Steps:**
1. Install `@protobuf-rs/core`
2. Replace `require('protobufjs')` with `require('@protobuf-rs/core/protobufjs-compat')`
3. No other code changes needed!
4. Enjoy 3-4x performance improvement

**迁移步骤：**
1. 安装 `@protobuf-rs/core`
2. 将 `require('protobufjs')` 替换为 `require('@protobuf-rs/core/protobufjs-compat')`
3. 无需其他代码修改！
4. 享受 3-4 倍的性能提升

---

### 6. Drop-in Replacement / 直接替换

**File:** `drop-in-replacement.js`

Demonstrates the drop-in replacement capability:
- Zero-code-change migration
- Automatic fallback to JavaScript
- Implementation type detection
- Performance verification

演示直接替换功能：
- 零代码修改迁移
- 自动降级到 JavaScript
- 实现类型检测
- 性能验证

---

## Performance Tips / 性能技巧

### 1. Reuse Writer Objects / 重用 Writer 对象

```javascript
const writer = Writer.create();
for (const item of items) {
  // Encode item
  const buffer = writer.finish();
  writer.reset(); // Reuse the writer
}
```

### 2. Pre-allocate Buffer Capacity / 预分配缓冲区容量

```javascript
// If you know the message will be ~1KB
const writer = Writer.withCapacity(1024);
```

### 3. Use Batch Operations / 使用批量操作

```javascript
// Good: Batch encoding
const encoded = encodeVarintBatchSimd(values);

// Avoid: Individual encoding
values.forEach(v => encodeVarint(v));
```

### 4. Use Packed Encoding for Repeated Fields / 对重复字段使用打包编码

```protobuf
message MyMessage {
  repeated uint32 numbers = 1 [packed=true];  // More efficient
}
```

## Common Patterns / 常见模式

### Message Definition / 消息定义

```javascript
// message User {
//   uint32 id = 1;
//   string name = 2;
//   bool active = 3;
// }

function encodeUser(user) {
  const writer = Writer.create();
  
  if (user.id !== undefined) {
    writer.uint32((1 << 3) | 0);  // Field 1, wire type 0
    writer.uint32(user.id);
  }
  
  if (user.name !== undefined) {
    writer.uint32((2 << 3) | 2);  // Field 2, wire type 2
    writer.string(user.name);
  }
  
  if (user.active !== undefined) {
    writer.uint32((3 << 3) | 0);  // Field 3, wire type 0
    writer.bool(user.active);
  }
  
  return writer.finish();
}

function decodeUser(buffer) {
  const reader = Reader.create(buffer);
  const user = {};
  
  while (reader.pos < reader.len) {
    const tag = reader.uint32();
    const fieldNumber = tag >>> 3;
    const wireType = tag & 7;
    
    switch (fieldNumber) {
      case 1:
        user.id = reader.uint32();
        break;
      case 2:
        user.name = reader.string();
        break;
      case 3:
        user.active = reader.bool();
        break;
      default:
        reader.skipType(wireType);
    }
  }
  
  return user;
}
```

### Nested Messages / 嵌套消息

```javascript
// Encode nested message as bytes
writer.uint32((field_number << 3) | 2);  // Wire type 2
const nestedBuffer = encodeNestedMessage(nested);
writer.bytes(nestedBuffer);
```

### Repeated Fields / 重复字段

```javascript
// Unpacked: One tag per value
for (const value of values) {
  writer.uint32((field_number << 3) | wire_type);
  writer.uint32(value);
}

// Packed: One tag, length-delimited
writer.uint32((field_number << 3) | 2);  // Wire type 2
writer.fork();
for (const value of values) {
  writer.uint32(value);
}
writer.ldelim();
```

## Troubleshooting / 故障排除

### Error: "Cannot find module" / 错误："找不到模块"

Make sure you've installed dependencies and built the native module:

确保已安装依赖并构建了原生模块：

```bash
npm install
npm run build
```

### Native module not loading / 原生模块未加载

Check if native module is available:

检查原生模块是否可用：

```javascript
const protobuf = require('@protobuf-rs/core/protobufjs-compat');
console.log('Native:', protobuf.isNativeAccelerated());
```

If false, the library will automatically fall back to JavaScript implementation.

如果为 false，库将自动降级到 JavaScript 实现。

### Type Errors with TypeScript / TypeScript 类型错误

Make sure to import types correctly:

确保正确导入类型：

```typescript
import { Reader, Writer } from '@protobuf-rs/core';
```

## Additional Resources / 其他资源

- [Main README](../README.md) - Project overview
- [Chinese README](../README.zh.md) - 中文说明
- [API Documentation](../docs/zh_CN/API.md) - API 文档
- [FAQ](../docs/zh_CN/FAQ.md) - 常见问题
- [Performance Report](../docs/PERFORMANCE_REPORT.md) - Performance benchmarks
- [Integration Guide](../docs/INTEGRATION_GUIDE.md) - Integration with protobuf.js

## Contributing / 贡献

Found a bug or have an idea for a new example? Please open an issue or submit a pull request!

发现错误或对新示例有想法？请提出 issue 或提交 pull request！

---

**Happy Coding! / 编码愉快！** 🚀
