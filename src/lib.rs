//! # protobuf-rs
//!
//! High-performance Rust implementation of Protocol Buffers core.
//! 高性能 Rust 实现的 Protocol Buffers 核心库。
//!
//! This library provides low-level primitives for encoding and decoding Protocol Buffers
//! wire format, designed to be compiled to WebAssembly for use with JavaScript/TypeScript.
//!
//! 本库提供 Protocol Buffers 线格式编解码的底层原语，
//! 设计用于编译为 WebAssembly 以便与 JavaScript/TypeScript 配合使用。
//!
//! ## Features | 功能特性
//!
//! - **Zero-copy reading**: Reader operates on borrowed data without allocation
//!   **零拷贝读取**: Reader 在借用数据上操作，无需内存分配
//!
//! - **Efficient writing**: Writer with automatic buffer growth
//!   **高效写入**: Writer 支持自动缓冲区增长
//!
//! - **WASM support**: Feature-gated WebAssembly bindings
//!   **WASM 支持**: 特性门控的 WebAssembly 绑定
//!
//! ## Quick Start | 快速开始
//!
//! ```rust
//! use protobuf_rs::{Writer, Reader, WireType};
//!
//! // Encoding | 编码
//! let mut writer = Writer::new();
//! writer.write_tag(1, WireType::Varint);
//! writer.write_varint32(42);
//! writer.write_tag(2, WireType::LengthDelimited);
//! writer.write_string("Hello, Protobuf!");
//! let bytes = writer.finish();
//!
//! // Decoding | 解码
//! let mut reader = Reader::new(&bytes);
//! let (field1, _) = reader.read_tag().unwrap();
//! let value1 = reader.read_varint32().unwrap();
//! let (field2, _) = reader.read_tag().unwrap();
//! let value2 = reader.read_string().unwrap();
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod error;
pub mod reader;
pub mod varint;
#[cfg(feature = "wasm")]
pub mod wasm;
pub mod wire;
pub mod writer;
pub mod zigzag;

// Re-exports for convenience | 便捷的重新导出
pub use error::{DecodeError, EncodeError, Result};
pub use reader::Reader;
pub use wire::WireType;
pub use writer::Writer;
