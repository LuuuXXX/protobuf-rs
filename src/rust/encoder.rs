//! Protocol Buffers Encoder
//! 
//! Provides high-performance encoding of Protocol Buffer messages

use napi::bindgen_prelude::*;
use napi_derive::napi;

/// High-performance Protocol Buffer encoder
#[napi]
pub struct Encoder {
    buffer: Vec<u8>,
}

#[napi]
impl Encoder {
    /// Create a new encoder
    #[napi(constructor)]
    pub fn new() -> Self {
        Encoder {
            buffer: Vec::with_capacity(1024),
        }
    }

    /// Create a new encoder with specified capacity
    #[napi(factory)]
    pub fn with_capacity(capacity: u32) -> Self {
        Encoder {
            buffer: Vec::with_capacity(capacity as usize),
        }
    }

    /// Encode a varint value
    #[napi]
    pub fn encode_varint(&mut self, value: i64) -> Result<()> {
        let mut val = value as u64;
        
        loop {
            let mut byte = (val & 0x7F) as u8;
            val >>= 7;
            
            if val != 0 {
                byte |= 0x80;
            }
            
            self.buffer.push(byte);
            
            if val == 0 {
                break;
            }
        }
        
        Ok(())
    }

    /// Encode a 32-bit unsigned integer
    #[napi]
    pub fn encode_uint32(&mut self, value: u32) -> Result<()> {
        self.encode_varint(value as i64)
    }

    /// Encode a 32-bit signed integer
    #[napi]
    pub fn encode_int32(&mut self, value: i32) -> Result<()> {
        self.encode_varint(value as i64)
    }

    /// Encode a 32-bit signed integer with ZigZag encoding
    #[napi]
    pub fn encode_sint32(&mut self, value: i32) -> Result<()> {
        let zigzag = ((value << 1) ^ (value >> 31)) as u32;
        self.encode_varint(zigzag as i64)
    }

    /// Encode a 64-bit unsigned integer
    #[napi]
    pub fn encode_uint64(&mut self, value: i64) -> Result<()> {
        self.encode_varint(value)
    }

    /// Encode a 64-bit signed integer
    #[napi]
    pub fn encode_int64(&mut self, value: i64) -> Result<()> {
        self.encode_varint(value)
    }

    /// Encode a 64-bit signed integer with ZigZag encoding
    #[napi]
    pub fn encode_sint64(&mut self, value: i64) -> Result<()> {
        let zigzag = ((value << 1) ^ (value >> 63)) as u64;
        self.encode_varint(zigzag as i64)
    }

    /// Encode a boolean value
    #[napi]
    pub fn encode_bool(&mut self, value: bool) -> Result<()> {
        self.buffer.push(if value { 1 } else { 0 });
        Ok(())
    }

    /// Encode a fixed 32-bit value
    #[napi]
    pub fn encode_fixed32(&mut self, value: u32) -> Result<()> {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    /// Encode a fixed 64-bit value
    #[napi]
    pub fn encode_fixed64(&mut self, value: i64) -> Result<()> {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    /// Encode a 32-bit floating point value
    #[napi]
    pub fn encode_float(&mut self, value: f64) -> Result<()> {
        let float_val = value as f32;
        self.buffer.extend_from_slice(&float_val.to_le_bytes());
        Ok(())
    }

    /// Encode a 64-bit floating point value
    #[napi]
    pub fn encode_double(&mut self, value: f64) -> Result<()> {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    /// Encode a byte array with length prefix
    #[napi]
    pub fn encode_bytes(&mut self, value: Buffer) -> Result<()> {
        let bytes = value.as_ref();
        self.encode_varint(bytes.len() as i64)?;
        self.buffer.extend_from_slice(bytes);
        Ok(())
    }

    /// Encode a UTF-8 string with length prefix
    #[napi]
    pub fn encode_string(&mut self, value: String) -> Result<()> {
        let bytes = value.as_bytes();
        self.encode_varint(bytes.len() as i64)?;
        self.buffer.extend_from_slice(bytes);
        Ok(())
    }

    /// Encode a field tag
    #[napi]
    pub fn encode_tag(&mut self, field_number: u32, wire_type: u32) -> Result<()> {
        if wire_type > 5 {
            return Err(Error::from_reason(format!(
                "Invalid wire type: {}. Must be 0-5",
                wire_type
            )));
        }
        
        let tag = (field_number << 3) | wire_type;
        self.encode_varint(tag as i64)
    }

    /// Get the current size of the encoded buffer
    #[napi]
    pub fn size(&self) -> u32 {
        self.buffer.len() as u32
    }

    /// Get the encoded buffer
    #[napi]
    pub fn finish(&mut self) -> Buffer {
        self.buffer.clone().into()
    }

    /// Reset the encoder for reuse
    #[napi]
    pub fn reset(&mut self) {
        self.buffer.clear();
    }
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder_varint() {
        let mut encoder = Encoder::new();
        encoder.encode_varint(300).unwrap();
        let result = encoder.finish();
        assert_eq!(result.as_ref(), &[0xAC, 0x02]);
    }

    #[test]
    fn test_encoder_string() {
        let mut encoder = Encoder::new();
        encoder.encode_string("hello".to_string()).unwrap();
        let result = encoder.finish();
        // Length (5) followed by "hello"
        assert_eq!(result.as_ref()[0], 5);
        assert_eq!(&result.as_ref()[1..], b"hello");
    }
}
