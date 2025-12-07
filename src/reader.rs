//! Binary reader for Protocol Buffers wire format.
//! Protocol Buffers 线格式的二进制读取器。
//!
//! The `Reader` provides methods for decoding Protocol Buffers data from
//! a binary format. It operates on borrowed data without allocation (zero-copy).
//!
//! `Reader` 提供从二进制格式解码 Protocol Buffers 数据的方法。
//! 它在借用数据上操作，无需内存分配（零拷贝）。

#[cfg(not(feature = "std"))]
use alloc::string::String;

use crate::error::{DecodeError, Result};
use crate::varint::{decode_varint32, decode_varint64};
use crate::wire::{parse_tag, WireType};
use crate::zigzag::{decode_zigzag32, decode_zigzag64};

/// Binary reader for decoding Protocol Buffers.
/// 用于解码 Protocol Buffers 的二进制读取器。
///
/// # Examples
///
/// ```
/// use protobuf_rs::{Reader, WireType};
///
/// let data = vec![8, 42]; // field 1, varint, value 42
/// let mut reader = Reader::new(&data);
/// 
/// let (field, wire_type) = reader.read_tag().unwrap();
/// assert_eq!(field, 1);
/// assert_eq!(wire_type, WireType::Varint);
/// 
/// let value = reader.read_varint32().unwrap();
/// assert_eq!(value, 42);
/// ```
#[derive(Debug, Clone)]
pub struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    /// Create a new reader from a byte slice.
    /// 从字节切片创建新读取器。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Reader;
    ///
    /// let data = vec![1, 2, 3];
    /// let reader = Reader::new(&data);
    /// assert_eq!(reader.len(), 3);
    /// ```
    #[inline]
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    /// Get the current position in the buffer.
    /// 获取缓冲区中的当前位置。
    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Get the total length of the buffer.
    /// 获取缓冲区的总长度。
    #[inline]
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Get the number of remaining bytes.
    /// 获取剩余字节数。
    #[inline]
    pub fn remaining(&self) -> usize {
        self.buf.len() - self.pos
    }

    /// Check if the reader is empty (no remaining bytes).
    /// 检查读取器是否为空（无剩余字节）。
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.pos >= self.buf.len()
    }

    /// Check if we've reached the end of the buffer.
    /// 检查是否已到达缓冲区末尾。
    #[inline]
    pub fn is_eof(&self) -> bool {
        self.is_empty()
    }

    /// Reset the reader to the beginning.
    /// 将读取器重置到开头。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Reader;
    ///
    /// let data = vec![1, 2, 3];
    /// let mut reader = Reader::new(&data);
    /// reader.skip_bytes(2).unwrap();
    /// assert_eq!(reader.pos(), 2);
    /// reader.reset();
    /// assert_eq!(reader.pos(), 0);
    /// ```
    #[inline]
    pub fn reset(&mut self) {
        self.pos = 0;
    }

    /// Seek to a specific position.
    /// 跳转到特定位置。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Reader;
    ///
    /// let data = vec![1, 2, 3, 4, 5];
    /// let mut reader = Reader::new(&data);
    /// reader.seek(3).unwrap();
    /// assert_eq!(reader.pos(), 3);
    /// ```
    #[inline]
    pub fn seek(&mut self, pos: usize) -> Result<()> {
        if pos > self.buf.len() {
            return Err(DecodeError::BufferOverflow {
                position: self.pos,
                needed: pos - self.pos,
                available: self.remaining(),
            });
        }
        self.pos = pos;
        Ok(())
    }

    /// Read and parse a tag.
    /// 读取并解析标签。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::{Reader, WireType};
    ///
    /// let data = vec![8]; // field 1, varint
    /// let mut reader = Reader::new(&data);
    /// let (field, wire_type) = reader.read_tag().unwrap();
    /// assert_eq!(field, 1);
    /// assert_eq!(wire_type, WireType::Varint);
    /// ```
    pub fn read_tag(&mut self) -> Result<(u32, WireType)> {
        let tag = self.read_varint32()?;
        parse_tag(tag)
    }

    /// Peek at the next tag without consuming it.
    /// 查看下一个标签而不消费它。
    pub fn peek_tag(&self) -> Result<(u32, WireType)> {
        let mut temp_reader = Reader::new(&self.buf[self.pos..]);
        let tag = temp_reader.read_varint32()?;
        parse_tag(tag)
    }

    /// Read a 32-bit varint.
    /// 读取 32 位 varint。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Reader;
    ///
    /// let data = vec![0xAC, 0x02]; // 300
    /// let mut reader = Reader::new(&data);
    /// let value = reader.read_varint32().unwrap();
    /// assert_eq!(value, 300);
    /// ```
    pub fn read_varint32(&mut self) -> Result<u32> {
        let (value, len) = decode_varint32(&self.buf[self.pos..]).ok_or_else(|| {
            DecodeError::InvalidVarint {
                position: self.pos,
            }
        })?;
        self.pos += len;
        Ok(value)
    }

    /// Read a 64-bit varint.
    /// 读取 64 位 varint。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Reader;
    ///
    /// let data = vec![0xAC, 0x02]; // 300
    /// let mut reader = Reader::new(&data);
    /// let value = reader.read_varint64().unwrap();
    /// assert_eq!(value, 300);
    /// ```
    pub fn read_varint64(&mut self) -> Result<u64> {
        let (value, len) = decode_varint64(&self.buf[self.pos..]).ok_or_else(|| {
            DecodeError::InvalidVarint {
                position: self.pos,
            }
        })?;
        self.pos += len;
        Ok(value)
    }

    /// Read a signed 32-bit integer (ZigZag encoded).
    /// 读取有符号 32 位整数（ZigZag 编码）。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Reader;
    ///
    /// let data = vec![1]; // -1 in ZigZag
    /// let mut reader = Reader::new(&data);
    /// let value = reader.read_sint32().unwrap();
    /// assert_eq!(value, -1);
    /// ```
    #[inline]
    pub fn read_sint32(&mut self) -> Result<i32> {
        let value = self.read_varint32()?;
        Ok(decode_zigzag32(value))
    }

    /// Read a signed 64-bit integer (ZigZag encoded).
    /// 读取有符号 64 位整数（ZigZag 编码）。
    #[inline]
    pub fn read_sint64(&mut self) -> Result<i64> {
        let value = self.read_varint64()?;
        Ok(decode_zigzag64(value))
    }

    /// Read a signed 32-bit integer (not ZigZag encoded).
    /// 读取有符号 32 位整数（非 ZigZag 编码）。
    #[inline]
    pub fn read_int32(&mut self) -> Result<i32> {
        self.read_varint32().map(|v| v as i32)
    }

    /// Read a signed 64-bit integer (not ZigZag encoded).
    /// 读取有符号 64 位整数（非 ZigZag 编码）。
    #[inline]
    pub fn read_int64(&mut self) -> Result<i64> {
        self.read_varint64().map(|v| v as i64)
    }

    /// Read a fixed 32-bit unsigned integer.
    /// 读取固定 32 位无符号整数。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Reader;
    ///
    /// let data = vec![0x78, 0x56, 0x34, 0x12]; // 0x12345678
    /// let mut reader = Reader::new(&data);
    /// let value = reader.read_fixed32().unwrap();
    /// assert_eq!(value, 0x12345678);
    /// ```
    pub fn read_fixed32(&mut self) -> Result<u32> {
        if self.remaining() < 4 {
            return Err(DecodeError::BufferOverflow {
                position: self.pos,
                needed: 4,
                available: self.remaining(),
            });
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 4]);
        self.pos += 4;
        Ok(u32::from_le_bytes(bytes))
    }

    /// Read a fixed 64-bit unsigned integer.
    /// 读取固定 64 位无符号整数。
    pub fn read_fixed64(&mut self) -> Result<u64> {
        if self.remaining() < 8 {
            return Err(DecodeError::BufferOverflow {
                position: self.pos,
                needed: 8,
                available: self.remaining(),
            });
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 8]);
        self.pos += 8;
        Ok(u64::from_le_bytes(bytes))
    }

    /// Read a fixed 32-bit signed integer.
    /// 读取固定 32 位有符号整数。
    #[inline]
    pub fn read_sfixed32(&mut self) -> Result<i32> {
        self.read_fixed32().map(|v| v as i32)
    }

    /// Read a fixed 64-bit signed integer.
    /// 读取固定 64 位有符号整数。
    #[inline]
    pub fn read_sfixed64(&mut self) -> Result<i64> {
        self.read_fixed64().map(|v| v as i64)
    }

    /// Read a 32-bit floating point number.
    /// 读取 32 位浮点数。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Reader;
    ///
    /// let data = vec![0x00, 0x00, 0x80, 0x3F]; // 1.0
    /// let mut reader = Reader::new(&data);
    /// let value = reader.read_float().unwrap();
    /// assert_eq!(value, 1.0);
    /// ```
    pub fn read_float(&mut self) -> Result<f32> {
        if self.remaining() < 4 {
            return Err(DecodeError::BufferOverflow {
                position: self.pos,
                needed: 4,
                available: self.remaining(),
            });
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 4]);
        self.pos += 4;
        Ok(f32::from_le_bytes(bytes))
    }

    /// Read a 64-bit floating point number.
    /// 读取 64 位浮点数。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Reader;
    ///
    /// let data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x3F]; // 1.0
    /// let mut reader = Reader::new(&data);
    /// let value = reader.read_double().unwrap();
    /// assert_eq!(value, 1.0);
    /// ```
    pub fn read_double(&mut self) -> Result<f64> {
        if self.remaining() < 8 {
            return Err(DecodeError::BufferOverflow {
                position: self.pos,
                needed: 8,
                available: self.remaining(),
            });
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 8]);
        self.pos += 8;
        Ok(f64::from_le_bytes(bytes))
    }

    /// Read a boolean value.
    /// 读取布尔值。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Reader;
    ///
    /// let data = vec![1]; // true
    /// let mut reader = Reader::new(&data);
    /// let value = reader.read_bool().unwrap();
    /// assert_eq!(value, true);
    /// ```
    #[inline]
    pub fn read_bool(&mut self) -> Result<bool> {
        let value = self.read_varint32()?;
        Ok(value != 0)
    }

    /// Read a string (length-delimited).
    /// 读取字符串（长度分隔）。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Reader;
    ///
    /// let data = vec![4, b't', b'e', b's', b't'];
    /// let mut reader = Reader::new(&data);
    /// let value = reader.read_string().unwrap();
    /// assert_eq!(value, "test");
    /// ```
    pub fn read_string(&mut self) -> Result<String> {
        let bytes = self.read_bytes()?;
        String::from_utf8(bytes.to_vec()).map_err(|_| DecodeError::InvalidUtf8 {
            position: self.pos,
        })
    }

    /// Read a string reference (length-delimited, zero-copy).
    /// 读取字符串引用（长度分隔，零拷贝）。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Reader;
    ///
    /// let data = vec![4, b't', b'e', b's', b't'];
    /// let mut reader = Reader::new(&data);
    /// let value = reader.read_string_ref().unwrap();
    /// assert_eq!(value, "test");
    /// ```
    pub fn read_string_ref(&mut self) -> Result<&'a str> {
        let bytes = self.read_bytes_ref()?;
        core::str::from_utf8(bytes).map_err(|_| DecodeError::InvalidUtf8 {
            position: self.pos,
        })
    }

    /// Read bytes (length-delimited).
    /// 读取字节（长度分隔）。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Reader;
    ///
    /// let data = vec![3, 1, 2, 3];
    /// let mut reader = Reader::new(&data);
    /// let value = reader.read_bytes().unwrap();
    /// assert_eq!(value, &[1, 2, 3]);
    /// ```
    #[inline]
    pub fn read_bytes(&mut self) -> Result<&'a [u8]> {
        self.read_bytes_ref()
    }

    /// Read bytes reference (length-delimited, zero-copy).
    /// 读取字节引用（长度分隔，零拷贝）。
    pub fn read_bytes_ref(&mut self) -> Result<&'a [u8]> {
        let len = self.read_varint32()? as usize;
        if self.remaining() < len {
            return Err(DecodeError::BufferOverflow {
                position: self.pos,
                needed: len,
                available: self.remaining(),
            });
        }
        let bytes = &self.buf[self.pos..self.pos + len];
        self.pos += len;
        Ok(bytes)
    }

    /// Skip a field based on its wire type.
    /// 根据线类型跳过字段。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::{Reader, WireType};
    ///
    /// let data = vec![8, 42]; // field 1, varint, value 42
    /// let mut reader = Reader::new(&data);
    /// reader.skip(WireType::Varint).unwrap();
    /// reader.skip(WireType::Varint).unwrap();
    /// assert!(reader.is_eof());
    /// ```
    pub fn skip(&mut self, wire_type: WireType) -> Result<()> {
        match wire_type {
            WireType::Varint => {
                self.read_varint64()?;
            }
            WireType::Fixed64 => {
                self.skip_bytes(8)?;
            }
            WireType::LengthDelimited => {
                let len = self.read_varint32()? as usize;
                self.skip_bytes(len)?;
            }
            WireType::StartGroup => {
                // Groups are deprecated, but we should handle them
                loop {
                    let (_, wt) = self.read_tag()?;
                    if wt == WireType::EndGroup {
                        break;
                    }
                    self.skip(wt)?;
                }
            }
            WireType::EndGroup => {
                return Err(DecodeError::InvalidWireType {
                    wire_type: WireType::EndGroup.as_u8(),
                });
            }
            WireType::Fixed32 => {
                self.skip_bytes(4)?;
            }
        }
        Ok(())
    }

    /// Skip n bytes.
    /// 跳过 n 个字节。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Reader;
    ///
    /// let data = vec![1, 2, 3, 4, 5];
    /// let mut reader = Reader::new(&data);
    /// reader.skip_bytes(3).unwrap();
    /// assert_eq!(reader.pos(), 3);
    /// ```
    pub fn skip_bytes(&mut self, n: usize) -> Result<()> {
        if self.remaining() < n {
            return Err(DecodeError::BufferOverflow {
                position: self.pos,
                needed: n,
                available: self.remaining(),
            });
        }
        self.pos += n;
        Ok(())
    }

    /// Read a sub-reader for a length-delimited field.
    /// 为长度分隔字段读取子读取器。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::Reader;
    ///
    /// let data = vec![3, 1, 2, 3];
    /// let mut reader = Reader::new(&data);
    /// let sub = reader.read_sub_reader().unwrap();
    /// assert_eq!(sub.len(), 3);
    /// ```
    pub fn read_sub_reader(&mut self) -> Result<Reader<'a>> {
        let bytes = self.read_bytes_ref()?;
        Ok(Reader::new(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Writer;

    #[test]
    fn test_reader_new() {
        let data = vec![1, 2, 3];
        let reader = Reader::new(&data);
        assert_eq!(reader.len(), 3);
        assert_eq!(reader.remaining(), 3);
        assert_eq!(reader.pos(), 0);
    }

    #[test]
    fn test_read_varint32() {
        let data = vec![0xAC, 0x02]; // 300
        let mut reader = Reader::new(&data);
        let value = reader.read_varint32().unwrap();
        assert_eq!(value, 300);
        assert!(reader.is_eof());
    }

    #[test]
    fn test_read_varint64() {
        let data = vec![0xAC, 0x02]; // 300
        let mut reader = Reader::new(&data);
        let value = reader.read_varint64().unwrap();
        assert_eq!(value, 300);
    }

    #[test]
    fn test_read_tag() {
        let data = vec![8]; // field 1, varint
        let mut reader = Reader::new(&data);
        let (field, wire_type) = reader.read_tag().unwrap();
        assert_eq!(field, 1);
        assert_eq!(wire_type, WireType::Varint);
    }

    #[test]
    fn test_read_string() {
        let data = vec![4, b't', b'e', b's', b't'];
        let mut reader = Reader::new(&data);
        let value = reader.read_string().unwrap();
        assert_eq!(value, "test");
    }

    #[test]
    fn test_read_bytes() {
        let data = vec![3, 1, 2, 3];
        let mut reader = Reader::new(&data);
        let value = reader.read_bytes().unwrap();
        assert_eq!(value, &[1, 2, 3]);
    }

    #[test]
    fn test_read_bool() {
        let data = vec![1, 0];
        let mut reader = Reader::new(&data);
        assert_eq!(reader.read_bool().unwrap(), true);
        assert_eq!(reader.read_bool().unwrap(), false);
    }

    #[test]
    fn test_read_fixed32() {
        let data = vec![0x78, 0x56, 0x34, 0x12];
        let mut reader = Reader::new(&data);
        let value = reader.read_fixed32().unwrap();
        assert_eq!(value, 0x12345678);
    }

    #[test]
    fn test_read_fixed64() {
        let data = vec![0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12];
        let mut reader = Reader::new(&data);
        let value = reader.read_fixed64().unwrap();
        assert_eq!(value, 0x123456789ABCDEF0);
    }

    #[test]
    fn test_read_float() {
        let data = vec![0x00, 0x00, 0x80, 0x3F];
        let mut reader = Reader::new(&data);
        let value = reader.read_float().unwrap();
        assert_eq!(value, 1.0);
    }

    #[test]
    fn test_read_double() {
        let data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x3F];
        let mut reader = Reader::new(&data);
        let value = reader.read_double().unwrap();
        assert_eq!(value, 1.0);
    }

    #[test]
    fn test_read_sint32() {
        let data = vec![1]; // -1 in ZigZag
        let mut reader = Reader::new(&data);
        let value = reader.read_sint32().unwrap();
        assert_eq!(value, -1);
    }

    #[test]
    fn test_skip() {
        let data = vec![8, 42, 18, 4, b't', b'e', b's', b't'];
        let mut reader = Reader::new(&data);
        
        // Skip first field
        let (_, wt) = reader.read_tag().unwrap();
        reader.skip(wt).unwrap();
        
        // Read second field
        let (field, _) = reader.read_tag().unwrap();
        assert_eq!(field, 2);
        let s = reader.read_string().unwrap();
        assert_eq!(s, "test");
    }

    #[test]
    fn test_roundtrip() {
        let mut writer = Writer::new();
        writer.write_uint32_field(1, 42);
        writer.write_string_field(2, "test");
        writer.write_bool_field(3, true);
        
        let bytes = writer.finish();
        let mut reader = Reader::new(&bytes);
        
        let (field, _) = reader.read_tag().unwrap();
        assert_eq!(field, 1);
        assert_eq!(reader.read_varint32().unwrap(), 42);
        
        let (field, _) = reader.read_tag().unwrap();
        assert_eq!(field, 2);
        assert_eq!(reader.read_string().unwrap(), "test");
        
        let (field, _) = reader.read_tag().unwrap();
        assert_eq!(field, 3);
        assert_eq!(reader.read_bool().unwrap(), true);
    }

    #[test]
    fn test_buffer_overflow() {
        let data = vec![1];
        let mut reader = Reader::new(&data);
        assert!(reader.read_fixed32().is_err());
    }

    #[test]
    fn test_invalid_utf8() {
        let data = vec![3, 0xFF, 0xFE, 0xFD];
        let mut reader = Reader::new(&data);
        assert!(reader.read_string().is_err());
    }
}
