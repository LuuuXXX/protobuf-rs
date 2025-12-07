//! Wire type definitions and tag manipulation.
//! 线类型定义和标签操作。
//!
//! Protocol Buffers uses a binary wire format where each field is preceded by a tag.
//! The tag contains the field number and wire type encoded as a varint.
//!
//! Protocol Buffers 使用二进制线格式，其中每个字段前面都有一个标签。
//! 标签包含编码为 varint 的字段号和线类型。
//!
//! ## Wire Types | 线类型
//!
//! - **Varint**: Variable-length integers (int32, int64, uint32, uint64, sint32, sint64, bool, enum)
//!   **Varint**: 可变长度整数
//!
//! - **Fixed64**: 8-byte values (fixed64, sfixed64, double)
//!   **Fixed64**: 8 字节值
//!
//! - **LengthDelimited**: Length-prefixed data (string, bytes, embedded messages, packed repeated fields)
//!   **LengthDelimited**: 长度前缀数据
//!
//! - **Fixed32**: 4-byte values (fixed32, sfixed32, float)
//!   **Fixed32**: 4 字节值
//!
//! ## Tag Format | 标签格式
//!
//! A tag is encoded as: `(field_number << 3) | wire_type`
//! 标签编码为: `(字段号 << 3) | 线类型`

use crate::error::{DecodeError, Result};

/// Wire type used in protobuf binary format.
/// protobuf 二进制格式中使用的线类型。
///
/// Each field in a protobuf message is tagged with both a field number
/// and a wire type that indicates how to interpret the following data.
///
/// protobuf 消息中的每个字段都标记有字段号和线类型，
/// 线类型指示如何解释后续数据。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WireType {
    /// Variable-length integer (0)
    /// 可变长度整数 (0)
    Varint = 0,

    /// 64-bit fixed-size value (1)
    /// 64 位定长值 (1)
    Fixed64 = 1,

    /// Length-delimited data (2)
    /// 长度分隔数据 (2)
    LengthDelimited = 2,

    /// Start group (deprecated, 3)
    /// 开始组 (已弃用, 3)
    StartGroup = 3,

    /// End group (deprecated, 4)
    /// 结束组 (已弃用, 4)
    EndGroup = 4,

    /// 32-bit fixed-size value (5)
    /// 32 位定长值 (5)
    Fixed32 = 5,
}

impl WireType {
    /// Convert wire type to its numeric representation.
    /// 将线类型转换为其数值表示。
    #[inline]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Try to convert a numeric value to a wire type.
    /// 尝试将数值转换为线类型。
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf_rs::WireType;
    ///
    /// assert_eq!(WireType::from_u8(0), Some(WireType::Varint));
    /// assert_eq!(WireType::from_u8(2), Some(WireType::LengthDelimited));
    /// assert_eq!(WireType::from_u8(7), None);
    /// ```
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(WireType::Varint),
            1 => Some(WireType::Fixed64),
            2 => Some(WireType::LengthDelimited),
            3 => Some(WireType::StartGroup),
            4 => Some(WireType::EndGroup),
            5 => Some(WireType::Fixed32),
            _ => None,
        }
    }
}

/// Maximum valid field number (2^29 - 1).
/// 最大有效字段号 (2^29 - 1)。
pub const MAX_FIELD_NUMBER: u32 = 536_870_911;

/// Start of reserved field number range (19000-19999).
/// 保留字段号范围的起始值 (19000-19999)。
pub const RESERVED_FIELD_START: u32 = 19_000;

/// End of reserved field number range (19000-19999).
/// 保留字段号范围的结束值 (19000-19999)。
pub const RESERVED_FIELD_END: u32 = 19_999;

/// Check if a field number is valid.
/// 检查字段号是否有效。
///
/// Valid field numbers are 1 to MAX_FIELD_NUMBER (536,870,911),
/// excluding the reserved range 19000-19999.
///
/// 有效的字段号是 1 到 MAX_FIELD_NUMBER (536,870,911)，
/// 不包括保留范围 19000-19999。
///
/// # Examples
///
/// ```
/// use protobuf_rs::wire::is_valid_field_number;
///
/// assert!(is_valid_field_number(1));
/// assert!(is_valid_field_number(100));
/// assert!(!is_valid_field_number(0));
/// assert!(!is_valid_field_number(19000));
/// assert!(!is_valid_field_number(536_870_912));
/// ```
#[inline]
pub const fn is_valid_field_number(field_number: u32) -> bool {
    field_number > 0
        && field_number <= MAX_FIELD_NUMBER
        && !(field_number >= RESERVED_FIELD_START && field_number <= RESERVED_FIELD_END)
}

/// Create a tag from field number and wire type.
/// 从字段号和线类型创建标签。
///
/// # Panics
///
/// Panics if the field number is invalid (0, > MAX_FIELD_NUMBER, or in reserved range).
/// 如果字段号无效（0、> MAX_FIELD_NUMBER 或在保留范围内），则会 panic。
///
/// # Examples
///
/// ```
/// use protobuf_rs::wire::{make_tag, WireType};
///
/// let tag = make_tag(1, WireType::Varint);
/// assert_eq!(tag, 8); // (1 << 3) | 0
///
/// let tag = make_tag(2, WireType::LengthDelimited);
/// assert_eq!(tag, 18); // (2 << 3) | 2
/// ```
#[inline]
pub fn make_tag(field_number: u32, wire_type: WireType) -> u32 {
    assert!(
        is_valid_field_number(field_number),
        "Invalid field number: {}",
        field_number
    );
    (field_number << 3) | (wire_type.as_u8() as u32)
}

/// Parse a tag into field number and wire type.
/// 将标签解析为字段号和线类型。
///
/// # Examples
///
/// ```
/// use protobuf_rs::wire::{parse_tag, WireType};
///
/// let (field_number, wire_type) = parse_tag(8).unwrap();
/// assert_eq!(field_number, 1);
/// assert_eq!(wire_type, WireType::Varint);
///
/// let (field_number, wire_type) = parse_tag(18).unwrap();
/// assert_eq!(field_number, 2);
/// assert_eq!(wire_type, WireType::LengthDelimited);
/// ```
#[inline]
pub fn parse_tag(tag: u32) -> Result<(u32, WireType)> {
    let wire_type_value = (tag & 0x7) as u8;
    let field_number = tag >> 3;

    let wire_type = WireType::from_u8(wire_type_value).ok_or(DecodeError::InvalidWireType {
        wire_type: wire_type_value,
    })?;

    if !is_valid_field_number(field_number) {
        return Err(DecodeError::InvalidTag { tag });
    }

    Ok((field_number, wire_type))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_type_conversions() {
        assert_eq!(WireType::Varint.as_u8(), 0);
        assert_eq!(WireType::Fixed64.as_u8(), 1);
        assert_eq!(WireType::LengthDelimited.as_u8(), 2);
        assert_eq!(WireType::Fixed32.as_u8(), 5);

        assert_eq!(WireType::from_u8(0), Some(WireType::Varint));
        assert_eq!(WireType::from_u8(1), Some(WireType::Fixed64));
        assert_eq!(WireType::from_u8(2), Some(WireType::LengthDelimited));
        assert_eq!(WireType::from_u8(5), Some(WireType::Fixed32));
        assert_eq!(WireType::from_u8(6), None);
        assert_eq!(WireType::from_u8(7), None);
    }

    #[test]
    fn test_is_valid_field_number() {
        assert!(!is_valid_field_number(0));
        assert!(is_valid_field_number(1));
        assert!(is_valid_field_number(100));
        assert!(is_valid_field_number(18999));
        assert!(!is_valid_field_number(19000));
        assert!(!is_valid_field_number(19500));
        assert!(!is_valid_field_number(19999));
        assert!(is_valid_field_number(20000));
        assert!(is_valid_field_number(MAX_FIELD_NUMBER));
        assert!(!is_valid_field_number(MAX_FIELD_NUMBER + 1));
    }

    #[test]
    fn test_make_tag() {
        assert_eq!(make_tag(1, WireType::Varint), 8);
        assert_eq!(make_tag(1, WireType::Fixed64), 9);
        assert_eq!(make_tag(1, WireType::LengthDelimited), 10);
        assert_eq!(make_tag(2, WireType::Varint), 16);
        assert_eq!(make_tag(2, WireType::LengthDelimited), 18);
    }

    #[test]
    #[should_panic]
    fn test_make_tag_invalid_field_number_zero() {
        make_tag(0, WireType::Varint);
    }

    #[test]
    #[should_panic]
    fn test_make_tag_invalid_field_number_reserved() {
        make_tag(19000, WireType::Varint);
    }

    #[test]
    fn test_parse_tag() {
        assert_eq!(parse_tag(8).unwrap(), (1, WireType::Varint));
        assert_eq!(parse_tag(9).unwrap(), (1, WireType::Fixed64));
        assert_eq!(parse_tag(10).unwrap(), (1, WireType::LengthDelimited));
        assert_eq!(parse_tag(16).unwrap(), (2, WireType::Varint));
        assert_eq!(parse_tag(18).unwrap(), (2, WireType::LengthDelimited));
    }

    #[test]
    fn test_parse_tag_invalid_wire_type() {
        let result = parse_tag(7); // wire type 7 is invalid
        assert!(matches!(result, Err(DecodeError::InvalidWireType { .. })));
    }

    #[test]
    #[allow(clippy::identity_op)]
    fn test_parse_tag_invalid_field_number() {
        let tag = (0 << 3) | 0; // field number 0, wire type 0
        assert!(matches!(
            parse_tag(tag),
            Err(DecodeError::InvalidTag { .. })
        ));

        let tag = (19000 << 3) | 0; // reserved field number, wire type 0
        assert!(matches!(
            parse_tag(tag),
            Err(DecodeError::InvalidTag { .. })
        ));
    }

    #[test]
    fn test_tag_roundtrip() {
        for field_number in [1, 2, 100, 18999, 20000, MAX_FIELD_NUMBER] {
            for wire_type in [
                WireType::Varint,
                WireType::Fixed64,
                WireType::LengthDelimited,
                WireType::Fixed32,
            ] {
                let tag = make_tag(field_number, wire_type);
                let (parsed_field, parsed_wire) = parse_tag(tag).unwrap();
                assert_eq!(parsed_field, field_number);
                assert_eq!(parsed_wire, wire_type);
            }
        }
    }
}
