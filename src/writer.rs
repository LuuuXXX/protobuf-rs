use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Writer for protobuf wire format with buffer optimization
#[napi]
pub struct Writer {
    buffer: Vec<u8>,
}

#[napi]
impl Writer {
    /// Create a new Writer
    #[napi(constructor)]
    pub fn new() -> Self {
        Writer { buffer: Vec::new() }
    }

    /// Create a new Writer with pre-allocated capacity
    #[napi(factory)]
    pub fn with_capacity(capacity: u32) -> Self {
        Writer {
            buffer: Vec::with_capacity(capacity as usize),
        }
    }

    /// Write a u32 as varint
    #[napi]
    pub fn uint32(&mut self, value: u32) {
        let mut n = value;
        loop {
            let mut byte = (n & 0x7F) as u8;
            n >>= 7;

            if n != 0 {
                byte |= 0x80;
            }

            self.buffer.push(byte);

            if n == 0 {
                break;
            }
        }
    }

    /// Write bytes with length prefix
    #[napi]
    pub fn bytes(&mut self, value: Buffer) {
        let bytes = value.as_ref();
        self.uint32(bytes.len() as u32);
        self.buffer.extend_from_slice(bytes);
    }

    /// Write string with length prefix
    #[napi]
    pub fn string(&mut self, value: String) {
        let bytes = value.as_bytes();
        self.uint32(bytes.len() as u32);
        self.buffer.extend_from_slice(bytes);
    }

    /// Get the finished buffer
    #[napi]
    pub fn finish(&self) -> Buffer {
        self.buffer.clone().into()
    }

    /// Get estimated buffer size (current size)
    #[napi]
    pub fn estimated_size(&self) -> u32 {
        self.buffer.len() as u32
    }

    /// Reset the writer for reuse
    #[napi]
    pub fn reset(&mut self) {
        self.buffer.clear();
    }

    /// Get current length
    #[napi]
    pub fn len(&self) -> u32 {
        self.buffer.len() as u32
    }

    /// Check if empty
    #[napi]
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
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
        let buffer = writer.finish();
        assert_eq!(buffer.as_ref(), &[0xAC, 0x02]);
    }

    #[test]
    fn test_writer_string() {
        let mut writer = Writer::new();
        writer.string("hello".to_string());
        let buffer = writer.finish();
        // Length prefix (5) + "hello"
        assert_eq!(buffer.as_ref(), &[5, b'h', b'e', b'l', b'l', b'o']);
    }

    #[test]
    fn test_writer_reset() {
        let mut writer = Writer::new();
        writer.uint32(123);
        assert!(!writer.is_empty());

        writer.reset();
        assert!(writer.is_empty());
        assert_eq!(writer.len(), 0);
    }

    #[test]
    fn test_writer_with_capacity() {
        let writer = Writer::with_capacity(100);
        assert_eq!(writer.len(), 0);
        assert!(writer.buffer.capacity() >= 100);
    }

    #[test]
    fn test_writer_chaining() {
        let mut writer = Writer::new();
        writer.uint32(100);
        writer.uint32(200);
        writer.uint32(300);
        assert!(writer.len() > 0);
    }
}
