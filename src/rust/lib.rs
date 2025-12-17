//! High-performance Protocol Buffers implementation in Rust
//! 
//! This module provides core Protocol Buffer encoding/decoding functionality
//! with performance-critical operations implemented in Rust for maximum efficiency.

use napi::bindgen_prelude::*;
use napi_derive::napi;

mod encoder;
mod decoder;
mod reader;
mod writer;

pub use encoder::Encoder;
pub use decoder::Decoder;
pub use reader::Reader;
pub use writer::Writer;

/// Protocol Buffers specification constants
const MAX_VARINT_BYTES: usize = 10;  // Maximum bytes for varint (64-bit integer)
const MAX_WIRE_TYPE: u32 = 5;        // Maximum wire type number

/// Encode a varint (variable-length integer)
/// 
/// # Arguments
/// * `value` - The 64-bit integer to encode
/// 
/// # Returns
/// A Buffer containing the encoded varint
#[napi]
pub fn encode_varint(value: i64) -> Result<Buffer> {
    let mut result = Vec::new();
    let mut val = value as u64;
    
    loop {
        let mut byte = (val & 0x7F) as u8;
        val >>= 7;
        
        if val != 0 {
            byte |= 0x80;
        }
        
        result.push(byte);
        
        if val == 0 {
            break;
        }
    }
    
    Ok(result.into())
}

/// Decode a varint from a buffer
/// 
/// # Arguments
/// * `buffer` - The buffer containing the varint
/// 
/// # Returns
/// The decoded 64-bit integer
#[napi]
pub fn decode_varint(buffer: Buffer) -> Result<i64> {
    let bytes = buffer.as_ref();
    let mut result: u64 = 0;
    let mut shift = 0;
    
    for (i, &byte) in bytes.iter().enumerate() {
        if i >= MAX_VARINT_BYTES {
            return Err(Error::from_reason("Varint too long"));
        }
        
        result |= ((byte & 0x7F) as u64) << shift;
        
        if byte & 0x80 == 0 {
            return Ok(result as i64);
        }
        
        shift += 7;
    }
    
    Err(Error::from_reason("Incomplete varint"))
}

/// Encode a signed integer using ZigZag encoding
/// 
/// ZigZag encoding maps signed integers to unsigned integers efficiently,
/// so that small absolute values have small encoded values.
/// 
/// # Arguments
/// * `value` - The signed 64-bit integer to encode
/// 
/// # Returns
/// The ZigZag encoded value as signed 64-bit integer
#[napi]
pub fn encode_zigzag(value: i64) -> i64 {
    ((value << 1) ^ (value >> 63)) as i64
}

/// Decode a ZigZag encoded integer
/// 
/// # Arguments
/// * `value` - The ZigZag encoded value
/// 
/// # Returns
/// The decoded signed 64-bit integer
#[napi]
pub fn decode_zigzag(value: i64) -> i64 {
    let uval = value as u64;
    ((uval >> 1) as i64) ^ (-((uval & 1) as i64))
}

/// Encode a field tag (field number + wire type)
/// 
/// # Arguments
/// * `field_number` - The field number (must be >= 0)
/// * `wire_type` - The wire type (0-5)
/// 
/// # Returns
/// A Buffer containing the encoded tag
#[napi]
pub fn encode_field_tag(field_number: u32, wire_type: u32) -> Result<Buffer> {
    if wire_type > MAX_WIRE_TYPE {
        return Err(Error::from_reason(format!(
            "Invalid wire type: {}. Must be 0-5",
            wire_type
        )));
    }
    
    let tag = (field_number << 3) | wire_type;
    encode_varint(tag as i64)
}

/// Decode a field tag
/// 
/// # Arguments
/// * `buffer` - The buffer containing the field tag
/// 
/// # Returns
/// A tuple of [field_number, wire_type]
#[napi]
pub fn decode_field_tag(buffer: Buffer) -> Result<Vec<u32>> {
    let tag = decode_varint(buffer)? as u32;
    let field_number = tag >> 3;
    let wire_type = tag & 0x7;
    
    Ok(vec![field_number, wire_type])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varint_encoding() {
        let value = 300i64;
        let encoded = encode_varint(value).unwrap();
        let decoded = decode_varint(encoded).unwrap();
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_zigzag_encoding() {
        let value = -150i64;
        let encoded = encode_zigzag(value);
        let decoded = decode_zigzag(encoded);
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_field_tag() {
        let field_number = 1u32;
        let wire_type = 2u32;
        let encoded = encode_field_tag(field_number, wire_type).unwrap();
        let decoded = decode_field_tag(encoded).unwrap();
        assert_eq!(decoded[0], field_number);
        assert_eq!(decoded[1], wire_type);
    }
}
