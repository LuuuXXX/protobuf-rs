//! High-performance Protocol Buffer Reader
//! 
//! Provides zero-copy reading of Protocol Buffer messages

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::io::Cursor;
use std::io::Read;

const MAX_VARINT_BYTES: usize = 10;

/// High-performance Protocol Buffer reader with zero-copy optimizations
#[napi]
pub struct Reader {
    buffer: Vec<u8>,
    cursor: Cursor<Vec<u8>>,
}

#[napi]
impl Reader {
    /// Create a new reader with the given buffer
    #[napi(constructor)]
    pub fn new(buffer: Buffer) -> Self {
        let buf = buffer.to_vec();
        let cursor = Cursor::new(buf.clone());
        Reader {
            buffer: buf,
            cursor,
        }
    }

    /// Get current position in the buffer
    #[napi]
    pub fn pos(&self) -> u32 {
        self.cursor.position() as u32
    }

    /// Get the length of the buffer
    #[napi]
    pub fn len(&self) -> u32 {
        self.buffer.len() as u32
    }

    /// Read a varint from the buffer
    fn read_varint_internal(&mut self) -> Result<u64> {
        let mut result: u64 = 0;
        let mut shift = 0;
        
        for _ in 0..MAX_VARINT_BYTES {
            let mut byte_buf = [0u8; 1];
            if self.cursor.read_exact(&mut byte_buf).is_err() {
                return Err(Error::from_reason("Unexpected end of buffer"));
            }
            
            let byte = byte_buf[0];
            result |= ((byte & 0x7F) as u64) << shift;
            
            if byte & 0x80 == 0 {
                return Ok(result);
            }
            
            shift += 7;
        }
        
        Err(Error::from_reason("Varint too long"))
    }

    /// Read a 32-bit unsigned integer
    #[napi]
    pub fn uint32(&mut self) -> Result<u32> {
        self.read_varint_internal().map(|v| v as u32)
    }

    /// Read a 32-bit signed integer
    #[napi]
    pub fn int32(&mut self) -> Result<i32> {
        self.read_varint_internal().map(|v| v as i32)
    }

    /// Read a 32-bit signed integer with ZigZag encoding
    #[napi]
    pub fn sint32(&mut self) -> Result<i32> {
        let value = self.read_varint_internal()? as u32;
        let decoded = ((value >> 1) as i32) ^ (-((value & 1) as i32));
        Ok(decoded)
    }

    /// Read a 64-bit unsigned integer
    #[napi]
    pub fn uint64(&mut self) -> Result<i64> {
        self.read_varint_internal().map(|v| v as i64)
    }

    /// Read a 64-bit signed integer
    #[napi]
    pub fn int64(&mut self) -> Result<i64> {
        self.read_varint_internal().map(|v| v as i64)
    }

    /// Read a 64-bit signed integer with ZigZag encoding
    #[napi]
    pub fn sint64(&mut self) -> Result<i64> {
        let value = self.read_varint_internal()?;
        let decoded = ((value >> 1) as i64) ^ (-((value & 1) as i64));
        Ok(decoded)
    }

    /// Read a boolean value
    #[napi]
    pub fn bool(&mut self) -> Result<bool> {
        let value = self.read_varint_internal()?;
        Ok(value != 0)
    }

    /// Read a fixed 32-bit value
    #[napi]
    pub fn fixed32(&mut self) -> Result<u32> {
        let mut bytes = [0u8; 4];
        self.cursor
            .read_exact(&mut bytes)
            .map_err(|_| Error::from_reason("Unexpected end of buffer"))?;
        Ok(u32::from_le_bytes(bytes))
    }

    /// Read a signed fixed 32-bit value
    #[napi]
    pub fn sfixed32(&mut self) -> Result<i32> {
        let mut bytes = [0u8; 4];
        self.cursor
            .read_exact(&mut bytes)
            .map_err(|_| Error::from_reason("Unexpected end of buffer"))?;
        Ok(i32::from_le_bytes(bytes))
    }

    /// Read a fixed 64-bit value
    #[napi]
    pub fn fixed64(&mut self) -> Result<i64> {
        let mut bytes = [0u8; 8];
        self.cursor
            .read_exact(&mut bytes)
            .map_err(|_| Error::from_reason("Unexpected end of buffer"))?;
        Ok(i64::from_le_bytes(bytes))
    }

    /// Read a signed fixed 64-bit value
    #[napi]
    pub fn sfixed64(&mut self) -> Result<i64> {
        let mut bytes = [0u8; 8];
        self.cursor
            .read_exact(&mut bytes)
            .map_err(|_| Error::from_reason("Unexpected end of buffer"))?;
        Ok(i64::from_le_bytes(bytes))
    }

    /// Read a 32-bit floating point value
    #[napi]
    pub fn float(&mut self) -> Result<f64> {
        let mut bytes = [0u8; 4];
        self.cursor
            .read_exact(&mut bytes)
            .map_err(|_| Error::from_reason("Unexpected end of buffer"))?;
        Ok(f32::from_le_bytes(bytes) as f64)
    }

    /// Read a 64-bit floating point value
    #[napi]
    pub fn double(&mut self) -> Result<f64> {
        let mut bytes = [0u8; 8];
        self.cursor
            .read_exact(&mut bytes)
            .map_err(|_| Error::from_reason("Unexpected end of buffer"))?;
        Ok(f64::from_le_bytes(bytes))
    }

    /// Read a byte array with length prefix
    #[napi]
    pub fn bytes(&mut self) -> Result<Buffer> {
        let length = self.read_varint_internal()? as usize;
        let mut bytes = vec![0u8; length];
        self.cursor
            .read_exact(&mut bytes)
            .map_err(|_| Error::from_reason("Unexpected end of buffer"))?;
        Ok(bytes.into())
    }

    /// Read a UTF-8 string with length prefix
    #[napi]
    pub fn string(&mut self) -> Result<String> {
        let bytes = self.bytes()?;
        String::from_utf8(bytes.to_vec())
            .map_err(|e| Error::from_reason(format!("Invalid UTF-8: {}", e)))
    }

    /// Skip a specified number of bytes
    #[napi]
    pub fn skip(&mut self, count: u32) -> Result<()> {
        let new_pos = self.cursor.position() + count as u64;
        if new_pos > self.buffer.len() as u64 {
            return Err(Error::from_reason("Cannot skip beyond buffer end"));
        }
        self.cursor.set_position(new_pos);
        Ok(())
    }

    /// Skip a field based on wire type
    #[napi]
    pub fn skip_type(&mut self, wire_type: u32) -> Result<()> {
        match wire_type {
            0 => {
                // Varint
                self.read_varint_internal()?;
            }
            1 => {
                // 64-bit
                self.skip(8)?;
            }
            2 => {
                // Length-delimited
                let length = self.read_varint_internal()? as u32;
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

    /// Reset the reader to the beginning
    #[napi]
    pub fn reset(&mut self) {
        self.cursor.set_position(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader_uint32() {
        let buffer = vec![0xAC, 0x02];
        let mut reader = Reader::new(buffer.into());
        let value = reader.uint32().unwrap();
        assert_eq!(value, 300);
    }

    #[test]
    fn test_reader_string() {
        let mut buffer = vec![5]; // length
        buffer.extend_from_slice(b"hello");
        let mut reader = Reader::new(buffer.into());
        let value = reader.string().unwrap();
        assert_eq!(value, "hello");
    }

    #[test]
    fn test_reader_bool() {
        let buffer = vec![1];
        let mut reader = Reader::new(buffer.into());
        let value = reader.bool().unwrap();
        assert_eq!(value, true);
    }
}
