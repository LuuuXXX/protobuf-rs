#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use napi::Either;

mod writer;
mod reader;

use writer::WriterImpl;
use reader::ReaderImpl;

#[napi]
pub struct Writer {
    inner: WriterImpl,
}

#[napi]
impl Writer {
    #[napi(constructor)]
    pub fn new() -> Self {
        Writer {
            inner: WriterImpl::new(),
        }
    }

    #[napi]
    pub fn uint32(&mut self, value: u32) -> &Self {
        self.inner.write_varint32(value);
        self
    }

    #[napi]
    pub fn int32(&mut self, value: i32) -> &Self {
        if value < 0 {
            // Negative int32 is encoded as 10-byte varint
            self.inner.write_varint64(value as u64);
        } else {
            self.inner.write_varint32(value as u32);
        }
        self
    }

    #[napi]
    pub fn sint32(&mut self, value: i32) -> &Self {
        self.inner.write_sint32(value);
        self
    }

    #[napi]
    pub fn uint64(&mut self, value: Either<i64, Object>) -> &Self {
        let val = match value {
            Either::A(num) => num as u64,
            Either::B(obj) => {
                // Handle Long object with low and high parts
                let low: u32 = obj.get::<_, u32>("low").ok().flatten().unwrap_or(0);
                let high: u32 = obj.get::<_, u32>("high").ok().flatten().unwrap_or(0);
                ((high as u64) << 32) | (low as u64)
            }
        };
        self.inner.write_varint64(val);
        self
    }

    #[napi]
    pub fn int64(&mut self, value: Either<i64, Object>) -> &Self {
        let val = match value {
            Either::A(num) => num as u64,
            Either::B(obj) => {
                // Handle Long object with low and high parts
                let low: u32 = obj.get::<_, u32>("low").ok().flatten().unwrap_or(0);
                let high: i32 = obj.get::<_, i32>("high").ok().flatten().unwrap_or(0);
                (((high as i64) << 32) | (low as i64)) as u64
            }
        };
        self.inner.write_varint64(val);
        self
    }

    #[napi]
    pub fn sint64(&mut self, value: Either<i64, Object>) -> &Self {
        let val = match value {
            Either::A(num) => num,
            Either::B(obj) => {
                // Handle Long object with low and high parts
                let low: u32 = obj.get::<_, u32>("low").ok().flatten().unwrap_or(0);
                let high: i32 = obj.get::<_, i32>("high").ok().flatten().unwrap_or(0);
                ((high as i64) << 32) | (low as i64)
            }
        };
        self.inner.write_sint64(val);
        self
    }

    #[napi]
    pub fn bool(&mut self, value: bool) -> &Self {
        self.inner.write_varint32(if value { 1 } else { 0 });
        self
    }

    #[napi]
    pub fn fixed32(&mut self, value: u32) -> &Self {
        self.inner.write_fixed32(value);
        self
    }

    #[napi]
    pub fn sfixed32(&mut self, value: i32) -> &Self {
        self.inner.write_fixed32(value as u32);
        self
    }

    #[napi]
    pub fn fixed64(&mut self, value: Either<i64, Object>) -> &Self {
        let val = match value {
            Either::A(num) => num as u64,
            Either::B(obj) => {
                // Handle Long object with low and high parts
                let low: u32 = obj.get::<_, u32>("low").ok().flatten().unwrap_or(0);
                let high: u32 = obj.get::<_, u32>("high").ok().flatten().unwrap_or(0);
                ((high as u64) << 32) | (low as u64)
            }
        };
        self.inner.write_fixed64(val);
        self
    }

    #[napi]
    pub fn sfixed64(&mut self, value: Either<i64, Object>) -> &Self {
        let val = match value {
            Either::A(num) => num as u64,
            Either::B(obj) => {
                // Handle Long object with low and high parts
                let low: u32 = obj.get::<_, u32>("low").ok().flatten().unwrap_or(0);
                let high: i32 = obj.get::<_, i32>("high").ok().flatten().unwrap_or(0);
                (((high as i64) << 32) | (low as i64)) as u64
            }
        };
        self.inner.write_fixed64(val);
        self
    }

    #[napi]
    pub fn float(&mut self, value: f64) -> &Self {
        self.inner.write_float(value as f32);
        self
    }

    #[napi]
    pub fn double(&mut self, value: f64) -> &Self {
        self.inner.write_double(value);
        self
    }

    #[napi]
    pub fn bytes(&mut self, value: Buffer) -> &Self {
        self.inner.write_bytes(value.as_ref());
        self
    }

    #[napi]
    pub fn string(&mut self, value: String) -> &Self {
        let bytes = value.as_bytes();
        self.inner.write_varint32(bytes.len() as u32);
        self.inner.write_bytes(bytes);
        self
    }

    #[napi]
    pub fn fork(&mut self) -> &Self {
        self.inner.fork();
        self
    }

    #[napi]
    pub fn reset(&mut self) -> &Self {
        self.inner.reset();
        self
    }

    #[napi]
    pub fn ldelim(&mut self) -> &Self {
        self.inner.ldelim();
        self
    }

    #[napi]
    pub fn finish(&mut self) -> Buffer {
        let data = self.inner.finish();
        Buffer::from(data)
    }

    #[napi(getter)]
    pub fn len(&self) -> u32 {
        self.inner.len() as u32
    }
}

#[napi]
pub struct Reader {
    inner: ReaderImpl,
}

#[napi]
impl Reader {
    #[napi(constructor)]
    pub fn new(buffer: Buffer) -> Self {
        Reader {
            inner: ReaderImpl::new(buffer.to_vec()),
        }
    }

    #[napi]
    pub fn uint32(&mut self) -> Result<u32> {
        self.inner.read_varint32()
    }

    #[napi]
    pub fn int32(&mut self) -> Result<i32> {
        let value = self.inner.read_varint32()?;
        Ok(value as i32)
    }

    #[napi]
    pub fn sint32(&mut self) -> Result<i32> {
        self.inner.read_sint32()
    }

    #[napi]
    pub fn uint64(&mut self) -> Result<i64> {
        let value = self.inner.read_varint64()?;
        Ok(value as i64)
    }

    #[napi]
    pub fn int64(&mut self) -> Result<i64> {
        let value = self.inner.read_varint64()?;
        Ok(value as i64)
    }

    #[napi]
    pub fn sint64(&mut self) -> Result<i64> {
        self.inner.read_sint64()
    }

    #[napi]
    pub fn bool(&mut self) -> Result<bool> {
        let value = self.inner.read_varint32()?;
        Ok(value != 0)
    }

    #[napi]
    pub fn fixed32(&mut self) -> Result<u32> {
        self.inner.read_fixed32()
    }

    #[napi]
    pub fn sfixed32(&mut self) -> Result<i32> {
        self.inner.read_sfixed32()
    }

    #[napi]
    pub fn fixed64(&mut self) -> Result<i64> {
        let value = self.inner.read_fixed64()?;
        Ok(value as i64)
    }

    #[napi]
    pub fn sfixed64(&mut self) -> Result<i64> {
        self.inner.read_sfixed64()
    }

    #[napi]
    pub fn float(&mut self) -> Result<f64> {
        let value = self.inner.read_float()?;
        Ok(value as f64)
    }

    #[napi]
    pub fn double(&mut self) -> Result<f64> {
        self.inner.read_double()
    }

    #[napi]
    pub fn bytes(&mut self) -> Result<Buffer> {
        let len = self.inner.read_varint32()? as usize;
        let data = self.inner.read_bytes(len)?;
        Ok(Buffer::from(data))
    }

    #[napi]
    pub fn string(&mut self) -> Result<String> {
        let len = self.inner.read_varint32()? as usize;
        let bytes = self.inner.read_bytes(len)?;
        String::from_utf8(bytes).map_err(|e| {
            napi::Error::new(
                napi::Status::GenericFailure,
                format!("invalid utf8: {}", e),
            )
        })
    }

    #[napi]
    pub fn skip(&mut self, length: u32) -> Result<()> {
        self.inner.skip(length as usize)
    }

    #[napi]
    pub fn skip_type(&mut self, wire_type: u32) -> Result<()> {
        self.inner.skip_type(wire_type)
    }

    #[napi(getter)]
    pub fn pos(&self) -> u32 {
        self.inner.pos() as u32
    }

    #[napi(getter)]
    pub fn len(&self) -> u32 {
        self.inner.len() as u32
    }
}
