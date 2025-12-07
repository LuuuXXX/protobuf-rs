//! Binary writer for Protocol Buffers wire format.
//! Protocol Buffers 线格式的二进制写入器。
//!
//! The `Writer` provides methods for encoding Protocol Buffers data into
//! a binary format. It manages a growable buffer and provides convenience
//! methods for writing all protobuf types.
//!
//! `Writer` 提供将 Protocol Buffers 数据编码为二进制格式的方法。
//! 它管理一个可增长的缓冲区，并提供用于写入所有 protobuf 类型的便捷方法。

#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::varint::{encode_varint32, encode_varint64, MAX_VARINT32_BYTES, MAX_VARINT64_BYTES};
use crate::wire::{make_tag, WireType};
use crate::zigzag::{encode_zigzag32, encode_zigzag64};

/// Binary writer for encoding Protocol Buffers.
/// 用于编码 Protocol Buffers 的二进制写入器。
///
/// # Examples
///
/// ```
/// use protobuf_rs::{Writer, WireType};
///
/// let mut writer = Writer::new();
/// writer.write_tag(1, WireType::Varint);
/// writer.write_varint32(42);
/// writer.write_string_field(2, "Hello");
///
/// let bytes = writer.finish();
/// assert!(bytes.len() > 0);
/// ```
#[derive(Debug, Clone)]
pub struct Writer {
    buf: Vec<u8>,
}

impl Writer {
    /// Create a new writer with default capacity.
    /// 创建具有默认容量的新写入器。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let writer = Writer::new();
    /// assert_eq!(writer.len(), 0);
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    /// Create a new writer with the specified capacity.
    /// 创建具有指定容量的新写入器。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let writer = Writer::with_capacity(1024);
    /// assert_eq!(writer.len(), 0);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: Vec::with_capacity(capacity),
        }
    }

    /// Get the current length of encoded data.
    /// 获取已编码数据的当前长度。
    #[inline]
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Check if the writer is empty.
    /// 检查写入器是否为空。
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// Reset the writer, clearing all data.
    /// 重置写入器，清除所有数据。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_varint32(42);
    /// assert!(writer.len() > 0);
    /// writer.reset();
    /// assert_eq!(writer.len(), 0);
    /// ```
    #[inline]
    pub fn reset(&mut self) {
        self.buf.clear();
    }

    /// Finish writing and return the encoded bytes.
    /// 完成写入并返回编码的字节。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_varint32(42);
    /// let bytes = writer.finish();
    /// assert!(bytes.len() > 0);
    /// ```
    #[inline]
    pub fn finish(self) -> Vec<u8> {
        self.buf
    }

    /// Get a slice of the encoded data.
    /// 获取已编码数据的切片。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_varint32(42);
    /// let slice = writer.as_slice();
    /// assert!(slice.len() > 0);
    /// ```
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.buf
    }

    /// Write a tag (field number + wire type).
    /// 写入标签（字段号 + 线类型）。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::{Writer, WireType};
    ///
    /// let mut writer = Writer::new();
    /// writer.write_tag(1, WireType::Varint);
    /// ```
    #[inline]
    pub fn write_tag(&mut self, field_number: u32, wire_type: WireType) {
        let tag = make_tag(field_number, wire_type);
        self.write_varint32(tag);
    }

    /// Write a 32-bit varint.
    /// 写入 32 位 varint。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_varint32(300);
    /// ```
    #[inline]
    pub fn write_varint32(&mut self, value: u32) {
        let mut tmp = [0u8; MAX_VARINT32_BYTES];
        let len = encode_varint32(&mut tmp, value);
        self.buf.extend_from_slice(&tmp[..len]);
    }

    /// Write a 64-bit varint.
    /// 写入 64 位 varint。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_varint64(300);
    /// ```
    #[inline]
    pub fn write_varint64(&mut self, value: u64) {
        let mut tmp = [0u8; MAX_VARINT64_BYTES];
        let len = encode_varint64(&mut tmp, value);
        self.buf.extend_from_slice(&tmp[..len]);
    }

    /// Write a signed 32-bit integer using ZigZag encoding.
    /// 使用 ZigZag 编码写入有符号 32 位整数。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_sint32(-1);
    /// ```
    #[inline]
    pub fn write_sint32(&mut self, value: i32) {
        self.write_varint32(encode_zigzag32(value));
    }

    /// Write a signed 64-bit integer using ZigZag encoding.
    /// 使用 ZigZag 编码写入有符号 64 位整数。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_sint64(-1);
    /// ```
    #[inline]
    pub fn write_sint64(&mut self, value: i64) {
        self.write_varint64(encode_zigzag64(value));
    }

    /// Write a fixed 32-bit unsigned integer.
    /// 写入固定 32 位无符号整数。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_fixed32(42);
    /// ```
    #[inline]
    pub fn write_fixed32(&mut self, value: u32) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    /// Write a fixed 64-bit unsigned integer.
    /// 写入固定 64 位无符号整数。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_fixed64(42);
    /// ```
    #[inline]
    pub fn write_fixed64(&mut self, value: u64) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    /// Write a fixed 32-bit signed integer.
    /// 写入固定 32 位有符号整数。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_sfixed32(-42);
    /// ```
    #[inline]
    pub fn write_sfixed32(&mut self, value: i32) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    /// Write a fixed 64-bit signed integer.
    /// 写入固定 64 位有符号整数。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_sfixed64(-42);
    /// ```
    #[inline]
    pub fn write_sfixed64(&mut self, value: i64) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    /// Write a 32-bit floating point number.
    /// 写入 32 位浮点数。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_float(3.14);
    /// ```
    #[inline]
    pub fn write_float(&mut self, value: f32) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    /// Write a 64-bit floating point number.
    /// 写入 64 位浮点数。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_double(3.14159);
    /// ```
    #[inline]
    pub fn write_double(&mut self, value: f64) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    /// Write a boolean value.
    /// 写入布尔值。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_bool(true);
    /// ```
    #[inline]
    pub fn write_bool(&mut self, value: bool) {
        self.write_varint32(if value { 1 } else { 0 });
    }

    /// Write a string with length prefix.
    /// 写入带长度前缀的字符串。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_string("Hello, World!");
    /// ```
    #[inline]
    pub fn write_string(&mut self, value: &str) {
        self.write_bytes(value.as_bytes());
    }

    /// Write bytes with length prefix.
    /// 写入带长度前缀的字节。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_bytes(&[1, 2, 3, 4]);
    /// ```
    #[inline]
    pub fn write_bytes(&mut self, value: &[u8]) {
        self.write_varint32(value.len() as u32);
        self.buf.extend_from_slice(value);
    }

    /// Write raw bytes without length prefix.
    /// 写入原始字节，不带长度前缀。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_raw_bytes(&[1, 2, 3, 4]);
    /// ```
    #[inline]
    pub fn write_raw_bytes(&mut self, value: &[u8]) {
        self.buf.extend_from_slice(value);
    }

    // Convenience methods for writing fields with tags
    // 带标签写入字段的便捷方法

    /// Write a uint32 field (tag + value).
    /// 写入 uint32 字段（标签 + 值）。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_uint32_field(1, 42);
    /// ```
    #[inline]
    pub fn write_uint32_field(&mut self, field_number: u32, value: u32) {
        self.write_tag(field_number, WireType::Varint);
        self.write_varint32(value);
    }

    /// Write a uint64 field (tag + value).
    /// 写入 uint64 字段（标签 + 值）。
    #[inline]
    pub fn write_uint64_field(&mut self, field_number: u32, value: u64) {
        self.write_tag(field_number, WireType::Varint);
        self.write_varint64(value);
    }

    /// Write an int32 field (tag + value).
    /// 写入 int32 字段（标签 + 值）。
    #[inline]
    pub fn write_int32_field(&mut self, field_number: u32, value: i32) {
        self.write_tag(field_number, WireType::Varint);
        self.write_varint32(value as u32);
    }

    /// Write an int64 field (tag + value).
    /// 写入 int64 字段（标签 + 值）。
    #[inline]
    pub fn write_int64_field(&mut self, field_number: u32, value: i64) {
        self.write_tag(field_number, WireType::Varint);
        self.write_varint64(value as u64);
    }

    /// Write a sint32 field (tag + value).
    /// 写入 sint32 字段（标签 + 值）。
    #[inline]
    pub fn write_sint32_field(&mut self, field_number: u32, value: i32) {
        self.write_tag(field_number, WireType::Varint);
        self.write_sint32(value);
    }

    /// Write a sint64 field (tag + value).
    /// 写入 sint64 字段（标签 + 值）。
    #[inline]
    pub fn write_sint64_field(&mut self, field_number: u32, value: i64) {
        self.write_tag(field_number, WireType::Varint);
        self.write_sint64(value);
    }

    /// Write a string field (tag + value).
    /// 写入字符串字段（标签 + 值）。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_string_field(2, "Hello");
    /// ```
    #[inline]
    pub fn write_string_field(&mut self, field_number: u32, value: &str) {
        self.write_tag(field_number, WireType::LengthDelimited);
        self.write_string(value);
    }

    /// Write a bytes field (tag + value).
    /// 写入字节字段（标签 + 值）。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_bytes_field(3, &[1, 2, 3]);
    /// ```
    #[inline]
    pub fn write_bytes_field(&mut self, field_number: u32, value: &[u8]) {
        self.write_tag(field_number, WireType::LengthDelimited);
        self.write_bytes(value);
    }

    /// Write a bool field (tag + value).
    /// 写入布尔字段（标签 + 值）。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_bool_field(4, true);
    /// ```
    #[inline]
    pub fn write_bool_field(&mut self, field_number: u32, value: bool) {
        self.write_tag(field_number, WireType::Varint);
        self.write_bool(value);
    }

    /// Write a float field (tag + value).
    /// 写入浮点字段（标签 + 值）。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_float_field(5, 3.14);
    /// ```
    #[inline]
    pub fn write_float_field(&mut self, field_number: u32, value: f32) {
        self.write_tag(field_number, WireType::Fixed32);
        self.write_float(value);
    }

    /// Write a double field (tag + value).
    /// 写入双精度浮点字段（标签 + 值）。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.write_double_field(6, 3.14159);
    /// ```
    #[inline]
    pub fn write_double_field(&mut self, field_number: u32, value: f64) {
        self.write_tag(field_number, WireType::Fixed64);
        self.write_double(value);
    }

    /// Write a fixed32 field (tag + value).
    /// 写入 fixed32 字段（标签 + 值）。
    #[inline]
    pub fn write_fixed32_field(&mut self, field_number: u32, value: u32) {
        self.write_tag(field_number, WireType::Fixed32);
        self.write_fixed32(value);
    }

    /// Write a fixed64 field (tag + value).
    /// 写入 fixed64 字段（标签 + 值）。
    #[inline]
    pub fn write_fixed64_field(&mut self, field_number: u32, value: u64) {
        self.write_tag(field_number, WireType::Fixed64);
        self.write_fixed64(value);
    }

    /// Write an sfixed32 field (tag + value).
    /// 写入 sfixed32 字段（标签 + 值）。
    #[inline]
    pub fn write_sfixed32_field(&mut self, field_number: u32, value: i32) {
        self.write_tag(field_number, WireType::Fixed32);
        self.write_sfixed32(value);
    }

    /// Write an sfixed64 field (tag + value).
    /// 写入 sfixed64 字段（标签 + 值）。
    #[inline]
    pub fn write_sfixed64_field(&mut self, field_number: u32, value: i64) {
        self.write_tag(field_number, WireType::Fixed64);
        self.write_sfixed64(value);
    }
}

impl Default for Writer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_writer_new() {
        let writer = Writer::new();
        assert_eq!(writer.len(), 0);
        assert!(writer.is_empty());
    }

    #[test]
    fn test_writer_with_capacity() {
        let writer = Writer::with_capacity(1024);
        assert_eq!(writer.len(), 0);
    }

    #[test]
    fn test_writer_reset() {
        let mut writer = Writer::new();
        writer.write_varint32(42);
        assert!(!writer.is_empty());
        writer.reset();
        assert_eq!(writer.len(), 0);
    }

    #[test]
    fn test_write_varint32() {
        let mut writer = Writer::new();
        writer.write_varint32(300);
        assert_eq!(writer.as_slice(), &[0xAC, 0x02]);
    }

    #[test]
    fn test_write_tag() {
        let mut writer = Writer::new();
        writer.write_tag(1, WireType::Varint);
        assert_eq!(writer.as_slice(), &[8]); // (1 << 3) | 0
    }

    #[test]
    fn test_write_string() {
        let mut writer = Writer::new();
        writer.write_string("test");
        assert_eq!(writer.as_slice(), &[4, b't', b'e', b's', b't']);
    }

    #[test]
    fn test_write_bytes() {
        let mut writer = Writer::new();
        writer.write_bytes(&[1, 2, 3]);
        assert_eq!(writer.as_slice(), &[3, 1, 2, 3]);
    }

    #[test]
    fn test_write_bool() {
        let mut writer = Writer::new();
        writer.write_bool(true);
        assert_eq!(writer.as_slice(), &[1]);

        writer.reset();
        writer.write_bool(false);
        assert_eq!(writer.as_slice(), &[0]);
    }

    #[test]
    fn test_write_fixed32() {
        let mut writer = Writer::new();
        writer.write_fixed32(0x12345678);
        assert_eq!(writer.as_slice(), &[0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn test_write_fixed64() {
        let mut writer = Writer::new();
        writer.write_fixed64(0x123456789ABCDEF0);
        assert_eq!(
            writer.as_slice(),
            &[0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]
        );
    }

    #[test]
    fn test_write_float() {
        let mut writer = Writer::new();
        writer.write_float(1.0);
        assert_eq!(writer.as_slice(), &[0x00, 0x00, 0x80, 0x3F]);
    }

    #[test]
    fn test_write_double() {
        let mut writer = Writer::new();
        writer.write_double(1.0);
        assert_eq!(
            writer.as_slice(),
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x3F]
        );
    }

    #[test]
    fn test_write_sint32() {
        let mut writer = Writer::new();
        writer.write_sint32(-1);
        assert_eq!(writer.as_slice(), &[1]); // ZigZag encoded
    }

    #[test]
    fn test_write_uint32_field() {
        let mut writer = Writer::new();
        writer.write_uint32_field(1, 42);
        assert_eq!(writer.as_slice(), &[8, 42]);
    }

    #[test]
    fn test_write_string_field() {
        let mut writer = Writer::new();
        writer.write_string_field(2, "Hi");
        assert_eq!(writer.as_slice(), &[18, 2, b'H', b'i']);
    }

    #[test]
    fn test_multiple_fields() {
        let mut writer = Writer::new();
        writer.write_uint32_field(1, 150);
        writer.write_string_field(2, "test");

        let bytes = writer.finish();
        assert!(!bytes.is_empty());
    }
}
