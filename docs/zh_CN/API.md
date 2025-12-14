# API 文档

本文档提供 protobuf-rs 的完整 API 参考。

## 目录

- [核心 API](#核心-api)
  - [Varint 操作](#varint-操作)
  - [ZigZag 操作](#zigzag-操作)
  - [字段标签操作](#字段标签操作)
- [Reader 类](#reader-类)
- [Writer 类](#writer-类)
- [批量操作](#批量操作)
- [集成 API](#集成-api)
- [性能监控](#性能监控)

## 核心 API

### Varint 操作

Protocol Buffers 使用 varint 编码来高效地表示整数。较小的数值使用较少的字节。

#### `encodeVarint(value: number): Buffer`

将 64 位有符号整数编码为 Protocol Buffer varint。

**参数：**
- `value` (number) - 要编码的整数（-2^63 到 2^63-1）

**返回值：** Buffer - 包含编码 varint 的缓冲区（1-10 字节）

**示例：**
```javascript
const { encodeVarint } = require('@protobuf-rs/core');

const encoded = encodeVarint(300);
console.log(encoded); // <Buffer ac 02>

// 小数值使用更少的字节
console.log(encodeVarint(1).length);    // 1 字节
console.log(encodeVarint(127).length);  // 1 字节
console.log(encodeVarint(128).length);  // 2 字节
console.log(encodeVarint(16384).length); // 3 字节
```

**性能：** ~621K ops/sec（比纯 JavaScript 快 3.45 倍）

#### `decodeVarint(buffer: Buffer): number`

从缓冲区解码 Protocol Buffer varint。

**参数：**
- `buffer` (Buffer) - 包含 varint 的缓冲区

**返回值：** number - 解码后的整数值

**示例：**
```javascript
const { encodeVarint, decodeVarint } = require('@protobuf-rs/core');

const buffer = encodeVarint(300);
const value = decodeVarint(buffer);
console.log(value); // 300
```

**错误处理：**
- 如果缓冲区为空或不完整，将抛出错误
- 如果 varint 超过 10 字节，将抛出错误

### ZigZag 操作

ZigZag 编码将有符号整数映射为无符号整数，使得绝对值较小的数（无论正负）都有较小的编码值。

**映射规则：**
- 0 → 0
- -1 → 1
- 1 → 2
- -2 → 3
- 2 → 4
- ...

#### `encodeZigzag(value: number): number`

使用 ZigZag 编码对有符号整数进行编码。

**参数：**
- `value` (number) - 要编码的有符号整数

**返回值：** number - ZigZag 编码的无符号值

**示例：**
```javascript
const { encodeZigzag } = require('@protobuf-rs/core');

console.log(encodeZigzag(0));   // 0
console.log(encodeZigzag(-1));  // 1
console.log(encodeZigzag(1));   // 2
console.log(encodeZigzag(-2));  // 3
console.log(encodeZigzag(2));   // 4
```

**使用场景：**
- 编码可能为负的小整数（如增量、偏移量）
- protobuf 的 sint32 和 sint64 类型
- 提高小负数的编码效率

#### `decodeZigzag(value: number): number`

将 ZigZag 编码的整数解码回有符号整数。

**参数：**
- `value` (number) - ZigZag 编码的值

**返回值：** number - 解码后的有符号整数

**示例：**
```javascript
const { encodeZigzag, decodeZigzag } = require('@protobuf-rs/core');

const encoded = encodeZigzag(-100);
const decoded = decodeZigzag(encoded);
console.log(decoded); // -100
```

### 字段标签操作

Protocol Buffer 消息中的每个字段都有一个标签，由字段编号和线路类型组成。

#### `encodeFieldTag(fieldNumber: number, wireType: number): Buffer`

编码 Protocol Buffer 字段标签。

**参数：**
- `fieldNumber` (number) - 字段编号（1 到 2^29-1，不包括 19000-19999）
- `wireType` (number) - 线路类型（0-5）

**返回值：** Buffer - 包含编码标签的缓冲区

**线路类型：**
| 值 | 类型 | 用途 |
|----|------|------|
| 0 | Varint | int32, int64, uint32, uint64, sint32, sint64, bool, enum |
| 1 | 64-bit | fixed64, sfixed64, double |
| 2 | Length-delimited | string, bytes, embedded messages, packed repeated fields |
| 3 | Start group | 已弃用 |
| 4 | End group | 已弃用 |
| 5 | 32-bit | fixed32, sfixed32, float |

**示例：**
```javascript
const { encodeFieldTag } = require('@protobuf-rs/core');

// 字段 1, 类型 varint (wire type 0)
const tag1 = encodeFieldTag(1, 0);
console.log(tag1); // <Buffer 08> (即 8)

// 字段 2, 类型 length-delimited (wire type 2)
const tag2 = encodeFieldTag(2, 2);
console.log(tag2); // <Buffer 12> (即 18)

// 字段 3, 类型 32-bit (wire type 5)
const tag3 = encodeFieldTag(3, 5);
console.log(tag3); // <Buffer 1d> (即 29)
```

**计算公式：** `tag = (field_number << 3) | wire_type`

**错误处理：**
- 字段编号必须 >= 1
- 字段编号不能在 19000-19999 范围内（protobuf 保留）
- 线路类型必须在 0-5 范围内

#### `decodeFieldTag(buffer: Buffer): Array<number>`

解码 Protocol Buffer 字段标签。

**参数：**
- `buffer` (Buffer) - 包含字段标签的缓冲区

**返回值：** [number, number] - 数组 `[fieldNumber, wireType]`

**示例：**
```javascript
const { encodeFieldTag, decodeFieldTag } = require('@protobuf-rs/core');

const tag = encodeFieldTag(5, 2);
const [fieldNumber, wireType] = decodeFieldTag(tag);
console.log(fieldNumber); // 5
console.log(wireType);    // 2
```

## Reader 类

高性能的二进制数据读取器，具有零拷贝优化。

### 构造函数

#### `new Reader(buffer: Buffer)`

创建一个新的 Reader 实例。

**参数：**
- `buffer` (Buffer) - 要读取的缓冲区

**示例：**
```javascript
const { Reader } = require('@protobuf-rs/core');

const buffer = Buffer.from([0x08, 0x96, 0x01]);
const reader = new Reader(buffer);
```

#### `Reader.create(buffer: Buffer): Reader`

静态工厂方法，创建一个新的 Reader 实例。

**参数：**
- `buffer` (Buffer) - 要读取的缓冲区

**返回值：** Reader - 新的 Reader 实例

**示例：**
```javascript
const reader = Reader.create(buffer);
```

### 属性

#### `reader.pos: number`

当前读取位置（只读）。

#### `reader.len: number`

缓冲区总长度（只读）。

### 整数读取方法

#### `reader.uint32(): number`

读取无符号 32 位整数（varint 编码）。

**返回值：** number - 0 到 2^32-1

**示例：**
```javascript
const value = reader.uint32();
```

#### `reader.int32(): number`

读取有符号 32 位整数（varint 编码）。

**返回值：** number - -2^31 到 2^31-1

#### `reader.sint32(): number`

读取有符号 32 位整数（ZigZag + varint 编码）。

**返回值：** number - -2^31 到 2^31-1

**使用场景：** 当值可能为负且绝对值较小时更高效

#### `reader.uint64(): number`

读取无符号 64 位整数（varint 编码）。

**返回值：** number - 0 到 2^53-1（JavaScript 安全整数范围）

**注意：** 超过 2^53 的值可能失去精度

#### `reader.int64(): number`

读取有符号 64 位整数（varint 编码）。

**返回值：** number - -2^53 到 2^53-1（JavaScript 安全整数范围）

#### `reader.sint64(): number`

读取有符号 64 位整数（ZigZag + varint 编码）。

**返回值：** number - -2^53 到 2^53-1（JavaScript 安全整数范围）

#### `reader.bool(): boolean`

读取布尔值。

**返回值：** boolean - true 或 false

**示例：**
```javascript
const active = reader.bool();
```

### 固定宽度类型

#### `reader.fixed32(): number`

读取固定 32 位无符号整数（小端序）。

**返回值：** number - 0 到 2^32-1

#### `reader.sfixed32(): number`

读取固定 32 位有符号整数（小端序）。

**返回值：** number - -2^31 到 2^31-1

#### `reader.fixed64(): number`

读取固定 64 位无符号整数（小端序）。

**返回值：** number - 0 到 2^53-1

#### `reader.sfixed64(): number`

读取固定 64 位有符号整数（小端序）。

**返回值：** number - -2^53 到 2^53-1

#### `reader.float(): number`

读取 32 位浮点数（IEEE 754）。

**返回值：** number

**示例：**
```javascript
const temperature = reader.float();
```

#### `reader.double(): number`

读取 64 位双精度浮点数（IEEE 754）。

**返回值：** number

### 字节和字符串

#### `reader.bytes(): Buffer`

读取长度限定的字节数组。

**返回值：** Buffer - 字节数据

**示例：**
```javascript
const data = reader.bytes();
```

#### `reader.string(): string`

读取长度限定的 UTF-8 字符串。

**返回值：** string

**示例：**
```javascript
const name = reader.string();
console.log(name); // "你好，世界！"
```

### 导航方法

#### `reader.skip(length: number): Reader`

跳过指定字节数。

**参数：**
- `length` (number) - 要跳过的字节数

**返回值：** Reader - 返回 this 以支持链式调用

**示例：**
```javascript
reader.skip(10);
```

#### `reader.skipType(wireType: number): Reader`

根据线路类型跳过一个字段。

**参数：**
- `wireType` (number) - 线路类型（0-5）

**返回值：** Reader - 返回 this 以支持链式调用

**示例：**
```javascript
const tag = reader.uint32();
const wireType = tag & 7;
if (fieldNumber !== expectedField) {
  reader.skipType(wireType);
}
```

**跳过的字节数：**
- Wire type 0 (varint): 读取并跳过 varint
- Wire type 1 (64-bit): 跳过 8 字节
- Wire type 2 (length-delimited): 读取长度并跳过该长度的字节
- Wire type 5 (32-bit): 跳过 4 字节

#### `reader.reset(): Reader`

重置读取位置到缓冲区开始处。

**返回值：** Reader - 返回 this 以支持链式调用

**示例：**
```javascript
reader.reset();
```

### 使用示例

```javascript
const { Reader, Writer } = require('@protobuf-rs/core');

// 创建一个包含多个字段的消息
const writer = Writer.create();
writer.uint32((1 << 3) | 0);  // 字段 1, varint
writer.uint32(42);
writer.uint32((2 << 3) | 2);  // 字段 2, string
writer.string("你好");
writer.uint32((3 << 3) | 0);  // 字段 3, bool
writer.bool(true);

const buffer = writer.finish();

// 读取消息
const reader = Reader.create(buffer);
const message = {};

while (reader.pos < reader.len) {
  const tag = reader.uint32();
  const fieldNumber = tag >>> 3;
  const wireType = tag & 7;
  
  switch (fieldNumber) {
    case 1:
      message.id = reader.uint32();
      break;
    case 2:
      message.name = reader.string();
      break;
    case 3:
      message.active = reader.bool();
      break;
    default:
      reader.skipType(wireType);
  }
}

console.log(message); // { id: 42, name: '你好', active: true }
```

## Writer 类

高性能的二进制数据写入器，具有缓冲区优化。

### 构造函数

#### `new Writer()`

创建一个新的 Writer 实例。

**示例：**
```javascript
const { Writer } = require('@protobuf-rs/core');

const writer = new Writer();
```

#### `Writer.create(): Writer`

静态工厂方法，创建一个新的 Writer 实例。

**返回值：** Writer - 新的 Writer 实例

**示例：**
```javascript
const writer = Writer.create();
```

#### `Writer.withCapacity(capacity: number): Writer`

创建一个具有预分配容量的 Writer 实例。

**参数：**
- `capacity` (number) - 初始缓冲区容量（字节）

**返回值：** Writer - 新的 Writer 实例

**使用场景：** 当已知消息大小时，预分配可以提高性能

**示例：**
```javascript
const writer = Writer.withCapacity(1024);
```

### 整数写入方法

#### `writer.uint32(value: number): Writer`

写入无符号 32 位整数（varint 编码）。

**参数：**
- `value` (number) - 0 到 2^32-1

**返回值：** Writer - 返回 this 以支持链式调用

**示例：**
```javascript
writer.uint32(42);
```

#### `writer.int32(value: number): Writer`

写入有符号 32 位整数（varint 编码）。

**参数：**
- `value` (number) - -2^31 到 2^31-1

**返回值：** Writer

#### `writer.sint32(value: number): Writer`

写入有符号 32 位整数（ZigZag + varint 编码）。

**参数：**
- `value` (number) - -2^31 到 2^31-1

**返回值：** Writer

**使用场景：** 当值可能为负且绝对值较小时更高效

#### `writer.uint64(value: number): Writer`

写入无符号 64 位整数（varint 编码）。

**参数：**
- `value` (number) - 0 到 2^53-1

**返回值：** Writer

#### `writer.int64(value: number): Writer`

写入有符号 64 位整数（varint 编码）。

**参数：**
- `value` (number) - -2^53 到 2^53-1

**返回值：** Writer

#### `writer.sint64(value: number): Writer`

写入有符号 64 位整数（ZigZag + varint 编码）。

**参数：**
- `value` (number) - -2^53 到 2^53-1

**返回值：** Writer

#### `writer.bool(value: boolean): Writer`

写入布尔值。

**参数：**
- `value` (boolean) - true 或 false

**返回值：** Writer

**示例：**
```javascript
writer.bool(true);
```

### 固定宽度类型

#### `writer.fixed32(value: number): Writer`

写入固定 32 位无符号整数（小端序）。

**参数：**
- `value` (number) - 0 到 2^32-1

**返回值：** Writer

#### `writer.sfixed32(value: number): Writer`

写入固定 32 位有符号整数（小端序）。

**参数：**
- `value` (number) - -2^31 到 2^31-1

**返回值：** Writer

#### `writer.fixed64(value: number): Writer`

写入固定 64 位无符号整数（小端序）。

**参数：**
- `value` (number) - 0 到 2^53-1

**返回值：** Writer

#### `writer.sfixed64(value: number): Writer`

写入固定 64 位有符号整数（小端序）。

**参数：**
- `value` (number) - -2^53 到 2^53-1

**返回值：** Writer

#### `writer.float(value: number): Writer`

写入 32 位浮点数（IEEE 754）。

**参数：**
- `value` (number)

**返回值：** Writer

**示例：**
```javascript
writer.float(3.14);
```

#### `writer.double(value: number): Writer`

写入 64 位双精度浮点数（IEEE 754）。

**参数：**
- `value` (number)

**返回值：** Writer

### 字节和字符串

#### `writer.bytes(value: Buffer | Uint8Array): Writer`

写入长度限定的字节数组。

**参数：**
- `value` (Buffer | Uint8Array) - 字节数据

**返回值：** Writer

**示例：**
```javascript
const data = Buffer.from([1, 2, 3, 4, 5]);
writer.bytes(data);
```

#### `writer.string(value: string): Writer`

写入长度限定的 UTF-8 字符串。

**参数：**
- `value` (string)

**返回值：** Writer

**示例：**
```javascript
writer.string("你好，世界！");
```

### 长度限定字段

#### `writer.fork(): Writer`

为长度限定字段创建一个分支。

**返回值：** Writer - 返回 this 以支持链式调用

**使用场景：** 写入嵌套消息时需要先写入长度

**示例：**
```javascript
writer.uint32((1 << 3) | 2);  // 字段 1, length-delimited
writer.fork();
// 写入嵌套消息的内容
writer.uint32((1 << 3) | 0);
writer.uint32(42);
writer.ldelim();
```

#### `writer.ldelim(): Writer`

完成长度限定字段。

**返回值：** Writer - 返回 this 以支持链式调用

**说明：** 计算自 fork() 以来写入的字节数，并在适当位置写入长度

### 工具方法

#### `writer.reset(): Writer`

重置 writer，清除所有数据。

**返回值：** Writer - 返回 this 以支持链式调用

**使用场景：** 重用 writer 对象以避免重复分配

**示例：**
```javascript
const writer = Writer.create();

// 第一条消息
writer.uint32(100);
const buffer1 = writer.finish();

// 重用 writer
writer.reset();
writer.uint32(200);
const buffer2 = writer.finish();
```

#### `writer.finish(): Buffer`

完成写入并返回结果缓冲区。

**返回值：** Buffer - 包含所有写入数据的缓冲区

**示例：**
```javascript
const buffer = writer.finish();
```

### 使用示例

```javascript
const { Writer } = require('@protobuf-rs/core');

// 编写一个简单的消息
// message User {
//   uint32 id = 1;
//   string name = 2;
//   bool active = 3;
// }

function encodeUser(user) {
  const writer = Writer.create();
  
  // 字段 1: id
  writer.uint32((1 << 3) | 0);
  writer.uint32(user.id);
  
  // 字段 2: name
  writer.uint32((2 << 3) | 2);
  writer.string(user.name);
  
  // 字段 3: active
  writer.uint32((3 << 3) | 0);
  writer.bool(user.active);
  
  return writer.finish();
}

const buffer = encodeUser({
  id: 42,
  name: "张三",
  active: true
});
```

## 批量操作

高性能的批量数据处理，支持 SIMD 优化和并行处理。

### `encodeVarintBatchSimd(values: number[]): Buffer`

使用 SIMD 优化批量编码 varint。

**参数：**
- `values` (number[]) - 要编码的整数数组

**返回值：** Buffer - 包含所有编码 varint 的缓冲区

**性能：** 比逐个编码快 10-15 倍

**示例：**
```javascript
const { encodeVarintBatchSimd } = require('@protobuf-rs/core');

const values = Array.from({ length: 1000 }, (_, i) => i);
const encoded = encodeVarintBatchSimd(values);
```

### `decodeVarintBatchSimd(buffer: Buffer): number[]`

使用 SIMD 优化批量解码 varint。

**参数：**
- `buffer` (Buffer) - 包含多个 varint 的缓冲区

**返回值：** number[] - 解码后的整数数组

**示例：**
```javascript
const { encodeVarintBatchSimd, decodeVarintBatchSimd } = require('@protobuf-rs/core');

const values = [1, 2, 3, 4, 5];
const encoded = encodeVarintBatchSimd(values);
const decoded = decodeVarintBatchSimd(encoded);
```

### `processU32BatchParallel(values: number[], chunkSize: number): ProcessedBatch`

并行处理大批量 uint32 数据。

**参数：**
- `values` (number[]) - 要处理的整数数组
- `chunkSize` (number) - 每个并行任务的块大小

**返回值：** ProcessedBatch - 处理后的结果

**性能：** 在多核 CPU 上可获得 3-4 倍提速

**使用场景：** 大数据集的批量处理（> 10,000 条目）

**示例：**
```javascript
const { processU32BatchParallel } = require('@protobuf-rs/core');

const largeDataset = Array.from({ length: 100000 }, (_, i) => i);
const result = processU32BatchParallel(largeDataset, 1000);
```

## 集成 API

### protobufjs-compat

完整的 protobuf.js 兼容层。

```javascript
const protobuf = require('@protobuf-rs/core/protobufjs-compat');

// 所有 protobuf.js API 都可用
const Root = protobuf.Root;
const Type = protobuf.Type;
const Reader = protobuf.Reader;
const Writer = protobuf.Writer;
```

### protobufjs-adapter

Reader 和 Writer 的适配器。

```javascript
const { Reader, Writer, isNativeAvailable } = require('@protobuf-rs/core/integration/protobufjs-adapter');

console.log('Native available:', isNativeAvailable());
```

#### `isNativeAvailable(): boolean`

检查原生 Rust 实现是否可用。

**返回值：** boolean - 如果原生模块已加载则为 true

#### `getImplementationType(): string`

获取当前实现类型。

**返回值：** string - "native" 或 "javascript"

## 性能监控

内置的性能监控工具。

```javascript
const PerformanceMonitor = require('@protobuf-rs/core/integration/performance-monitor');

const monitor = new PerformanceMonitor('My Benchmark');

// 记录操作
const start = Date.now();
// ... 你的代码 ...
monitor.record('encode', Date.now() - start);

// 生成报告
monitor.report();
```

### `PerformanceMonitor`

#### `constructor(name: string)`

创建一个新的性能监控器。

**参数：**
- `name` (string) - 监控器名称

#### `record(operation: string, duration: number): void`

记录一次操作。

**参数：**
- `operation` (string) - 操作名称
- `duration` (number) - 持续时间（毫秒）

#### `report(): void`

生成并打印性能报告。

**输出：**
- 每个操作的统计信息
- 平均值、最小值、最大值
- P50、P95、P99 百分位数

## 错误处理

所有 API 在遇到无效输入时都会抛出错误。建议使用 try-catch 处理：

```javascript
try {
  const reader = Reader.create(buffer);
  const value = reader.uint32();
} catch (error) {
  console.error('解码失败:', error.message);
}
```

**常见错误：**
- "Invalid field number" - 字段编号无效
- "Invalid wire type" - 线路类型无效
- "Buffer too short" - 缓冲区数据不足
- "Varint overflow" - Varint 值过大

## 类型定义

完整的 TypeScript 类型定义在 `index.d.ts` 中提供。

```typescript
import { Reader, Writer, encodeVarint, decodeVarint } from '@protobuf-rs/core';

const encoded: Buffer = encodeVarint(300);
const decoded: number = decodeVarint(encoded);

const writer: Writer = Writer.create();
writer.uint32(100);
const buffer: Buffer = writer.finish();

const reader: Reader = Reader.create(buffer);
const value: number = reader.uint32();
```

## 性能最佳实践

1. **重用 Writer 对象**
   ```javascript
   const writer = Writer.create();
   for (const item of items) {
     // 编码
     writer.reset();  // 重用
   }
   ```

2. **预分配容量**
   ```javascript
   const writer = Writer.withCapacity(1024);
   ```

3. **使用批量操作**
   ```javascript
   // 好：批量编码
   const encoded = encodeVarintBatchSimd(values);
   
   // 不好：逐个编码
   for (const value of values) {
     encodeVarint(value);
   }
   ```

4. **避免不必要的字符串转换**
   ```javascript
   // 好：直接使用 bytes
   writer.bytes(buffer);
   
   // 不好：转换为字符串再转回来
   writer.string(buffer.toString());
   ```

## 另请参阅

- [性能报告](PERFORMANCE_REPORT.md) - 详细的性能分析
- [集成指南](INTEGRATION_GUIDE.md) - 与 protobuf.js 的集成
- [兼容性报告](COMPATIBILITY_REPORT.md) - API 兼容性详情
- [示例](../../examples/) - 实际使用示例
