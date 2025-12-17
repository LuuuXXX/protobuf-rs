use napi::Result;
use napi::Error;
use napi::Status;

pub struct ReaderImpl {
    buf: Vec<u8>,
    pos: usize,
}

impl ReaderImpl {
    pub fn new(buf: Vec<u8>) -> Self {
        ReaderImpl { buf, pos: 0 }
    }

    #[inline]
    fn check_bounds(&self, len: usize) -> Result<()> {
        if self.pos + len > self.buf.len() {
            Err(Error::new(
                Status::GenericFailure,
                format!("index out of range: {} + {} > {}", self.pos, len, self.buf.len()),
            ))
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn read_varint32(&mut self) -> Result<u32> {
        let mut value: u32 = 0;
        let mut shift = 0;

        loop {
            self.check_bounds(1)?;
            let byte = self.buf[self.pos];
            self.pos += 1;

            if shift == 28 {
                // Last byte, only use lower 4 bits
                value |= ((byte & 0x0F) as u32) << shift;
                if byte < 0x80 {
                    return Ok(value);
                }
                // Skip remaining bytes if continuation bit is set
                for _ in 0..5 {
                    if self.pos >= self.buf.len() {
                        return Err(Error::new(
                            Status::GenericFailure,
                            format!("index out of range: {} + 10 > {}", self.pos - 1, self.buf.len()),
                        ));
                    }
                    if self.buf[self.pos] < 0x80 {
                        self.pos += 1;
                        return Ok(value);
                    }
                    self.pos += 1;
                }
                return Ok(value);
            }

            value |= ((byte & 0x7F) as u32) << shift;
            if byte < 0x80 {
                return Ok(value);
            }
            shift += 7;
        }
    }

    #[inline]
    pub fn read_varint64(&mut self) -> Result<u64> {
        let mut value: u64 = 0;
        let mut shift = 0;

        loop {
            self.check_bounds(1)?;
            let byte = self.buf[self.pos];
            self.pos += 1;

            if shift == 63 {
                // Last byte
                value |= ((byte & 0x01) as u64) << shift;
                return Ok(value);
            }

            value |= ((byte & 0x7F) as u64) << shift;
            if byte < 0x80 {
                return Ok(value);
            }
            shift += 7;

            if shift > 63 {
                return Ok(value);
            }
        }
    }

    #[inline]
    pub fn read_sint32(&mut self) -> Result<i32> {
        let value = self.read_varint32()?;
        Ok(((value >> 1) as i32) ^ (-((value & 1) as i32)))
    }

    #[inline]
    pub fn read_sint64(&mut self) -> Result<i64> {
        let value = self.read_varint64()?;
        Ok(((value >> 1) as i64) ^ (-((value & 1) as i64)))
    }

    #[inline]
    pub fn read_fixed32(&mut self) -> Result<u32> {
        self.check_bounds(4)?;
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 4]);
        self.pos += 4;
        Ok(u32::from_le_bytes(bytes))
    }

    #[inline]
    pub fn read_fixed64(&mut self) -> Result<u64> {
        self.check_bounds(8)?;
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 8]);
        self.pos += 8;
        Ok(u64::from_le_bytes(bytes))
    }

    #[inline]
    pub fn read_sfixed32(&mut self) -> Result<i32> {
        self.check_bounds(4)?;
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 4]);
        self.pos += 4;
        Ok(i32::from_le_bytes(bytes))
    }

    #[inline]
    pub fn read_sfixed64(&mut self) -> Result<i64> {
        self.check_bounds(8)?;
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 8]);
        self.pos += 8;
        Ok(i64::from_le_bytes(bytes))
    }

    #[inline]
    pub fn read_float(&mut self) -> Result<f32> {
        self.check_bounds(4)?;
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 4]);
        self.pos += 4;
        Ok(f32::from_le_bytes(bytes))
    }

    #[inline]
    pub fn read_double(&mut self) -> Result<f64> {
        self.check_bounds(8)?;
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 8]);
        self.pos += 8;
        Ok(f64::from_le_bytes(bytes))
    }

    #[inline]
    pub fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>> {
        self.check_bounds(len)?;
        let bytes = self.buf[self.pos..self.pos + len].to_vec();
        self.pos += len;
        Ok(bytes)
    }

    #[inline]
    pub fn skip(&mut self, len: usize) -> Result<()> {
        self.check_bounds(len)?;
        self.pos += len;
        Ok(())
    }

    pub fn skip_type(&mut self, wire_type: u32) -> Result<()> {
        match wire_type {
            0 => {
                // Varint
                loop {
                    self.check_bounds(1)?;
                    let byte = self.buf[self.pos];
                    self.pos += 1;
                    if byte < 0x80 {
                        break;
                    }
                }
                Ok(())
            }
            1 => {
                // 64-bit
                self.skip(8)
            }
            2 => {
                // Length-delimited
                let len = self.read_varint32()? as usize;
                self.skip(len)
            }
            3 => {
                // Start group (deprecated) - read until end group (wire type 4)
                loop {
                    let field_and_type = self.read_varint32()?;
                    let wire_type = field_and_type & 7;
                    if wire_type == 4 {
                        // End group
                        break;
                    }
                    self.skip_type(wire_type)?;
                }
                Ok(())
            }
            5 => {
                // 32-bit
                self.skip(4)
            }
            _ => Err(Error::new(
                Status::GenericFailure,
                format!("invalid wire type {} at offset {}", wire_type, self.pos),
            )),
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }
}
