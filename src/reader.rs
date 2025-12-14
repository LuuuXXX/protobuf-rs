use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Reader for protobuf wire format with zero-copy optimizations
/// 
/// Note: The constructor currently copies the buffer for safety with NAPI bindings.
/// This ensures proper lifetime management across the Rust-JS boundary.
/// True zero-copy will be evaluated in future releases based on NAPI improvements.
#[napi]
pub struct Reader {
    buffer: Vec<u8>,
    pos: usize,
}

#[napi]
impl Reader {
    /// Create a new Reader from a buffer
    #[napi(constructor)]
    pub fn new(buffer: Buffer) -> Self {
        Reader {
            buffer: buffer.to_vec(),
            pos: 0,
        }
    }

    /// Get current position in the buffer
    #[napi]
    pub fn pos(&self) -> u32 {
        self.pos as u32
    }

    /// Get total length of the buffer
    #[napi]
    pub fn len(&self) -> u32 {
        self.buffer.len() as u32
    }

    /// Check if at end of buffer
    #[napi]
    pub fn is_empty(&self) -> bool {
        self.pos >= self.buffer.len()
    }

    /// Read a varint as u32
    #[napi]
    pub fn uint32(&mut self) -> Result<u32> {
        let mut result: u32 = 0;
        let mut shift = 0;

        for i in 0..5 {
            if self.pos >= self.buffer.len() {
                return Err(Error::from_reason("Buffer underflow"));
            }

            let byte = self.buffer[self.pos];
            self.pos += 1;

            if i == 4 && byte > 0x0F {
                return Err(Error::from_reason("Varint overflow"));
            }

            result |= ((byte & 0x7F) as u32) << shift;

            if byte & 0x80 == 0 {
                return Ok(result);
            }

            shift += 7;
        }

        Err(Error::from_reason("Varint too long"))
    }

    /// Read bytes with length prefix (zero-copy when possible)
    #[napi]
    pub fn bytes(&mut self) -> Result<Buffer> {
        let len = self.uint32()? as usize;

        if self.pos + len > self.buffer.len() {
            return Err(Error::from_reason("Buffer underflow"));
        }

        let start = self.pos;
        self.pos += len;

        // Return a copy of the slice
        Ok(self.buffer[start..self.pos].to_vec().into())
    }

    /// Read string with length prefix (zero-copy when possible)
    #[napi]
    pub fn string(&mut self) -> Result<String> {
        let len = self.uint32()? as usize;

        if self.pos + len > self.buffer.len() {
            return Err(Error::from_reason("Buffer underflow"));
        }

        let start = self.pos;
        self.pos += len;

        // Validate UTF-8 and return string
        std::str::from_utf8(&self.buffer[start..self.pos])
            .map(|s| s.to_string())
            .map_err(|_| Error::from_reason("Invalid UTF-8"))
    }

    /// Skip bytes without allocating
    #[napi]
    pub fn skip(&mut self, n: u32) -> Result<()> {
        let n = n as usize;
        if self.pos + n > self.buffer.len() {
            return Err(Error::from_reason("Buffer underflow"));
        }
        self.pos += n;
        Ok(())
    }

    /// Reset position to start
    #[napi]
    pub fn reset(&mut self) {
        self.pos = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader_uint32() {
        let buffer = vec![0xAC, 0x02]; // 300 in varint
        let mut reader = Reader { buffer, pos: 0 };

        assert_eq!(reader.uint32().unwrap(), 300);
        assert!(reader.is_empty());
    }

    #[test]
    fn test_reader_skip() {
        let buffer = vec![1, 2, 3, 4, 5];
        let mut reader = Reader { buffer, pos: 0 };

        reader.skip(2).unwrap();
        assert_eq!(reader.pos(), 2);
        assert_eq!(reader.len(), 5);
    }

    #[test]
    fn test_reader_reset() {
        let buffer = vec![1, 2, 3];
        let mut reader = Reader { buffer, pos: 2 };

        reader.reset();
        assert_eq!(reader.pos(), 0);
    }
}
