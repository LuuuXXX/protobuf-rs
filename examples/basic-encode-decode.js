// Copyright (c) 2024 LuuuXXX
// Licensed under the BSD-3-Clause License.
// See LICENSE file in the project root for full license information.

/**
 * 基本的编码/解码示例
 * Basic Encode/Decode Example
 * 
 * 演示如何使用 protobuf-rs 进行基本的消息编码和解码操作。
 * Demonstrates basic message encoding and decoding with protobuf-rs.
 */

const { Reader, Writer } = require('../integration/protobufjs-adapter');

console.log('='.repeat(80));
console.log('基本编码/解码示例 | Basic Encode/Decode Example');
console.log('='.repeat(80));
console.log();

// =============================================================================
// 示例 1: 简单数据类型 | Example 1: Simple Data Types
// =============================================================================
console.log('示例 1: 编码和解码简单数据类型');
console.log('Example 1: Encoding and Decoding Simple Data Types');
console.log('-'.repeat(80));

// 编码 | Encode
const writer = Writer.create();
writer.uint32(42);           // 无符号整数 | Unsigned integer
writer.int32(-100);          // 有符号整数 | Signed integer
writer.bool(true);           // 布尔值 | Boolean
writer.float(3.14);          // 浮点数 | Float
writer.double(2.71828);      // 双精度浮点数 | Double
writer.string('你好世界');    // 字符串 | String
writer.bytes(Buffer.from([1, 2, 3, 4])); // 字节数组 | Bytes

const buffer = writer.finish();
console.log('编码后的缓冲区大小:', buffer.length, '字节');
console.log('Encoded buffer size:', buffer.length, 'bytes');
console.log('缓冲区内容 (hex):', buffer.toString('hex'));
console.log('Buffer content (hex):', buffer.toString('hex'));
console.log();

// 解码 | Decode
const reader = Reader.create(buffer);
console.log('解码的值 | Decoded values:');
console.log('  uint32:', reader.uint32());
console.log('  int32:', reader.int32());
console.log('  bool:', reader.bool());
console.log('  float:', reader.float());
console.log('  double:', reader.double());
console.log('  string:', reader.string());
console.log('  bytes:', reader.bytes());
console.log();

// =============================================================================
// 示例 2: 带字段标签的消息 | Example 2: Message with Field Tags
// =============================================================================
console.log('示例 2: 编码带字段标签的完整消息');
console.log('Example 2: Encoding a Complete Message with Field Tags');
console.log('-'.repeat(80));

// 定义消息结构 | Define message structure
// message Person {
//   uint32 id = 1;
//   string name = 2;
//   string email = 3;
//   bool active = 4;
// }

function encodePerson(person) {
  const writer = Writer.create();
  
  // 字段 1: id (uint32, wire type 0)
  if (person.id !== undefined) {
    writer.uint32((1 << 3) | 0);
    writer.uint32(person.id);
  }
  
  // 字段 2: name (string, wire type 2)
  if (person.name !== undefined) {
    writer.uint32((2 << 3) | 2);
    writer.string(person.name);
  }
  
  // 字段 3: email (string, wire type 2)
  if (person.email !== undefined) {
    writer.uint32((3 << 3) | 2);
    writer.string(person.email);
  }
  
  // 字段 4: active (bool, wire type 0)
  if (person.active !== undefined) {
    writer.uint32((4 << 3) | 0);
    writer.bool(person.active);
  }
  
  return writer.finish();
}

function decodePerson(buffer) {
  const reader = Reader.create(buffer);
  const person = {};
  
  while (reader.pos < reader.len) {
    const tag = reader.uint32();
    const fieldNumber = tag >>> 3;
    const wireType = tag & 7;
    
    switch (fieldNumber) {
      case 1:
        person.id = reader.uint32();
        break;
      case 2:
        person.name = reader.string();
        break;
      case 3:
        person.email = reader.string();
        break;
      case 4:
        person.active = reader.bool();
        break;
      default:
        reader.skipType(wireType);
    }
  }
  
  return person;
}

const person = {
  id: 1001,
  name: '张三',
  email: 'zhangsan@example.com',
  active: true
};

console.log('原始对象 | Original object:', person);

const personBuffer = encodePerson(person);
console.log('编码后大小 | Encoded size:', personBuffer.length, '字节');
console.log('编码内容 (hex) | Encoded (hex):', personBuffer.toString('hex'));

const decodedPerson = decodePerson(personBuffer);
console.log('解码对象 | Decoded object:', decodedPerson);
console.log();

// =============================================================================
// 示例 3: 嵌套消息 | Example 3: Nested Messages
// =============================================================================
console.log('示例 3: 嵌套消息编码');
console.log('Example 3: Nested Message Encoding');
console.log('-'.repeat(80));

// message Address {
//   string street = 1;
//   string city = 2;
// }
//
// message PersonWithAddress {
//   uint32 id = 1;
//   string name = 2;
//   Address address = 3;
// }

function encodeAddress(address) {
  const writer = Writer.create();
  
  if (address.street !== undefined) {
    writer.uint32((1 << 3) | 2);
    writer.string(address.street);
  }
  
  if (address.city !== undefined) {
    writer.uint32((2 << 3) | 2);
    writer.string(address.city);
  }
  
  return writer.finish();
}

function encodePersonWithAddress(person) {
  const writer = Writer.create();
  
  // 字段 1: id
  if (person.id !== undefined) {
    writer.uint32((1 << 3) | 0);
    writer.uint32(person.id);
  }
  
  // 字段 2: name
  if (person.name !== undefined) {
    writer.uint32((2 << 3) | 2);
    writer.string(person.name);
  }
  
  // 字段 3: address (嵌套消息 | nested message)
  if (person.address !== undefined) {
    writer.uint32((3 << 3) | 2);
    const addressBuffer = encodeAddress(person.address);
    writer.bytes(addressBuffer);
  }
  
  return writer.finish();
}

const personWithAddress = {
  id: 2001,
  name: '李四',
  address: {
    street: '北京路123号',
    city: '北京'
  }
};

console.log('原始对象 | Original object:', personWithAddress);

const nestedBuffer = encodePersonWithAddress(personWithAddress);
console.log('编码后大小 | Encoded size:', nestedBuffer.length, '字节');
console.log('编码内容 (hex) | Encoded (hex):', nestedBuffer.toString('hex'));
console.log();

// =============================================================================
// 示例 4: 重复字段 | Example 4: Repeated Fields
// =============================================================================
console.log('示例 4: 重复字段（数组）编码');
console.log('Example 4: Repeated Fields (Arrays) Encoding');
console.log('-'.repeat(80));

// message NumberList {
//   repeated uint32 numbers = 1;
// }

function encodeNumberList(numbers) {
  const writer = Writer.create();
  
  // 方式 1: 每个值一个标签 | Method 1: One tag per value
  for (const num of numbers) {
    writer.uint32((1 << 3) | 0);
    writer.uint32(num);
  }
  
  return writer.finish();
}

function encodeNumberListPacked(numbers) {
  const writer = Writer.create();
  
  // 方式 2: packed 编码 | Method 2: Packed encoding
  writer.uint32((1 << 3) | 2); // wire type 2 for packed
  writer.fork(); // 开始长度限定 | Start length-delimited
  
  for (const num of numbers) {
    writer.uint32(num);
  }
  
  writer.ldelim(); // 结束长度限定 | End length-delimited
  
  return writer.finish();
}

const numbers = [1, 2, 3, 4, 5, 100, 1000];

console.log('数字数组 | Number array:', numbers);

const unpackedBuffer = encodeNumberList(numbers);
console.log('未打包编码 | Unpacked encoding:', unpackedBuffer.length, '字节');

const packedBuffer = encodeNumberListPacked(numbers);
console.log('打包编码 | Packed encoding:', packedBuffer.length, '字节');
console.log('节省空间 | Space saved:', 
  ((1 - packedBuffer.length / unpackedBuffer.length) * 100).toFixed(1) + '%');
console.log();

// =============================================================================
// 总结 | Summary
// =============================================================================
console.log('='.repeat(80));
console.log('✅ 所有示例执行成功！');
console.log('✅ All examples executed successfully!');
console.log('='.repeat(80));
console.log();
console.log('学到的要点 | Key Takeaways:');
console.log('1. 使用 Writer 编码各种数据类型 | Use Writer to encode various data types');
console.log('2. 使用 Reader 解码数据 | Use Reader to decode data');
console.log('3. 字段标签 = (字段编号 << 3) | 线路类型 | Field tag = (field number << 3) | wire type');
console.log('4. 嵌套消息需要先编码为 bytes | Nested messages are encoded as bytes');
console.log('5. Packed 编码更高效用于数字数组 | Packed encoding is more efficient for number arrays');
console.log();
