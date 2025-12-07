//! ZigZag encoding for signed integers.
//! 有符号整数的 ZigZag 编码。
//!
//! ZigZag encoding maps signed integers to unsigned integers so that numbers
//! with small absolute values have small encoded values. This is more efficient
//! than encoding negative numbers directly as varints.
//!
//! ZigZag 编码将有符号整数映射到无符号整数，使得绝对值较小的数字具有较小的编码值。
//! 这比直接将负数编码为 varint 更有效率。
//!
//! ## Mapping | 映射
//!
//! ```text
//! Signed  -> Unsigned (ZigZag)
//! 0       -> 0
//! -1      -> 1
//! 1       -> 2
//! -2      -> 3
//! 2       -> 4
//! ...
//! ```
//!
//! ## Formula | 公式
//!
//! - Encode: `(n << 1) ^ (n >> 31)` for 32-bit
//!   编码: 32 位使用 `(n << 1) ^ (n >> 31)`
//!
//! - Decode: `(n >> 1) ^ -(n & 1)` for 32-bit
//!   解码: 32 位使用 `(n >> 1) ^ -(n & 1)`

/// Encode a signed 32-bit integer using ZigZag encoding.
/// 使用 ZigZag 编码对有符号 32 位整数进行编码。
///
/// # Examples
///
/// ```
/// use protobuf_rs::zigzag::encode_zigzag32;
///
/// assert_eq!(encode_zigzag32(0), 0);
/// assert_eq!(encode_zigzag32(-1), 1);
/// assert_eq!(encode_zigzag32(1), 2);
/// assert_eq!(encode_zigzag32(-2), 3);
/// assert_eq!(encode_zigzag32(2), 4);
/// assert_eq!(encode_zigzag32(i32::MAX), 4294967294);
/// assert_eq!(encode_zigzag32(i32::MIN), 4294967295);
/// ```
#[inline]
pub const fn encode_zigzag32(n: i32) -> u32 {
    ((n << 1) ^ (n >> 31)) as u32
}

/// Decode a ZigZag-encoded 32-bit integer.
/// 解码 ZigZag 编码的 32 位整数。
///
/// # Examples
///
/// ```
/// use protobuf_rs::zigzag::decode_zigzag32;
///
/// assert_eq!(decode_zigzag32(0), 0);
/// assert_eq!(decode_zigzag32(1), -1);
/// assert_eq!(decode_zigzag32(2), 1);
/// assert_eq!(decode_zigzag32(3), -2);
/// assert_eq!(decode_zigzag32(4), 2);
/// assert_eq!(decode_zigzag32(4294967294), i32::MAX);
/// assert_eq!(decode_zigzag32(4294967295), i32::MIN);
/// ```
#[inline]
pub const fn decode_zigzag32(n: u32) -> i32 {
    ((n >> 1) as i32) ^ (-((n & 1) as i32))
}

/// Encode a signed 64-bit integer using ZigZag encoding.
/// 使用 ZigZag 编码对有符号 64 位整数进行编码。
///
/// # Examples
///
/// ```
/// use protobuf_rs::zigzag::encode_zigzag64;
///
/// assert_eq!(encode_zigzag64(0), 0);
/// assert_eq!(encode_zigzag64(-1), 1);
/// assert_eq!(encode_zigzag64(1), 2);
/// assert_eq!(encode_zigzag64(-2), 3);
/// assert_eq!(encode_zigzag64(2), 4);
/// assert_eq!(encode_zigzag64(i64::MAX), 18446744073709551614);
/// assert_eq!(encode_zigzag64(i64::MIN), 18446744073709551615);
/// ```
#[inline]
pub const fn encode_zigzag64(n: i64) -> u64 {
    ((n << 1) ^ (n >> 63)) as u64
}

/// Decode a ZigZag-encoded 64-bit integer.
/// 解码 ZigZag 编码的 64 位整数。
///
/// # Examples
///
/// ```
/// use protobuf_rs::zigzag::decode_zigzag64;
///
/// assert_eq!(decode_zigzag64(0), 0);
/// assert_eq!(decode_zigzag64(1), -1);
/// assert_eq!(decode_zigzag64(2), 1);
/// assert_eq!(decode_zigzag64(3), -2);
/// assert_eq!(decode_zigzag64(4), 2);
/// assert_eq!(decode_zigzag64(18446744073709551614), i64::MAX);
/// assert_eq!(decode_zigzag64(18446744073709551615), i64::MIN);
/// ```
#[inline]
pub const fn decode_zigzag64(n: u64) -> i64 {
    ((n >> 1) as i64) ^ (-((n & 1) as i64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zigzag32_basic() {
        assert_eq!(encode_zigzag32(0), 0);
        assert_eq!(encode_zigzag32(-1), 1);
        assert_eq!(encode_zigzag32(1), 2);
        assert_eq!(encode_zigzag32(-2), 3);
        assert_eq!(encode_zigzag32(2), 4);
        assert_eq!(encode_zigzag32(-3), 5);
        assert_eq!(encode_zigzag32(3), 6);
    }

    #[test]
    fn test_zigzag32_extremes() {
        assert_eq!(encode_zigzag32(i32::MAX), 4294967294);
        assert_eq!(encode_zigzag32(i32::MIN), 4294967295);
    }

    #[test]
    fn test_zigzag32_roundtrip() {
        let test_values = [
            0,
            1,
            -1,
            2,
            -2,
            127,
            -127,
            128,
            -128,
            32767,
            -32768,
            i32::MAX,
            i32::MIN,
        ];

        for &value in &test_values {
            let encoded = encode_zigzag32(value);
            let decoded = decode_zigzag32(encoded);
            assert_eq!(decoded, value, "Failed roundtrip for {}", value);
        }
    }

    #[test]
    fn test_zigzag64_basic() {
        assert_eq!(encode_zigzag64(0), 0);
        assert_eq!(encode_zigzag64(-1), 1);
        assert_eq!(encode_zigzag64(1), 2);
        assert_eq!(encode_zigzag64(-2), 3);
        assert_eq!(encode_zigzag64(2), 4);
        assert_eq!(encode_zigzag64(-3), 5);
        assert_eq!(encode_zigzag64(3), 6);
    }

    #[test]
    fn test_zigzag64_extremes() {
        assert_eq!(encode_zigzag64(i64::MAX), 18446744073709551614);
        assert_eq!(encode_zigzag64(i64::MIN), 18446744073709551615);
    }

    #[test]
    fn test_zigzag64_roundtrip() {
        let test_values = [
            0,
            1,
            -1,
            2,
            -2,
            127,
            -127,
            128,
            -128,
            32767,
            -32768,
            i32::MAX as i64,
            i32::MIN as i64,
            i64::MAX,
            i64::MIN,
        ];

        for &value in &test_values {
            let encoded = encode_zigzag64(value);
            let decoded = decode_zigzag64(encoded);
            assert_eq!(decoded, value, "Failed roundtrip for {}", value);
        }
    }

    #[test]
    fn test_decode_zigzag32_values() {
        assert_eq!(decode_zigzag32(0), 0);
        assert_eq!(decode_zigzag32(1), -1);
        assert_eq!(decode_zigzag32(2), 1);
        assert_eq!(decode_zigzag32(3), -2);
        assert_eq!(decode_zigzag32(4), 2);
        assert_eq!(decode_zigzag32(4294967294), i32::MAX);
        assert_eq!(decode_zigzag32(4294967295), i32::MIN);
    }

    #[test]
    fn test_decode_zigzag64_values() {
        assert_eq!(decode_zigzag64(0), 0);
        assert_eq!(decode_zigzag64(1), -1);
        assert_eq!(decode_zigzag64(2), 1);
        assert_eq!(decode_zigzag64(3), -2);
        assert_eq!(decode_zigzag64(4), 2);
        assert_eq!(decode_zigzag64(18446744073709551614), i64::MAX);
        assert_eq!(decode_zigzag64(18446744073709551615), i64::MIN);
    }
}
