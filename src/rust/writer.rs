//! High-performance Protocol Buffer Writer
//! 
//! Provides efficient writing of Protocol Buffer messages with buffer management

use napi::bindgen_prelude::*;
use napi_derive::napi;

/// High-performance Protocol Buffer writer with optimized buffer management
#[napi]
pub struct Writer {
    buffer: Vec<u8>,
    stack: Vec<usize>, // Stack for tracking fork positions
}

#[napi]
impl Writer {
    /// Create a new writer
    #[napi(constructor)]
    pub fn new() -> Self {
        Writer {
            buffer: Vec::with_capacity(1024),
            stack: Vec::new(),
        }
    }

    /// Create a new writer with specified capacity
    #[napi(factory)]
    pub fn with_capacity(capacity: u32) -> Self {
        Writer {
            buffer: Vec::with_capacity(capacity as usize),
            stack: Vec::new(),
        }
    }

    /// Write a varint to the buffer
    fn write_varint_internal(&mut self, mut value: u64) {
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            
            if value != 0 {
                byte |= 0x80;
            }
            
            self.buffer.push(byte);
            
            if value == 0 {
                break;
            }
        }
    }

    /// Write a 32-bit unsigned integer
    #[napi]
    pub fn uint32(&mut self, value: u32) -> &Self {
        self.write_varint_internal(value as u64);
        self
    }

    /// Write a 32-bit signed integer
    #[napi]
    pub fn int32(&mut self, value: i32) -> &Self {
        if value >= 0 {
            self.write_varint_internal(value as u64);
        } else {
            // Negative numbers need full 10 bytes
            self.write_varint_internal((value as i64) as u64);
        }
        self
    }

    /// Write a 32-bit signed integer with ZigZag encoding
    #[napi]
    pub fn sint32(&mut self, value: i32) -> &Self {
        let zigzag = ((value << 1) ^ (value >> 31)) as u32;
        self.write_varint_internal(zigzag as u64);
        self
    }

    /// Write a 64-bit unsigned integer
    #[napi]
    pub fn uint64(&mut self, value: i64) -> &Self {
        self.write_varint_internal(value as u64);
        self
    }

    /// Write a 64-bit signed integer
    #[napi]
    pub fn int64(&mut self, value: i64) -> &Self {
        self.write_varint_internal(value as u64);
        self
    }

    /// Write a 64-bit signed integer with ZigZag encoding
    #[napi]
    pub fn sint64(&mut self, value: i64) -> &Self {
        let zigzag = ((value << 1) ^ (value >> 63)) as u64;
        self.write_varint_internal(zigzag);
        self
    }

    /// Write a boolean value
    #[napi]
    pub fn bool(&mut self, value: bool) -> &Self {
        self.buffer.push(if value { 1 } else { 0 });
        self
    }

    /// Write a fixed 32-bit value
    #[napi]
    pub fn fixed32(&mut self, value: u32) -> &Self {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        self
    }

    /// Write a signed fixed 32-bit value
    #[napi]
    pub fn sfixed32(&mut self, value: i32) -> &Self {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        self
    }

    /// Write a fixed 64-bit value
    #[napi]
    pub fn fixed64(&mut self, value: i64) -> &Self {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        self
    }

    /// Write a signed fixed 64-bit value
    #[napi]
    pub fn sfixed64(&mut self, value: i64) -> &Self {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        self
    }

    /// Write a 32-bit floating point value
    #[napi]
    pub fn float(&mut self, value: f64) -> &Self {
        let float_val = value as f32;
        self.buffer.extend_from_slice(&float_val.to_le_bytes());
        self
    }

    /// Write a 64-bit floating point value
    #[napi]
    pub fn double(&mut self, value: f64) -> &Self {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        self
    }

    /// Write a byte array with length prefix
    #[napi]
    pub fn bytes(&mut self, value: Buffer) -> &Self {
        let bytes = value.as_ref();
        self.write_varint_internal(bytes.len() as u64);
        self.buffer.extend_from_slice(bytes);
        self
    }

    /// Write a UTF-8 string with length prefix
    #[napi]
    pub fn string(&mut self, value: String) -> &Self {
        let bytes = value.as_bytes();
        self.write_varint_internal(bytes.len() as u64);
        self.buffer.extend_from_slice(bytes);
        self
    }

    /// Write a field tag
    #[napi]
    pub fn tag(&mut self, field_number: u32, wire_type: u32) -> Result<&Self> {
        if wire_type > 5 {
            return Err(Error::from_reason(format!(
                "Invalid wire type: {}. Must be 0-5",
                wire_type
            )));
        }
        
        let tag = (field_number << 3) | wire_type;
        self.write_varint_internal(tag as u64);
        Ok(self)
    }

    /// Fork the writer to create a length-delimited section
    /// Returns the current position to be used with ldelim()
    #[napi]
    pub fn fork(&mut self) -> u32 {
        let pos = self.buffer.len();
        self.stack.push(pos);
        // Reserve space for length varint (max 5 bytes for 32-bit length)
        self.buffer.extend_from_slice(&[0, 0, 0, 0, 0]);
        pos as u32
    }

    /// Complete a length-delimited section started with fork()
    #[napi]
    pub fn ldelim(&mut self) -> Result<&Self> {
        if self.stack.is_empty() {
            return Err(Error::from_reason("No fork to delimit"));
        }
        
        let fork_pos = self.stack.pop().unwrap();
        let current_pos = self.buffer.len();
        let length = current_pos - fork_pos - 5; // Subtract the reserved 5 bytes
        
        // Encode the length
        let mut length_bytes = Vec::new();
        let mut len = length as u64;
        loop {
            let mut byte = (len & 0x7F) as u8;
            len >>= 7;
            if len != 0 {
                byte |= 0x80;
            }
            length_bytes.push(byte);
            if len == 0 {
                break;
            }
        }
        
        // Insert the actual length at the fork position
        let length_size = length_bytes.len();
        if length_size > 5 {
            return Err(Error::from_reason("Length too large"));
        }
        
        // Copy the data after the reserved space to close the gap
        let data_start = fork_pos + 5;
        let data = self.buffer[data_start..current_pos].to_vec();
        
        // Rebuild: fork_pos + length_bytes + data
        self.buffer.truncate(fork_pos);
        self.buffer.extend_from_slice(&length_bytes);
        self.buffer.extend_from_slice(&data);
        
        Ok(self)
    }

    /// Get the current length of the buffer
    #[napi]
    pub fn len(&self) -> u32 {
        self.buffer.len() as u32
    }

    /// Get the encoded buffer
    #[napi]
    pub fn finish(&mut self) -> Buffer {
        self.buffer.clone().into()
    }

    /// Reset the writer for reuse
    #[napi]
    pub fn reset(&mut self) {
        self.buffer.clear();
        self.stack.clear();
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
    fn test_writer_uint32() {
        let mut writer = Writer::new();
        writer.uint32(300);
        let result = writer.finish();
        assert_eq!(result.as_ref(), &[0xAC, 0x02]);
    }

    #[test]
    fn test_writer_string() {
        let mut writer = Writer::new();
        writer.string("hello".to_string());
        let result = writer.finish();
        // Length (5) followed by "hello"
        assert_eq!(result.as_ref()[0], 5);
        assert_eq!(&result.as_ref()[1..], b"hello");
    }

    #[test]
    fn test_writer_bool() {
        let mut writer = Writer::new();
        writer.bool(true);
        let result = writer.finish();
        assert_eq!(result.as_ref(), &[1]);
    }
}
