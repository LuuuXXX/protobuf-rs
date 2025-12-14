# protobuf-rs 中文文档

欢迎使用 protobuf-rs 中文文档！本文档提供了项目的完整中文资料。

## 📚 文档目录

### 架构和设计

- **[架构文档](architecture.md)** - 详细的系统架构说明
  - 整体架构图
  - 核心组件详解
  - 数据流程分析
  - 性能优化架构

- **[差异性分析](comparison.md)** - 与 protobuf.js 的全面对比
  - 架构差异分析
  - 实现差异说明
  - 性能收益分析（5个关键场景）
  - 性能测试数据
  - 权衡和劣势
  - 适用场景建议

### API 和使用

- **[API 文档](API.md)** - 完整的 API 参考
  - 核心 API
  - Varint 操作
  - ZigZag 操作
  - Reader/Writer 类
  - 批量操作
  - 性能监控

- **[常见问题 FAQ](FAQ.md)** - 常见问题解答
  - 安装和使用
  - 性能相关
  - 兼容性
  - 故障排除

### 性能和集成

- **[性能报告](PERFORMANCE_REPORT.md)** - 详细的性能分析
  - 真实场景基准测试
  - 与 protobuf.js 对比
  - 性能优化建议

- **[集成指南](INTEGRATION_GUIDE.md)** - 完整的集成文档
  - 零代码迁移指南
  - 与 protobuf.js 集成
  - 最佳实践

- **[兼容性报告](COMPATIBILITY_REPORT.md)** - protobuf.js 兼容性详情
  - API 兼容性
  - 行为一致性
  - 已知差异

## 🗺️ 架构图

项目提供了多个架构图帮助理解系统设计：

### 1. protobuf-rs 整体架构
- 展示各模块之间的关系
- 数据流向和处理流程
- 性能优化组件
- 📄 [查看架构图](../diagrams/protobuf-rs-arch.mmd)

### 2. 与 protobuf.js 的架构对比
- 并排对比两种实现
- 标注关键差异点
- 性能优势可视化
- 📄 [查看对比图](../diagrams/comparison-arch.mmd)

### 3. 数据流程图
- 从 .proto 到最终使用的完整流程
- 编码/解码详细步骤
- 性能关键路径
- 📄 [查看流程图](../diagrams/data-flow.mmd)

### 4. 核心组件图
- 编码器/解码器架构
- Reader/Writer 设计
- 性能优化组件
- 📄 [查看组件图](../diagrams/core-components.mmd)

## 🚀 快速导航

### 新手入门
1. 阅读 [主 README](../../README.zh.md) 了解项目概况
2. 查看 [常见问题 FAQ](FAQ.md) 解答基本疑问
3. 参考 [API 文档](API.md) 学习如何使用

### 深入理解
1. 阅读 [架构文档](architecture.md) 理解系统设计
2. 查看 [差异性分析](comparison.md) 了解与 protobuf.js 的区别
3. 研究 [性能报告](PERFORMANCE_REPORT.md) 掌握性能优化

### 项目集成
1. 按照 [集成指南](INTEGRATION_GUIDE.md) 集成到项目
2. 参考 [兼容性报告](COMPATIBILITY_REPORT.md) 确保兼容性
3. 使用 [API 文档](API.md) 作为开发参考

## 📊 性能数据速览

| 场景 | protobuf-rs | protobuf.js | 提升 |
|------|-------------|-------------|------|
| **gRPC 微服务** | 289K ops/s | 92K ops/s | **3.14x** |
| **批量处理** | 14.5K ops/s | 7.8K ops/s | **1.85x** |
| **Reader 操作** | 621K ops/s | 180K ops/s | **3.45x** |
| **Writer 操作** | 397K ops/s | 120K ops/s | **3.31x** |
| **内存占用** | 45.3 MB | 78.6 MB | **-42%** |

详见 [性能报告](PERFORMANCE_REPORT.md)

## 🎯 核心优势

### 性能
- ⚡ **3-15 倍性能提升** - 核心操作使用 Rust 实现
- 💾 **42% 内存减少** - 无 GC 开销，确定性释放
- 🚀 **亚微秒级延迟** - P50 延迟仅 1.53µs
- 📈 **线性扩展** - 并行处理，充分利用多核

### 兼容性
- ✅ **100% API 兼容** - 与 protobuf.js 完全兼容
- 🔄 **零代码迁移** - 仅需修改一行 require
- 🛡️ **自动降级** - 不支持平台自动切换到 JS
- 🌐 **跨平台支持** - Linux, macOS, Windows

### 可靠性
- 🔒 **内存安全** - Rust 所有权系统保证
- 🧪 **完整测试** - 74/74 测试通过
- 📊 **生产验证** - 真实场景性能测试
- 🛠️ **主动维护** - 持续更新和优化

## 🔗 相关资源

### 项目链接
- [GitHub 仓库](https://github.com/LuuuXXX/protobuf-rs)
- [npm 包](https://www.npmjs.com/package/@protobuf-rs/core)
- [问题反馈](https://github.com/LuuuXXX/protobuf-rs/issues)
- [讨论区](https://github.com/LuuuXXX/protobuf-rs/discussions)

### 外部资源
- [Protocol Buffers 规范](https://protobuf.dev/)
- [protobuf.js 文档](https://github.com/protobufjs/protobuf.js)
- [NAPI-RS 文档](https://napi.rs/)
- [Rust 官方文档](https://www.rust-lang.org/)

## 📝 贡献文档

如果你发现文档有误或需要改进，欢迎：

1. 提交 [Issue](https://github.com/LuuuXXX/protobuf-rs/issues)
2. 发起 [Pull Request](https://github.com/LuuuXXX/protobuf-rs/pulls)
3. 在 [Discussions](https://github.com/LuuuXXX/protobuf-rs/discussions) 讨论

## 📄 许可证

本项目采用 BSD-3-Clause 许可证。详见 [LICENSE](../../LICENSE)。

---

**用 ❤️ 和 Rust 制作**
