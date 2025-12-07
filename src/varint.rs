//! Variable-length integer encoding and decoding.
//! 可变长度整数编解码。
//!
//! Varints are a method of serializing integers using one or more bytes.
//! Smaller numbers take fewer bytes. Each byte in a varint has a continuation
//! bit that indicates whether there are more bytes to follow.
//!
//! Varint 是一种使用一个或多个字节序列化整数的方法。
//! 较小的数字占用较少的字节。varint 中的每个字节都有一个延续位，
//! 指示是否有更多字节需要读取。
//!
//! ## Encoding | 编码
//!
//! - Each byte uses the lower 7 bits for data
//!   每个字节使用低 7 位存储数据
//! - The MSB (most significant bit) is the continuation bit
//!   MSB（最高有效位）是延续位
//! - MSB = 1 means more bytes follow, MSB = 0 means this is the last byte
//!   MSB = 1 表示后面还有更多字节，MSB = 0 表示这是最后一个字节
//!
//! ## Examples | 示例
//!
//! ```text
//! 1 -> 0x01          (1 byte)
//! 127 -> 0x7F        (1 byte)
//! 128 -> 0x80 0x01   (2 bytes)
//! 300 -> 0xAC 0x02   (2 bytes)
//! ```

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Maximum number of bytes in a 32-bit varint.
/// 32 位 varint 的最大字节数。
pub const MAX_VARINT32_BYTES: usize = 5;

/// Maximum number of bytes in a 64-bit varint.
/// 64 位 varint 的最大字节数。
pub const MAX_VARINT64_BYTES: usize = 10;

/// Calculate the encoded size of a 32-bit varint.
/// 计算 32 位 varint 的编码大小。
///
/// # Examples
///
/// ```
/// use protobuf_rs::varint::varint32_size;
///
/// assert_eq!(varint32_size(0), 1);
/// assert_eq!(varint32_size(127), 1);
/// assert_eq!(varint32_size(128), 2);
/// assert_eq!(varint32_size(16383), 2);
/// assert_eq!(varint32_size(16384), 3);
/// ```
#[inline]
pub const fn varint32_size(mut value: u32) -> usize {
    if value == 0 {
        return 1;
    }

    let mut size = 0;
    while value > 0 {
        size += 1;
        value >>= 7;
    }
    size
}

/// Calculate the encoded size of a 64-bit varint.
/// 计算 64 位 varint 的编码大小。
///
/// # Examples
///
/// ```
/// use protobuf_rs::varint::varint64_size;
///
/// assert_eq!(varint64_size(0), 1);
/// assert_eq!(varint64_size(127), 1);
/// assert_eq!(varint64_size(128), 2);
/// assert_eq!(varint64_size(u64::MAX), 10);
/// ```
#[inline]
pub const fn varint64_size(mut value: u64) -> usize {
    if value == 0 {
        return 1;
    }

    let mut size = 0;
    while value > 0 {
        size += 1;
        value >>= 7;
    }
    size
}

/// Encode a 32-bit varint into a buffer.
/// 将 32 位 varint 编码到缓冲区中。
///
/// Returns the number of bytes written.
/// 返回写入的字节数。
///
/// # Panics
///
/// Panics if the buffer is too small (< MAX_VARINT32_BYTES).
/// 如果缓冲区太小（< MAX_VARINT32_BYTES），则会 panic。
///
/// # Examples
///
/// ```
/// use protobuf_rs::varint::encode_varint32;
///
/// let mut buf = [0u8; 5];
/// let len = encode_varint32(&mut buf, 300);
/// assert_eq!(len, 2);
/// assert_eq!(&buf[..len], &[0xAC, 0x02]);
/// ```
#[inline]
pub fn encode_varint32(buf: &mut [u8], mut value: u32) -> usize {
    let mut i = 0;
    while value >= 0x80 {
        buf[i] = (value as u8) | 0x80;
        value >>= 7;
        i += 1;
    }
    buf[i] = value as u8;
    i + 1
}

/// Encode a 64-bit varint into a buffer.
/// 将 64 位 varint 编码到缓冲区中。
///
/// Returns the number of bytes written.
/// 返回写入的字节数。
///
/// # Panics
///
/// Panics if the buffer is too small (< MAX_VARINT64_BYTES).
/// 如果缓冲区太小（< MAX_VARINT64_BYTES），则会 panic。
///
/// # Examples
///
/// ```
/// use protobuf_rs::varint::encode_varint64;
///
/// let mut buf = [0u8; 10];
/// let len = encode_varint64(&mut buf, 300);
/// assert_eq!(len, 2);
/// assert_eq!(&buf[..len], &[0xAC, 0x02]);
/// ```
#[inline]
pub fn encode_varint64(buf: &mut [u8], mut value: u64) -> usize {
    let mut i = 0;
    while value >= 0x80 {
        buf[i] = (value as u8) | 0x80;
        value >>= 7;
        i += 1;
    }
    buf[i] = value as u8;
    i + 1
}

/// Decode a 32-bit varint from a buffer.
/// 从缓冲区解码 32 位 varint。
///
/// Returns `Some((value, bytes_read))` on success, `None` if the buffer
/// is too small or the varint is invalid.
///
/// 成功时返回 `Some((值, 已读字节数))`，如果缓冲区太小或 varint 无效则返回 `None`。
///
/// # Examples
///
/// ```
/// use protobuf_rs::varint::decode_varint32;
///
/// let buf = [0xAC, 0x02];
/// let (value, len) = decode_varint32(&buf).unwrap();
/// assert_eq!(value, 300);
/// assert_eq!(len, 2);
/// ```
#[inline]
pub fn decode_varint32(buf: &[u8]) -> Option<(u32, usize)> {
    let mut result = 0u32;
    let mut shift = 0;

    for (i, &byte) in buf.iter().enumerate() {
        if i >= MAX_VARINT32_BYTES {
            return None; // Varint too long
        }

        let value = (byte & 0x7F) as u32;

        // Check for overflow
        if shift >= 32 || (shift == 28 && value > 0x0F) {
            return None;
        }

        result |= value << shift;

        if byte < 0x80 {
            return Some((result, i + 1));
        }

        shift += 7;
    }

    None // Unexpected end of buffer
}

/// Decode a 64-bit varint from a buffer.
/// 从缓冲区解码 64 位 varint。
///
/// Returns `Some((value, bytes_read))` on success, `None` if the buffer
/// is too small or the varint is invalid.
///
/// 成功时返回 `Some((值, 已读字节数))`，如果缓冲区太小或 varint 无效则返回 `None`。
///
/// # Examples
///
/// ```
/// use protobuf_rs::varint::decode_varint64;
///
/// let buf = [0xAC, 0x02];
/// let (value, len) = decode_varint64(&buf).unwrap();
/// assert_eq!(value, 300);
/// assert_eq!(len, 2);
/// ```
#[inline]
pub fn decode_varint64(buf: &[u8]) -> Option<(u64, usize)> {
    let mut result = 0u64;
    let mut shift = 0;

    for (i, &byte) in buf.iter().enumerate() {
        if i >= MAX_VARINT64_BYTES {
            return None; // Varint too long
        }

        let value = (byte & 0x7F) as u64;

        // Check for overflow
        if shift >= 64 || (shift == 63 && value > 0x01) {
            return None;
        }

        result |= value << shift;

        if byte < 0x80 {
            return Some((result, i + 1));
        }

        shift += 7;
    }

    None // Unexpected end of buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varint32_size() {
        assert_eq!(varint32_size(0), 1);
        assert_eq!(varint32_size(1), 1);
        assert_eq!(varint32_size(127), 1);
        assert_eq!(varint32_size(128), 2);
        assert_eq!(varint32_size(255), 2);
        assert_eq!(varint32_size(16383), 2);
        assert_eq!(varint32_size(16384), 3);
        assert_eq!(varint32_size(u32::MAX), 5);
    }

    #[test]
    fn test_varint64_size() {
        assert_eq!(varint64_size(0), 1);
        assert_eq!(varint64_size(1), 1);
        assert_eq!(varint64_size(127), 1);
        assert_eq!(varint64_size(128), 2);
        assert_eq!(varint64_size(16384), 3);
        assert_eq!(varint64_size(u32::MAX as u64), 5);
        assert_eq!(varint64_size(u64::MAX), 10);
    }

    #[test]
    fn test_encode_decode_varint32() {
        let test_cases = [0, 1, 127, 128, 255, 300, 16383, 16384, u32::MAX];

        for &value in &test_cases {
            let mut buf = [0u8; MAX_VARINT32_BYTES];
            let len = encode_varint32(&mut buf, value);
            assert_eq!(len, varint32_size(value));

            let (decoded, decoded_len) = decode_varint32(&buf).unwrap();
            assert_eq!(decoded, value);
            assert_eq!(decoded_len, len);
        }
    }

    #[test]
    fn test_encode_decode_varint64() {
        let test_cases = [
            0,
            1,
            127,
            128,
            255,
            300,
            16383,
            16384,
            u32::MAX as u64,
            u64::MAX,
        ];

        for &value in &test_cases {
            let mut buf = [0u8; MAX_VARINT64_BYTES];
            let len = encode_varint64(&mut buf, value);
            assert_eq!(len, varint64_size(value));

            let (decoded, decoded_len) = decode_varint64(&buf).unwrap();
            assert_eq!(decoded, value);
            assert_eq!(decoded_len, len);
        }
    }

    #[test]
    fn test_decode_varint32_invalid() {
        // Too many bytes
        let buf = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        assert_eq!(decode_varint32(&buf), None);

        // Incomplete varint
        let buf = [0xFF];
        assert_eq!(decode_varint32(&buf), None);

        // Empty buffer
        let buf = [];
        assert_eq!(decode_varint32(&buf), None);
    }

    #[test]
    fn test_decode_varint64_invalid() {
        // Too many bytes
        let buf = [0xFF; 11];
        assert_eq!(decode_varint64(&buf), None);

        // Incomplete varint
        let buf = [0xFF];
        assert_eq!(decode_varint64(&buf), None);

        // Empty buffer
        let buf = [];
        assert_eq!(decode_varint64(&buf), None);
    }

    #[test]
    fn test_specific_values() {
        // Test 300 specifically as shown in examples
        let mut buf = [0u8; 10];
        let len = encode_varint32(&mut buf, 300);
        assert_eq!(len, 2);
        assert_eq!(&buf[..len], &[0xAC, 0x02]);

        let (value, decoded_len) = decode_varint32(&buf).unwrap();
        assert_eq!(value, 300);
        assert_eq!(decoded_len, 2);
    }
}
