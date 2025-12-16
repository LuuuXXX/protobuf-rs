// Copyright (c) 2024 LuuuXXX
// Licensed under the BSD-3-Clause License.
// See LICENSE file in the project root for full license information.

/**
 * 流式处理示例
 * Streaming Processing Example
 * 
 * 演示如何使用 protobuf-rs 进行流式数据处理。
 * Demonstrates streaming data processing with protobuf-rs.
 */

const { Reader, Writer } = require('../integration/protobufjs-adapter');
const { Readable, Writable, Transform } = require('stream');

console.log('='.repeat(80));
console.log('流式处理示例 | Streaming Processing Example');
console.log('='.repeat(80));
console.log();

// =============================================================================
// 示例 1: 连续编码多个消息 | Example 1: Encoding Multiple Messages Sequentially
// =============================================================================
console.log('示例 1: 连续编码多个消息到单个流');
console.log('Example 1: Encoding Multiple Messages to a Single Stream');
console.log('-'.repeat(80));

// message LogEntry {
//   uint64 timestamp = 1;
//   string level = 2;
//   string message = 3;
// }

function encodeLogEntry(entry) {
  const writer = Writer.create();
  
  if (entry.timestamp !== undefined) {
    writer.uint32((1 << 3) | 0);
    writer.uint64(entry.timestamp);
  }
  
  if (entry.level) {
    writer.uint32((2 << 3) | 2);
    writer.string(entry.level);
  }
  
  if (entry.message) {
    writer.uint32((3 << 3) | 2);
    writer.string(entry.message);
  }
  
  return writer.finish();
}

// 创建长度前缀的消息（用于流式传输）| Create length-prefixed messages (for streaming)
function encodeLengthPrefixedMessage(buffer) {
  const lengthWriter = Writer.create();
  lengthWriter.uint32(buffer.length);
  const lengthBuffer = lengthWriter.finish();
  
  return Buffer.concat([lengthBuffer, buffer]);
}

const logEntries = [
  { timestamp: Date.now(), level: 'INFO', message: '应用启动 | Application started' },
  { timestamp: Date.now(), level: 'DEBUG', message: '加载配置 | Loading configuration' },
  { timestamp: Date.now(), level: 'INFO', message: '服务器监听 8080 端口 | Server listening on port 8080' },
  { timestamp: Date.now(), level: 'WARN', message: '高内存使用 | High memory usage detected' },
  { timestamp: Date.now(), level: 'ERROR', message: '数据库连接失败 | Database connection failed' }
];

console.log(`编码 ${logEntries.length} 条日志 | Encoding ${logEntries.length} log entries:`);

const encodedLogs = [];
let totalSize = 0;

logEntries.forEach((entry, index) => {
  const messageBuffer = encodeLogEntry(entry);
  const lengthPrefixed = encodeLengthPrefixedMessage(messageBuffer);
  encodedLogs.push(lengthPrefixed);
  totalSize += lengthPrefixed.length;
  
  console.log(`  [${index + 1}] ${entry.level}: ${messageBuffer.length} bytes (${lengthPrefixed.length} with length prefix)`);
});

console.log(`总大小 | Total size: ${totalSize} bytes`);
console.log();

// 合并所有消息到一个流 | Combine all messages into a single stream
const streamBuffer = Buffer.concat(encodedLogs);
console.log('流缓冲区大小 | Stream buffer size:', streamBuffer.length, 'bytes');
console.log();

// =============================================================================
// 示例 2: 从流中解码多个消息 | Example 2: Decoding Multiple Messages from Stream
// =============================================================================
console.log('示例 2: 从流中解码多个消息');
console.log('Example 2: Decoding Multiple Messages from Stream');
console.log('-'.repeat(80));

function decodeLogEntry(buffer) {
  const reader = Reader.create(buffer);
  const entry = {};
  
  while (reader.pos < reader.len) {
    const tag = reader.uint32();
    const fieldNumber = tag >>> 3;
    
    switch (fieldNumber) {
      case 1:
        entry.timestamp = reader.uint64();
        break;
      case 2:
        entry.level = reader.string();
        break;
      case 3:
        entry.message = reader.string();
        break;
      default:
        reader.skipType(tag & 7);
    }
  }
  
  return entry;
}

// 从流缓冲区解码所有消息 | Decode all messages from stream buffer
let position = 0;
const decodedEntries = [];

while (position < streamBuffer.length) {
  // 读取消息长度 | Read message length
  const lengthReader = Reader.create(streamBuffer.slice(position));
  const messageLength = lengthReader.uint32();
  position += lengthReader.pos;
  
  // 读取消息数据 | Read message data
  const messageBuffer = streamBuffer.slice(position, position + messageLength);
  position += messageLength;
  
  // 解码消息 | Decode message
  const entry = decodeLogEntry(messageBuffer);
  decodedEntries.push(entry);
  
  console.log(`  [${decodedEntries.length}] ${entry.level}: ${entry.message}`);
}

console.log(`成功解码 ${decodedEntries.length} 条消息 | Successfully decoded ${decodedEntries.length} messages`);
console.log();

// =============================================================================
// 示例 3: Node.js Transform Stream | Example 3: Node.js Transform Stream
// =============================================================================
console.log('示例 3: 使用 Node.js Transform Stream 处理数据');
console.log('Example 3: Processing Data with Node.js Transform Stream');
console.log('-'.repeat(80));

// 创建一个将对象编码为 Protobuf 的 Transform Stream
// Create a Transform Stream that encodes objects to Protobuf
class ProtobufEncodeStream extends Transform {
  constructor() {
    super({ objectMode: true });
  }
  
  _transform(entry, encoding, callback) {
    try {
      const messageBuffer = encodeLogEntry(entry);
      const lengthPrefixed = encodeLengthPrefixedMessage(messageBuffer);
      this.push(lengthPrefixed);
      callback();
    } catch (error) {
      callback(error);
    }
  }
}

// 创建一个将 Protobuf 解码为对象的 Transform Stream
// Create a Transform Stream that decodes Protobuf to objects
class ProtobufDecodeStream extends Transform {
  constructor() {
    super({ objectMode: true });
    this.buffer = Buffer.alloc(0);
  }
  
  _transform(chunk, encoding, callback) {
    // 追加新数据 | Append new data
    this.buffer = Buffer.concat([this.buffer, chunk]);
    
    // 尝试解码所有完整的消息 | Try to decode all complete messages
    try {
      let position = 0;
      
      while (position < this.buffer.length) {
        // 检查是否有足够的数据读取长度 | Check if enough data to read length
        if (this.buffer.length - position < 5) {
          break; // 需要更多数据 | Need more data
        }
        
        // 读取消息长度 | Read message length
        const lengthReader = Reader.create(this.buffer.slice(position));
        const messageLength = lengthReader.uint32();
        const lengthSize = lengthReader.pos;
        
        // 检查是否有完整的消息 | Check if complete message available
        if (this.buffer.length - position - lengthSize < messageLength) {
          break; // 需要更多数据 | Need more data
        }
        
        // 解码消息 | Decode message
        const messageBuffer = this.buffer.slice(position + lengthSize, position + lengthSize + messageLength);
        const entry = decodeLogEntry(messageBuffer);
        this.push(entry);
        
        position += lengthSize + messageLength;
      }
      
      // 保留未处理的数据 | Keep unprocessed data
      this.buffer = this.buffer.slice(position);
      callback();
    } catch (error) {
      callback(error);
    }
  }
}

console.log('创建编码流管道 | Creating encoding stream pipeline:');

// 创建可读流（模拟数据源）| Create readable stream (simulating data source)
const dataSource = new Readable({
  objectMode: true,
  read() {}
});

// 创建编码流 | Create encoding stream
const encodeStream = new ProtobufEncodeStream();

// 收集结果 | Collect results
const encodedChunks = [];
encodeStream.on('data', (chunk) => {
  encodedChunks.push(chunk);
  console.log(`  编码块 | Encoded chunk: ${chunk.length} bytes`);
});

encodeStream.on('end', () => {
  console.log(`  完成编码 | Encoding complete: ${encodedChunks.length} chunks`);
  console.log();
  
  // 现在测试解码 | Now test decoding
  console.log('创建解码流管道 | Creating decoding stream pipeline:');
  
  const decodeDataSource = new Readable({
    read() {}
  });
  
  const decodeStream = new ProtobufDecodeStream();
  
  let decodedCount = 0;
  decodeStream.on('data', (entry) => {
    decodedCount++;
    console.log(`  解码消息 | Decoded message ${decodedCount}: [${entry.level}] ${entry.message}`);
  });
  
  decodeStream.on('end', () => {
    console.log(`  解码完成 | Decoding complete: ${decodedCount} messages`);
    console.log();
    continueExamples();
  });
  
  // 推送数据到解码流 | Push data to decode stream
  decodeDataSource.pipe(decodeStream);
  encodedChunks.forEach(chunk => decodeDataSource.push(chunk));
  decodeDataSource.push(null);
});

// 推送数据到编码流 | Push data to encode stream
dataSource.pipe(encodeStream);
logEntries.forEach(entry => dataSource.push(entry));
dataSource.push(null);

// =============================================================================
// 继续其他示例 | Continue with other examples
// =============================================================================
function continueExamples() {
  // 示例 4: 批量处理优化 | Example 4: Batch Processing Optimization
  console.log('示例 4: 批量处理优化');
  console.log('Example 4: Batch Processing Optimization');
  console.log('-'.repeat(80));
  
  // 批量编码可以重用 Writer 以提高性能
  // Batch encoding can reuse Writer for better performance
  const writer = Writer.create();
  const batchSize = 100;
  const batches = [];
  
  console.log(`创建 ${batchSize} 条消息的批次 | Creating batch of ${batchSize} messages:`);
  
  const startTime = Date.now();
  
  for (let i = 0; i < batchSize; i++) {
    writer.reset(); // 重用 writer | Reuse writer
    
    const entry = {
      timestamp: Date.now() + i,
      level: ['INFO', 'DEBUG', 'WARN', 'ERROR'][i % 4],
      message: `批量消息 ${i} | Batch message ${i}`
    };
    
    // 编码消息 | Encode message
    if (entry.timestamp !== undefined) {
      writer.uint32((1 << 3) | 0);
      writer.uint64(entry.timestamp);
    }
    if (entry.level) {
      writer.uint32((2 << 3) | 2);
      writer.string(entry.level);
    }
    if (entry.message) {
      writer.uint32((3 << 3) | 2);
      writer.string(entry.message);
    }
    
    const buffer = writer.finish();
    batches.push(encodeLengthPrefixedMessage(buffer));
  }
  
  const elapsed = Date.now() - startTime;
  const totalBatchSize = batches.reduce((sum, b) => sum + b.length, 0);
  
  console.log(`  处理时间 | Processing time: ${elapsed}ms`);
  console.log(`  平均速度 | Average speed: ${(batchSize / elapsed * 1000).toFixed(0)} messages/sec`);
  console.log(`  总大小 | Total size: ${totalBatchSize} bytes`);
  console.log(`  平均消息大小 | Average message size: ${(totalBatchSize / batchSize).toFixed(1)} bytes`);
  console.log();
  
  // 示例 5: 实时日志流模拟 | Example 5: Real-time Log Stream Simulation
  console.log('示例 5: 实时日志流模拟（每秒 10 条）');
  console.log('Example 5: Real-time Log Stream Simulation (10 per second)');
  console.log('-'.repeat(80));
  
  let messageCount = 0;
  const maxMessages = 20;
  
  const logInterval = setInterval(() => {
    if (messageCount >= maxMessages) {
      clearInterval(logInterval);
      console.log();
      finishExamples();
      return;
    }
    
    const entry = {
      timestamp: Date.now(),
      level: ['INFO', 'DEBUG', 'WARN', 'ERROR'][messageCount % 4],
      message: `实时消息 ${messageCount} | Real-time message ${messageCount}`
    };
    
    const buffer = encodeLogEntry(entry);
    console.log(`  [${new Date(entry.timestamp).toISOString()}] ${entry.level}: ${buffer.length} bytes`);
    
    messageCount++;
  }, 100); // 每 100ms 一条 | One every 100ms
}

function finishExamples() {
  console.log('='.repeat(80));
  console.log('✅ 所有流式处理示例完成！');
  console.log('✅ All Streaming Examples Completed!');
  console.log('='.repeat(80));
  console.log();
  console.log('关键要点 | Key Takeaways:');
  console.log('1. 使用长度前缀在流中传输多个消息');
  console.log('   Use length prefixes to transmit multiple messages in streams');
  console.log('2. Transform Streams 提供了优雅的流式处理模式');
  console.log('   Transform Streams provide elegant streaming patterns');
  console.log('3. 重用 Writer 对象可提高批量处理性能');
  console.log('   Reusing Writer objects improves batch processing performance');
  console.log('4. 流式处理适合大数据集和实时场景');
  console.log('   Streaming is ideal for large datasets and real-time scenarios');
  console.log();
}
