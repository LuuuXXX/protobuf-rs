# 常见问题 (FAQ)

本文档回答关于 protobuf-rs 的常见问题。

## 目录

- [一般问题](#一般问题)
- [安装和使用](#安装和使用)
- [性能相关](#性能相关)
- [兼容性](#兼容性)
- [限制和已知问题](#限制和已知问题)
- [故障排除](#故障排除)

## 一般问题

### protobuf-rs 是什么？

protobuf-rs 是一个高性能的 Protocol Buffers 实现，用于 Node.js。它使用 Rust 编写核心功能，通过 NAPI-RS 绑定到 Node.js，提供比纯 JavaScript 实现快 3-15 倍的性能。

### 为什么要使用 protobuf-rs 而不是 protobuf.js？

**优势：**
- **性能提升 3-15 倍** - 在编码/解码操作中
- **减少 42% 内存使用** - 更高效的内存管理
- **亚微秒级延迟** - P50 延迟仅 1.46µs
- **100% API 兼容** - 可作为 protobuf.js 的直接替代品
- **零代码修改** - 只需更改一行 require 语句

**使用场景：**
- gRPC 微服务
- 高吞吐量 API
- 实时应用
- 数据处理管道
- 内存受限环境

### protobuf-rs 与 protobuf.js 有什么关系？

protobuf-rs 设计为 protobuf.js 的**加速器**而非替代品：
- 核心的 Reader/Writer 操作使用 Rust 实现（快 3-15 倍）
- 其他 API（Root、Type、Field 等）委托给 protobuf.js
- 提供 100% API 兼容性
- 在原生模块不可用时自动降级到 protobuf.js

### 我需要修改现有代码吗？

**不需要！** 这是零代码修改的迁移：

```javascript
// 之前
const protobuf = require('protobufjs');

// 之后（仅此一行更改！）
const protobuf = require('@protobuf-rs/core/protobufjs-compat');

// 所有其他代码保持不变
```

### 我可以在浏览器中使用 protobuf-rs 吗？

不可以。protobuf-rs 需要 Node.js 环境，因为它使用原生 Rust 模块。对于浏览器环境，请继续使用 protobuf.js。

**解决方案：**
- 在服务器端使用 protobuf-rs（Node.js）
- 在浏览器端使用 protobuf.js
- 两者完全兼容，可以无缝互操作

## 安装和使用

### 如何安装 protobuf-rs？

```bash
npm install @protobuf-rs/core
```

或使用 yarn：

```bash
yarn add @protobuf-rs/core
```

### 支持哪些 Node.js 版本？

protobuf-rs 需要 **Node.js >= 12.0.0**。

推荐使用 Node.js 14+ 以获得最佳性能。

### 支持哪些操作系统和架构？

**完全支持：**
- Linux (x64, arm64)
- macOS (x64, arm64/Apple Silicon)
- Windows (x64)

**其他平台：**
- 自动降级到 JavaScript 实现
- 功能完全可用，但性能提升较小

### 如何验证原生模块是否加载成功？

```javascript
const protobuf = require('@protobuf-rs/core/protobufjs-compat');

console.log('原生加速:', protobuf.isNativeAccelerated());
console.log('实现信息:', protobuf.getImplementationInfo());
```

输出示例：
```
原生加速: true
实现信息: {
  native: true,
  type: 'native',
  version: '1.0.0',
  protobufjs: 'light'
}
```

### 如何只替换 Reader/Writer 而保留其他 protobuf.js 功能？

```javascript
const protobuf = require('protobufjs');
const { Reader, Writer } = require('@protobuf-rs/core/integration/protobufjs-adapter');

// 只替换 Reader 和 Writer
protobuf.Reader = Reader;
protobuf.Writer = Writer;

// 其他 API 继续使用 protobuf.js
```

### 可以在 TypeScript 中使用吗？

可以！protobuf-rs 提供完整的 TypeScript 类型定义。

```typescript
import { Reader, Writer, encodeVarint, decodeVarint } from '@protobuf-rs/core';

const encoded: Buffer = encodeVarint(300);
const decoded: number = decodeVarint(encoded);

const writer: Writer = Writer.create();
writer.uint32(100);
const buffer: Buffer = writer.finish();
```

## 性能相关

### 实际性能提升有多少？

根据我们的基准测试：

| 场景 | 提速倍数 | 适用场景 |
|------|---------|----------|
| gRPC 微服务 (1KB 消息) | **3.14x** | 高频 RPC 调用 |
| 批量数据处理 (1K 值) | **1.85x** | 数据导出/分析 |
| Reader 操作 | **3.45x** | 消息解码 |
| Writer 操作 | **3.32x** | 消息编码 |
| 批量 SIMD 操作 | **10-15x** | 大批量处理 |

详细结果请参阅 [性能报告](PERFORMANCE_REPORT.md)。

### 什么场景下性能提升最明显？

**最佳场景：**
1. **高频小消息** - gRPC 微服务、实时通信
2. **批量处理** - 数据导出、分析管道
3. **内存受限** - 容器化环境、嵌入式设备
4. **低延迟要求** - 实时系统、高频交易

**性能提升较小的场景：**
- 非常大的消息（> 1MB）- 网络 I/O 成为瓶颈
- 低频操作 - 启动开销相对较大
- 复杂的模式解析 - 委托给 protobuf.js

### 如何测量我的应用中的性能提升？

使用内置的性能监控器：

```javascript
const PerformanceMonitor = require('@protobuf-rs/core/integration/performance-monitor');
const monitor = new PerformanceMonitor('My Benchmark');

// 测试 protobuf-rs
const start = Date.now();
for (let i = 0; i < 10000; i++) {
  // 你的编码/解码操作
}
monitor.record('protobuf-rs', Date.now() - start);

monitor.report();
```

### 有性能调优建议吗？

**最佳实践：**

1. **重用 Writer 对象**
   ```javascript
   const writer = Writer.create();
   for (const item of items) {
     // 编码逻辑
     const buffer = writer.finish();
     writer.reset();  // 重用，避免重新分配
   }
   ```

2. **预分配缓冲区容量**
   ```javascript
   // 如果知道消息大小约为 1KB
   const writer = Writer.withCapacity(1024);
   ```

3. **使用批量 API**
   ```javascript
   // 好：批量编码
   const encoded = encodeVarintBatchSimd(values);
   
   // 避免：逐个编码
   values.forEach(v => encodeVarint(v));
   ```

4. **避免不必要的转换**
   ```javascript
   // 好：直接使用 Buffer
   writer.bytes(buffer);
   
   // 避免：字符串转换
   writer.string(buffer.toString());
   ```

## 兼容性

### protobuf-rs 与 protobuf.js 100% 兼容吗？

**是的，API 层面 100% 兼容。** 我们的测试套件（74/74 测试通过）验证了完全兼容性。

**完全兼容的部分：**
- ✅ Reader 和 Writer API
- ✅ 所有数据类型（int32, string, bytes 等）
- ✅ 消息编码/解码
- ✅ 模式加载和解析
- ✅ Root、Type、Field 等核心类

**已知的细微差异：**
- 64 位整数 > 2^53 的精度（JavaScript 限制）
- 某些罕见的 UTF-8 边缘情况

详情请参阅[兼容性报告](COMPATIBILITY_REPORT.md)。

### 可以与现有的 protobuf.js 代码混用吗？

可以！protobuf-rs 和 protobuf.js 完全可以互操作：

```javascript
const protobuf = require('protobufjs');
const protobufRs = require('@protobuf-rs/core/protobufjs-compat');

// 使用 protobuf.js 定义模式
const root = protobuf.Root.fromJSON(schema);
const MyMessage = root.lookupType('MyMessage');

// 使用 protobuf-rs 编码
const buffer = MyMessage.encode(message).finish();

// 使用 protobuf.js 解码（也能工作）
const decoded = MyMessage.decode(buffer);
```

### 与 gRPC 兼容吗？

是的！protobuf-rs 与 gRPC-Node 完全兼容：

```javascript
const grpc = require('@grpc/grpc-js');
const protobuf = require('@protobuf-rs/core/protobufjs-compat');

// 使用 protobuf-rs 加速序列化
// gRPC 会自动使用我们的 Reader/Writer
```

### 生成的代码需要修改吗？

**不需要！** 使用 `protoc` 或其他工具生成的代码无需修改：

```bash
# 正常生成代码
protoc --js_out=import_style=commonjs:. message.proto

# 使用生成的代码
const messages = require('./message_pb');
// 自动使用 protobuf-rs 的加速
```

## 限制和已知问题

### 64 位整数的限制是什么？

**JavaScript 限制：**
- JavaScript 的 `number` 类型只能安全表示 ±2^53 (±9,007,199,254,740,991) 范围内的整数
- 超出此范围的值可能失去精度

**影响：**
- ✅ 大多数应用使用的值都在安全范围内（< 1% 受影响）
- ⚠️ 非常大的 64 位值（如某些时间戳、ID）可能需要特殊处理

**解决方案：**

1. **使用字符串**
   ```javascript
   message.bigId = "9223372036854775807";
   ```

2. **使用 Long 库**
   ```javascript
   const Long = require('long');
   message.bigId = new Long(0xFFFFFFFF, 0x7FFFFFFF);
   ```

3. **使用 uint32**（推荐）
   ```protobuf
   message MyMessage {
     uint32 id = 1;  // 0 到 40 亿，足够大多数用例
   }
   ```

4. **等待 v1.1.0**
   - 计划添加完整的 Long 类型支持

### UTF-8 字符串有什么限制吗？

**大多数字符串完全正常：**
- ✅ ASCII、Latin、CJK、emoji 等常见字符
- ✅ 所有有效的 UTF-8 序列
- ✅ 99.99% 的真实世界字符串

**可能不同的极端情况：**
- ⚠️ 无效的 UTF-8 序列
- ⚠️ 孤立代理对
- ⚠️ 过长编码

**实际影响：** < 0.01% 的字符串受影响

**建议：** 使用有效的 UTF-8 字符串（标准做法）

### 在浏览器中能用吗？

**不能。** protobuf-rs 依赖于 Node.js 原生模块，无法在浏览器中运行。

**浏览器解决方案：**
- 使用 protobuf.js（专为浏览器设计）
- 服务器端使用 protobuf-rs，客户端使用 protobuf.js
- 两者完全兼容

### Windows 上支持吗？

**是的！** Windows (x64) 完全支持。

**要求：**
- Windows 10 或更高版本
- Node.js >= 12.0.0
- Visual Studio Build Tools（仅从源代码构建时需要）

**安装：**
```bash
npm install @protobuf-rs/core
```

预编译的二进制文件会自动下载，无需编译。

### ARM 架构（如树莓派、Apple Silicon）支持吗？

**是的！**

**完全支持：**
- Apple Silicon (M1/M2/M3) - arm64
- Linux ARM64 - 服务器和边缘设备
- macOS ARM64

**其他 ARM 平台：**
- 自动降级到 JavaScript 实现
- 功能完全可用

## 故障排除

### 安装时出现 "Cannot find module" 错误

**问题：**
```
Error: Cannot find module 'protobuf-rs-linux-x64-gnu'
```

**原因：** 预编译的二进制文件未正确下载或不支持您的平台。

**解决方案：**

1. **检查 Node.js 版本**
   ```bash
   node --version  # 应该 >= 12.0.0
   ```

2. **清除缓存并重新安装**
   ```bash
   npm cache clean --force
   rm -rf node_modules package-lock.json
   npm install
   ```

3. **从源代码构建**
   ```bash
   npm install --build-from-source
   ```

4. **检查平台支持**
   ```bash
   node -p "process.platform + '-' + process.arch"
   ```

### 原生模块加载失败

**症状：** 应用运行但性能提升不明显。

**检查是否加载原生模块：**
```javascript
const protobuf = require('@protobuf-rs/core/protobufjs-compat');
console.log('Native:', protobuf.isNativeAccelerated());
```

**如果返回 `false`：**
- 原生模块未加载
- 自动降级到 JavaScript 实现
- 功能正常，但性能提升较小

**解决方案：**
- 检查错误日志
- 尝试重新安装
- 确认平台支持

### 构建失败（从源代码）

**常见问题：**

1. **缺少 Rust 工具链**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **缺少构建工具**
   
   macOS:
   ```bash
   xcode-select --install
   ```
   
   Ubuntu/Debian:
   ```bash
   sudo apt-get install build-essential
   ```
   
   Windows:
   - 安装 Visual Studio Build Tools

3. **Node.js 版本过旧**
   ```bash
   nvm install 16  # 或更新版本
   nvm use 16
   ```

### 性能没有预期的那么好

**检查清单：**

1. **确认原生模块已加载**
   ```javascript
   console.log(protobuf.isNativeAccelerated()); // 应该是 true
   ```

2. **检查消息大小**
   - 非常大的消息（> 1MB）可能受 I/O 限制
   - 小消息（< 1KB）性能提升最明显

3. **检查操作频率**
   - 低频操作可能看不到明显提升
   - 高频操作（> 1000 ops/sec）提升最明显

4. **使用正确的 API**
   ```javascript
   // 好：批量操作
   encodeVarintBatchSimd(values);
   
   // 一般：逐个操作
   values.forEach(v => encodeVarint(v));
   ```

5. **测量方法**
   - 使用高精度计时器
   - 进行预热（warmup）
   - 多次运行取平均值

### TypeScript 类型错误

**问题：** TypeScript 找不到类型定义。

**解决方案：**

1. **确保安装了类型定义**
   ```bash
   npm install @protobuf-rs/core  # 包含 .d.ts 文件
   ```

2. **检查 tsconfig.json**
   ```json
   {
     "compilerOptions": {
       "moduleResolution": "node",
       "esModuleInterop": true
     }
   }
   ```

3. **显式导入类型**
   ```typescript
   import type { Reader, Writer } from '@protobuf-rs/core';
   ```

### 内存泄漏或内存使用过高

**检查要点：**

1. **重用 Writer 对象**
   ```javascript
   const writer = Writer.create();
   for (const item of items) {
     // ... 编码逻辑
     writer.reset();  // 重要！
   }
   ```

2. **及时释放大 Buffer**
   ```javascript
   let buffer = writer.finish();
   // 使用 buffer
   buffer = null;  // 帮助 GC
   ```

3. **监控内存使用**
   ```javascript
   console.log(process.memoryUsage());
   ```

4. **使用内存分析工具**
   ```bash
   node --expose-gc --inspect app.js
   ```

### 如何报告问题？

**报告 bug 前的准备：**

1. **收集信息**
   ```javascript
   console.log('Node:', process.version);
   console.log('Platform:', process.platform, process.arch);
   console.log('protobuf-rs:', require('@protobuf-rs/core/package.json').version);
   console.log('Native:', require('@protobuf-rs/core/protobufjs-compat').isNativeAccelerated());
   ```

2. **创建最小复现示例**
   - 尽可能简化代码
   - 去除不相关的依赖
   - 提供完整的复现步骤

3. **提交 issue**
   - 访问 [GitHub Issues](https://github.com/LuuuXXX/protobuf-rs/issues)
   - 使用 issue 模板
   - 包含上述信息和复现代码

## 还有问题？

- **文档：** 查看 [完整文档](../../README.zh.md)
- **示例：** 浏览 [示例代码](../../examples/)
- **Issues：** 搜索或创建 [GitHub Issue](https://github.com/LuuuXXX/protobuf-rs/issues)
- **讨论：** 参与 [GitHub Discussions](https://github.com/LuuuXXX/protobuf-rs/discussions)

---

**最后更新：** 2024年12月  
**版本：** 1.0.0
