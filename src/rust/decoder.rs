//! Protocol Buffers Decoder
//! 
//! Provides high-performance decoding of Protocol Buffer messages

use napi::bindgen_prelude::*;
use napi_derive::napi;

const MAX_VARINT_BYTES: usize = 10;

/// High-performance Protocol Buffer decoder
#[napi]
pub struct Decoder {
    buffer: Vec<u8>,
    position: usize,
}

#[napi]
impl Decoder {
    /// Create a new decoder with the given buffer
    #[napi(constructor)]
    pub fn new(buffer: Buffer) -> Self {
        Decoder {
            buffer: buffer.to_vec(),
            position: 0,
        }
    }

    /// Get current position in the buffer
    #[napi]
    pub fn position(&self) -> u32 {
        self.position as u32
    }

    /// Get remaining bytes in the buffer
    #[napi]
    pub fn remaining(&self) -> u32 {
        (self.buffer.len() - self.position) as u32
    }

    /// Check if there are more bytes to read
    #[napi]
    pub fn has_more(&self) -> bool {
        self.position < self.buffer.len()
    }

    /// Decode a varint value
    #[napi]
    pub fn decode_varint(&mut self) -> Result<i64> {
        let mut result: u64 = 0;
        let mut shift = 0;
        
        for i in 0..MAX_VARINT_BYTES {
            if self.position >= self.buffer.len() {
                return Err(Error::from_reason("Unexpected end of buffer"));
            }
            
            let byte = self.buffer[self.position];
            self.position += 1;
            
            result |= ((byte & 0x7F) as u64) << shift;
            
            if byte & 0x80 == 0 {
                return Ok(result as i64);
            }
            
            shift += 7;
            
            if i == MAX_VARINT_BYTES - 1 {
                return Err(Error::from_reason("Varint too long"));
            }
        }
        
        Err(Error::from_reason("Invalid varint"))
    }

    /// Decode a 32-bit unsigned integer
    #[napi]
    pub fn decode_uint32(&mut self) -> Result<u32> {
        let value = self.decode_varint()?;
        Ok(value as u32)
    }

    /// Decode a 32-bit signed integer
    #[napi]
    pub fn decode_int32(&mut self) -> Result<i32> {
        let value = self.decode_varint()?;
        Ok(value as i32)
    }

    /// Decode a 32-bit signed integer with ZigZag encoding
    #[napi]
    pub fn decode_sint32(&mut self) -> Result<i32> {
        let value = self.decode_varint()? as u32;
        let decoded = ((value >> 1) as i32) ^ (-((value & 1) as i32));
        Ok(decoded)
    }

    /// Decode a 64-bit unsigned integer
    #[napi]
    pub fn decode_uint64(&mut self) -> Result<i64> {
        self.decode_varint()
    }

    /// Decode a 64-bit signed integer
    #[napi]
    pub fn decode_int64(&mut self) -> Result<i64> {
        self.decode_varint()
    }

    /// Decode a 64-bit signed integer with ZigZag encoding
    #[napi]
    pub fn decode_sint64(&mut self) -> Result<i64> {
        let value = self.decode_varint()? as u64;
        let decoded = ((value >> 1) as i64) ^ (-((value & 1) as i64));
        Ok(decoded)
    }

    /// Decode a boolean value
    #[napi]
    pub fn decode_bool(&mut self) -> Result<bool> {
        if self.position >= self.buffer.len() {
            return Err(Error::from_reason("Unexpected end of buffer"));
        }
        
        let value = self.buffer[self.position];
        self.position += 1;
        Ok(value != 0)
    }

    /// Decode a fixed 32-bit value
    #[napi]
    pub fn decode_fixed32(&mut self) -> Result<u32> {
        if self.position + 4 > self.buffer.len() {
            return Err(Error::from_reason("Unexpected end of buffer"));
        }
        
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.buffer[self.position..self.position + 4]);
        self.position += 4;
        Ok(u32::from_le_bytes(bytes))
    }

    /// Decode a fixed 64-bit value
    #[napi]
    pub fn decode_fixed64(&mut self) -> Result<i64> {
        if self.position + 8 > self.buffer.len() {
            return Err(Error::from_reason("Unexpected end of buffer"));
        }
        
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.buffer[self.position..self.position + 8]);
        self.position += 8;
        Ok(i64::from_le_bytes(bytes))
    }

    /// Decode a 32-bit floating point value
    #[napi]
    pub fn decode_float(&mut self) -> Result<f64> {
        if self.position + 4 > self.buffer.len() {
            return Err(Error::from_reason("Unexpected end of buffer"));
        }
        
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.buffer[self.position..self.position + 4]);
        self.position += 4;
        Ok(f32::from_le_bytes(bytes) as f64)
    }

    /// Decode a 64-bit floating point value
    #[napi]
    pub fn decode_double(&mut self) -> Result<f64> {
        if self.position + 8 > self.buffer.len() {
            return Err(Error::from_reason("Unexpected end of buffer"));
        }
        
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.buffer[self.position..self.position + 8]);
        self.position += 8;
        Ok(f64::from_le_bytes(bytes))
    }

    /// Decode a byte array with length prefix
    #[napi]
    pub fn decode_bytes(&mut self) -> Result<Buffer> {
        let length = self.decode_varint()? as usize;
        
        if self.position + length > self.buffer.len() {
            return Err(Error::from_reason("Unexpected end of buffer"));
        }
        
        let bytes = self.buffer[self.position..self.position + length].to_vec();
        self.position += length;
        Ok(bytes.into())
    }

    /// Decode a UTF-8 string with length prefix
    #[napi]
    pub fn decode_string(&mut self) -> Result<String> {
        let bytes = self.decode_bytes()?;
        String::from_utf8(bytes.to_vec())
            .map_err(|e| Error::from_reason(format!("Invalid UTF-8: {}", e)))
    }

    /// Decode a field tag
    #[napi]
    pub fn decode_tag(&mut self) -> Result<Vec<u32>> {
        let tag = self.decode_varint()? as u32;
        let field_number = tag >> 3;
        let wire_type = tag & 0x7;
        Ok(vec![field_number, wire_type])
    }

    /// Skip a specified number of bytes
    #[napi]
    pub fn skip(&mut self, count: u32) -> Result<()> {
        let count = count as usize;
        if self.position + count > self.buffer.len() {
            return Err(Error::from_reason("Cannot skip beyond buffer end"));
        }
        self.position += count;
        Ok(())
    }

    /// Skip a field based on wire type
    #[napi]
    pub fn skip_field(&mut self, wire_type: u32) -> Result<()> {
        match wire_type {
            0 => {
                // Varint
                self.decode_varint()?;
            }
            1 => {
                // 64-bit
                self.skip(8)?;
            }
            2 => {
                // Length-delimited
                let length = self.decode_varint()? as u32;
                self.skip(length)?;
            }
            5 => {
                // 32-bit
                self.skip(4)?;
            }
            _ => {
                return Err(Error::from_reason(format!(
                    "Invalid wire type: {}",
                    wire_type
                )));
            }
        }
        Ok(())
    }

    /// Reset the decoder to the beginning
    #[napi]
    pub fn reset(&mut self) {
        self.position = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoder_varint() {
        let buffer = vec![0xAC, 0x02];
        let mut decoder = Decoder::new(buffer.into());
        let value = decoder.decode_varint().unwrap();
        assert_eq!(value, 300);
    }

    #[test]
    fn test_decoder_string() {
        let mut buffer = vec![5]; // length
        buffer.extend_from_slice(b"hello");
        let mut decoder = Decoder::new(buffer.into());
        let value = decoder.decode_string().unwrap();
        assert_eq!(value, "hello");
    }
}
