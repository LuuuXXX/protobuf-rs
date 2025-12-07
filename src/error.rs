//! Error types for protobuf encoding and decoding.
//! protobuf 编解码错误类型。
//!
//! This module defines comprehensive error types that provide detailed information
//! about what went wrong during serialization or deserialization.
//!
//! 本模块定义了全面的错误类型，提供序列化或反序列化过程中出错的详细信息。

use thiserror::Error;

/// Result type alias using DecodeError
/// 使用 DecodeError 的 Result 类型别名
pub type Result<T> = core::result::Result<T, DecodeError>;

/// Errors that can occur during decoding.
/// 解码过程中可能发生的错误。
///
/// Each variant contains relevant position and size information
/// to help diagnose issues in the binary data.
///
/// 每个变体都包含相关的位置和大小信息，以帮助诊断二进制数据中的问题。
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    /// Buffer overflow - tried to read beyond buffer bounds.
    /// 缓冲区溢出 - 尝试读取超出缓冲区边界的数据。
    #[error("Buffer overflow at position {position}: needed {needed} bytes, but only {available} available")]
    BufferOverflow {
        position: usize,
        needed: usize,
        available: usize,
    },

    /// Invalid varint encoding - varint exceeds maximum allowed size.
    /// 无效的 varint 编码 - varint 超过最大允许大小。
    #[error("Invalid varint at position {position}: exceeded maximum length")]
    InvalidVarint { position: usize },

    /// Invalid wire type value.
    /// 无效的线类型值。
    #[error("Invalid wire type: {wire_type}")]
    InvalidWireType { wire_type: u8 },

    /// Invalid UTF-8 encoding in string field.
    /// 字符串字段中的无效 UTF-8 编码。
    #[error("Invalid UTF-8 at position {position}")]
    InvalidUtf8 { position: usize },

    /// Unexpected end of buffer.
    /// 意外的缓冲区结束。
    #[error("Unexpected end of buffer")]
    UnexpectedEndOfBuffer,

    /// Invalid tag - field number is 0 or reserved.
    /// 无效标签 - 字段号为 0 或保留。
    #[error("Invalid tag: {tag}")]
    InvalidTag { tag: u32 },

    /// Recursion limit exceeded (for nested messages).
    /// 超过递归限制（用于嵌套消息）。
    #[error("Recursion limit exceeded: depth {depth}")]
    RecursionLimitExceeded { depth: usize },
}

/// Errors that can occur during encoding.
/// 编码过程中可能发生的错误。
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum EncodeError {
    /// Buffer too small for the data being written.
    /// 缓冲区对于正在写入的数据来说太小。
    #[error("Buffer too small: needed {needed} bytes, but only {available} available")]
    BufferTooSmall { needed: usize, available: usize },

    /// String exceeds maximum length for length-delimited encoding.
    /// 字符串超过长度分隔编码的最大长度。
    #[error("String too long: {length} bytes")]
    StringTooLong { length: usize },

    /// Invalid field number (must be 1-536870911).
    /// 无效的字段号（必须是 1-536870911）。
    #[error("Invalid field number: {field_number}")]
    InvalidFieldNumber { field_number: u32 },
}