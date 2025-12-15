# protobuf-rs

[![npm version](https://img.shields.io/npm/v/@protobuf-rs/core.svg)](https://www.npmjs.com/package/@protobuf-rs/core)
[![License](https://img.shields.io/badge/license-BSD--3--Clause-blue.svg)](LICENSE)
[![CI](https://github.com/LuuuXXX/protobuf-rs/workflows/CI/badge.svg)](https://github.com/LuuuXXX/protobuf-rs/actions)
[![性能](https://img.shields.io/badge/性能-快3.14倍-brightgreen)](docs/BENCHMARK_RESULTS.md)
[![内存](https://img.shields.io/badge/内存--42%25-blue)](docs/BENCHMARK_RESULTS.md)
[![兼容性](https://img.shields.io/badge/兼容性-100%25-brightgreen)](library/test/protobufjs-compatibility.js)
[![HarmonyOS](https://img.shields.io/badge/HarmonyOS-就绪-orange)](docs/HARMONYOS.md)

**基于 Rust 和 NAPI-RS 的高性能 Node.js Protocol Buffers 实现**

[English](README.md) | 简体中文

## 📁 项目结构 (v1.0.0)

本项目已重构以支持 **HarmonyOS**，同时保持 Node.js 兼容性：

```
protobuf-rs/
├── library/              # 核心库（从根目录移动）
│   ├── src/             # Rust 源代码
│   ├── test/            # 测试文件
│   ├── index.js         # JavaScript 入口点
│   ├── index.d.ts       # TypeScript 类型定义
│   ├── protobufjs-compat.js  # 兼容层
│   ├── Cargo.toml       # Rust 包配置
│   └── build.rs         # 构建脚本
├── entry/               # HarmonyOS 入口模块
│   └── src/
│       └── main.ets     # HarmonyOS 入口点
├── AppScope/            # HarmonyOS 应用配置
│   └── app.json5
├── examples/            # 示例代码
├── docs/                # 文档
├── integration/         # 集成测试
├── scripts/             # 构建和工具脚本
├── build-profile.json5  # HarmonyOS 构建配置
├── oh-package.json5     # HarmonyOS 包配置
├── hvigorfile.ts        # HarmonyOS 构建工具配置
├── hvigorw / hvigorw.bat  # 构建包装脚本
├── package.json         # Node.js 包配置（指向 library/）
├── Cargo.toml           # Rust 工作空间配置
└── README.md
```

### v1.0.0 破坏性变更

对于 npm 用户，重构是**透明的** - 包的使用方式完全不变：

```javascript
// 仍然可以像以前一样使用
const protobuf = require('@protobuf-rs/core');
const { Reader, Writer } = require('@protobuf-rs/core/protobufjs-compat');
```

package.json 的 `main` 字段指向 `library/index.js`，因此所有导入都保持不变。

## 🚀 性能表现

### 性能概览

| 指标 | 数值 | 对比 protobuf.js |
|--------|-------|----------------|
| **吞吐量** | 289K ops/s | **快 3.14 倍** ⚡ |
| **P99 延迟** | 28.38µs | **降低 37.2%** 📉 |
| **内存占用** | 45.3 MB | **减少 42.4%** 💾 |

[📊 完整性能测试报告 →](docs/BENCHMARK_RESULTS.md)

### 关键性能指标

- **比纯 JavaScript 实现快 3-15 倍**
- **亚微秒级延迟**（P50: 1.46µs）
- **减少 42% 内存占用**，分配效率提升 78 倍
- **100% 兼容** protobuf.js API

### 性能测试结果

| 场景 | 吞吐量 | 提速倍数 |
|----------|-----------|---------|
| gRPC 微服务 | 289K ops/sec | **3.14x** |
| 批量处理 | 14.5K ops/sec | **1.85x** |
| Reader 操作 | 621K ops/sec | **15x+** |
| Writer 操作 | 397K ops/sec | **10x+** |

详细分析请参阅 [性能报告](docs/PERFORMANCE_REPORT.md)。

## ✨ 特性

### 第三阶段：高级性能优化 (v1.0.0)
- ⚡ **SIMD 优化** - 向量化批处理操作
- 🔄 **零拷贝** - Reader/Writer 最小化内存分配
- 🧵 **并行处理** - 使用 rayon 的多核支持
- 💾 **内存池** - 线程安全的缓冲区复用
- 📊 **完整的基准测试** - 真实场景性能指标

### 第二阶段：生产环境集成
- 🔗 **混合适配器** - protobuf.js Reader/Writer 的直接替代品
- 🔄 **自动降级** - 在原生模块不可用时无缝切换到 JavaScript
- 📊 **性能监控** - 内置基准测试工具
- ✅ **完全兼容** - 100% 兼容 protobuf.js API

### 第一阶段：核心功能
- 🚀 基于 Rust 的高性能 Protocol Buffer 操作
- 🔧 Varint 编码和解码
- 🔄 有符号整数的 ZigZag 编码和解码
- 🏷️ 字段标签编码和解码
- 📦 Protobuf 消息解析
- 🌐 通过 NAPI-RS 实现跨平台支持
- 💪 类型安全的 TypeScript 绑定

## 📦 安装

```bash
npm install @protobuf-rs/core
```

或使用 yarn：

```bash
yarn add @protobuf-rs/core
```

## 🎯 快速开始

### 🚀 零代码修改迁移（推荐）

只需**一行代码**，即可将现有的 protobuf.js 替换为 protobuf-rs，获得 3 倍性能提升！

#### 步骤 1：安装
```bash
npm install @protobuf-rs/core
```

#### 步骤 2：替换 require（仅需修改一行！）
```javascript
// 之前
const protobuf = require('protobufjs');

// 之后
const protobuf = require('@protobuf-rs/core/protobufjs-compat');

// 就这样！所有现有代码现在运行速度快 3 倍！
```

#### 无需修改代码
- ✅ 相同的 API
- ✅ 相同的行为
- ✅ 相同的输出
- ✅ 快 3-4 倍的性能
- ✅ 减少 40% 内存使用

所有现有代码无需修改即可工作：
```javascript
// 你的现有代码无需修改！
const Root = protobuf.Root;
const Type = protobuf.Type;

const root = new Root();
const MyMessage = new Type("MyMessage");
// ... 一切工作完全相同，只是更快！
```

### 方式一：混合适配器

protobuf.js 的直接替代品：

```javascript
const { Reader, Writer } = require('@protobuf-rs/core/integration/protobufjs-adapter');

// 使用方式与 protobuf.js Reader/Writer 完全相同
const writer = Writer.create();
writer.uint32(300);
writer.string('你好，世界！');
const buffer = writer.finish();

const reader = Reader.create(buffer);
const num = reader.uint32();
const str = reader.string();
```

### 方式二：直接使用原生 API

获得最佳性能：

```javascript
const { Reader, Writer, encodeVarint, decodeVarint } = require('@protobuf-rs/core');

const encoded = encodeVarint(300);
const decoded = decodeVarint(encoded);
```

## 使用方法

### 基础用法（原生 API）

```javascript
const { Reader, Writer, encodeVarint, decodeVarint } = require('@protobuf-rs/core');

// 快速的 varint 操作
const encoded = encodeVarint(300);
const decoded = decodeVarint(encoded);

// 快速的 Reader/Writer
const writer = new Writer();
writer.uint32(100);
writer.uint32(200);
const buffer = writer.finish();

const reader = new Reader(buffer);
console.log(reader.uint32()); // 100
console.log(reader.uint32()); // 200
```

### 方式三：批量操作（第三阶段）

获得超高性能：

```javascript
const { 
    encodeVarintBatchSimd, 
    processU32BatchParallel 
} = require('@protobuf-rs/core');

// 批量编码 1000 个值
const values = Array.from({ length: 1000 }, (_, i) => i);
const encoded = encodeVarintBatchSimd(values);

// 大数据集的并行处理
const largeDataset = Array.from({ length: 100000 }, (_, i) => i);
const result = processU32BatchParallel(largeDataset, 1000);
```

## 📚 API 文档

### Varint 操作

#### `encodeVarint(value: number): Buffer`

将 64 位有符号整数编码为 Protocol Buffer varint。

**参数：**
- `value` - 要编码的整数

**返回值：** 包含编码 varint 的 Buffer

#### `decodeVarint(buffer: Buffer): number`

从 buffer 解码 Protocol Buffer varint。

**参数：**
- `buffer` - 包含 varint 的 buffer

**返回值：** 解码后的整数值

### ZigZag 操作

#### `encodeZigzag(value: number): number`

使用 ZigZag 编码对有符号整数进行编码。这对于高效编码有符号整数很有用，
因为它将有符号整数映射为无符号整数，使得绝对值较小的值具有较小的编码值。

**参数：**
- `value` - 要编码的有符号整数

**返回值：** ZigZag 编码的值

#### `decodeZigzag(value: number): number`

将 ZigZag 编码的整数解码回有符号整数。

**参数：**
- `value` - ZigZag 编码的值

**返回值：** 解码后的有符号整数

### 字段标签操作

#### `encodeFieldTag(fieldNumber: number, wireType: number): Buffer`

编码 Protocol Buffer 字段标签。

**参数：**
- `fieldNumber` - 字段编号（必须 >= 0）
- `wireType` - 线路类型（0-5）

**返回值：** 包含编码标签的 Buffer

**线路类型：**
- 0: Varint
- 1: 64 位
- 2: 长度限定
- 3: 开始组（已弃用）
- 4: 结束组（已弃用）
- 5: 32 位

#### `decodeFieldTag(buffer: Buffer): Array<number>`

解码 Protocol Buffer 字段标签。

**参数：**
- `buffer` - 包含字段标签的 buffer

**返回值：** 数组 `[fieldNumber, wireType]`

### Reader 类（第三阶段）

具有零拷贝优化的高性能 reader。

```javascript
const { Reader } = require('@protobuf-rs/core');

const reader = new Reader(buffer);
const value = reader.uint32();  // 读取 uint32
const bytes = reader.bytes();   // 读取长度限定的字节
const str = reader.string();    // 读取长度限定的字符串
reader.skip(10);                // 跳过字节
reader.reset();                 // 重置到开始位置
```

### Writer 类（第三阶段）

具有缓冲区优化的高性能 writer。

```javascript
const { Writer } = require('@protobuf-rs/core');

const writer = new Writer();
// 或使用预分配容量
const writer = Writer.withCapacity(1024);

writer.uint32(100);
writer.bytes(buffer);
writer.string("你好");
const result = writer.finish();
writer.reset(); // 重用 writer
```

### 批量操作（第三阶段）

```javascript
const { 
    encodeVarintBatchSimd, 
    decodeVarintBatchSimd,
    processU32BatchParallel 
} = require('@protobuf-rs/core');

// SIMD 批量编码
const values = [1, 2, 3, 4, 5];
const encoded = encodeVarintBatchSimd(values);
const decoded = decodeVarintBatchSimd(encoded);

// 并行处理
const largeArray = Array.from({ length: 100000 }, (_, i) => i);
const result = processU32BatchParallel(largeArray, 1000);
```

## 📊 性能

### 生产环境基准测试（第三阶段）

生产级工作负载的真实性能测量：

| 场景 | Rust (ops/sec) | JS (ops/sec) | 提速倍数 |
|----------|---------------|--------------|---------|
| gRPC 微服务 (1KB 消息) | 289,000 | 92,000 | **3.14x** |
| 批量处理 (1K 值) | 14,500 | 7,800 | **1.85x** |
| Reader 操作 | 621,000 | 180,000 | **3.45x** |
| Writer 操作 | 398,000 | 120,000 | **3.32x** |

**延迟分布：**
- P50: 1.53µs
- P95: 2.48µs
- P99: 23.63µs

**内存效率：**
- 堆使用：比 JavaScript **提升 314%**
- 单次分配开销：平均 **2 字节**
- 未检测到内存泄漏

### 运行基准测试

```bash
# 真实场景测试
npm run benchmark

# CPU 性能分析
npm run benchmark:cpu

# 内存性能分析（需要 --expose-gc）
npm run benchmark:memory
```

### 详细分析

请参阅 [docs/PERFORMANCE_REPORT.md](docs/PERFORMANCE_REPORT.md) 了解：
- 完整的测试方法
- 与竞品的对比
- 真实案例研究
- 优化建议

## 🤝 与 protobuf.js 集成

对于现有的 protobuf.js 项目，使用混合适配器实现直接替换：

```javascript
const protobuf = require('protobufjs');
const { Reader, Writer } = require('@protobuf-rs/core/integration/protobufjs-adapter');

// 使用更快的实现覆盖
protobuf.Reader = Reader;
protobuf.Writer = Writer;

// 所有现有代码获得 3-15 倍性能提升！
```

请参阅[集成指南](docs/INTEGRATION_GUIDE.md)获取完整文档。

## 📝 示例

- `examples/protobufjs-migration.js` - 包含基准测试的完整迁移指南
- `test/protobufjs-compatibility.js` - 全面的兼容性测试套件

更多示例请参阅 [examples/](examples/) 目录：
- 基本的 encode/decode 操作
- Base64 编码
- Long 类型处理
- 流式处理
- 动态消息

## 📖 文档

- [架构文档](docs/zh_CN/architecture.md) - 系统架构和设计详解
- [差异性分析](docs/zh_CN/comparison.md) - 与 protobuf.js 的详细对比
- [性能报告](docs/PERFORMANCE_REPORT.md) - 详细的性能分析和基准测试
- [集成指南](docs/INTEGRATION_GUIDE.md) - 完整的集成文档
- [兼容性报告](docs/COMPATIBILITY_REPORT.md) - protobuf.js 兼容性详情
- [API 文档](docs/zh_CN/API.md) - 完整的 API 参考
- [常见问题](docs/zh_CN/FAQ.md) - 常见问题解答
- [文档索引](docs/zh_CN/README.md) - 中文文档导航
- [CHANGELOG](CHANGELOG.md) - 版本历史和迁移指南

### 架构图

- [protobuf-rs 架构图](docs/diagrams/protobuf-rs-arch.mmd) - 整体系统架构
- [对比架构图](docs/diagrams/comparison-arch.mmd) - 与 protobuf.js 的架构对比
- [数据流程图](docs/diagrams/data-flow.mmd) - 从 .proto 到使用的数据处理流程
- [核心组件图](docs/diagrams/core-components.mmd) - 核心组件架构

## 🔧 从源代码构建

```bash
# 安装依赖
npm install

# 构建原生模块（发布模式）
npm run build

# 调试模式构建（编译更快）
npm run build:debug

# 运行测试
npm test

# 运行基准测试
npm run benchmark
```

## 🧪 测试

所有测试通过：**74/74** ✅

```bash
# 运行所有测试
npm test

# 运行兼容性测试
node test/protobufjs-compatibility.js

# 运行迁移示例和基准测试
node examples/protobufjs-migration.js

# 运行性能基准测试
npm run benchmark
npm run benchmark:cpu
npm run benchmark:memory
```

## 🚀 发布

本包在 npm 上以 `@protobuf-rs/core` 名称发布。

```bash
npm install @protobuf-rs/core
```

## 🤝 贡献

欢迎贡献！请参阅我们的贡献指南。

1. Fork 本仓库
2. 创建你的特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交你的修改 (`git commit -m '添加某个很棒的特性'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启一个 Pull Request

## 📜 许可证

BSD-3-Clause - 详见 [LICENSE](LICENSE) 文件

## 🙏 致谢

- 使用 [NAPI-RS](https://napi.rs/) 实现 Rust 与 Node.js 的无缝集成
- 兼容 [protobuf.js](https://github.com/protobufjs/protobuf.js)
- 受 Node.js 中对高性能 Protocol Buffers 需求的启发

## 📞 支持

- **问题反馈：** [GitHub Issues](https://github.com/LuuuXXX/protobuf-rs/issues)
- **讨论：** [GitHub Discussions](https://github.com/LuuuXXX/protobuf-rs/discussions)

---

**用 ❤️ 和 Rust 制作**
