use napi::bindgen_prelude::*;
use napi_derive::napi;

// Phase 3: Advanced Performance Modules
mod parallel;
mod pool;
mod reader;
mod simd;
mod writer;

// Re-export Phase 3 functions
pub use parallel::{decode_varints_parallel, encode_varints_parallel, process_u32_batch_parallel};
pub use reader::Reader;
pub use simd::{decode_varint_batch_simd, encode_varint_batch_simd};
pub use writer::Writer;

const MAX_VARINT_BYTES: usize = 10;
const MAX_WIRE_TYPE: i64 = 5;
const MAX_FIELD_NUMBER: i64 = 536_870_911; // 2^29 - 1
const RESERVED_RANGE_START: i64 = 19_000;
const RESERVED_RANGE_END: i64 = 19_999;

#[napi]
pub struct ProtobufParser {
    data: Vec<u8>,
}

impl Default for ProtobufParser {
    fn default() -> Self {
        Self::new()
    }
}

#[napi]
impl ProtobufParser {
    #[napi(constructor)]
    pub fn new() -> Self {
        ProtobufParser { data: Vec::new() }
    }

    #[napi]
    pub fn parse(&mut self, buffer: Buffer) -> Result<String> {
        self.data = buffer.to_vec();
        Ok(format!("Parsed {} bytes", self.data.len()))
    }

    #[napi]
    pub fn get_size(&self) -> u32 {
        self.data.len() as u32
    }

    #[napi]
    pub fn get_data(&self) -> Buffer {
        self.data.clone().into()
    }
}

#[napi]
pub fn decode_varint(buffer: Buffer) -> Result<i64> {
    let bytes = buffer.as_ref();
    let mut result: u64 = 0;
    let mut shift = 0;

    for (i, &byte) in bytes.iter().enumerate() {
        if i >= MAX_VARINT_BYTES {
            return Err(Error::from_reason("Varint too long"));
        }

        // On the 10th byte (index 9), only 1 bit should be set to avoid overflow
        if i == MAX_VARINT_BYTES - 1 && byte > 1 {
            return Err(Error::from_reason("Varint overflow"));
        }

        result |= ((byte & 0x7F) as u64) << shift;

        if byte & 0x80 == 0 {
            return Ok(result as i64);
        }

        shift += 7;
    }

    Err(Error::from_reason("Incomplete varint"))
}

#[napi]
pub fn encode_varint(value: i64) -> Result<Buffer> {
    let mut result = Vec::new();
    let mut n = value as u64;

    loop {
        let mut byte = (n & 0x7F) as u8;
        n >>= 7;

        if n != 0 {
            byte |= 0x80;
        }

        result.push(byte);

        if n == 0 {
            break;
        }
    }

    Ok(result.into())
}

#[napi]
pub fn decode_zigzag(value: i64) -> i64 {
    let n = value as u64;
    ((n >> 1) as i64) ^ (-((n & 1) as i64))
}

#[napi]
pub fn encode_zigzag(value: i64) -> i64 {
    (value << 1) ^ (value >> 63)
}

#[napi]
pub fn decode_field_tag(buffer: Buffer) -> Result<Vec<i64>> {
    let bytes = buffer.as_ref();
    if bytes.is_empty() {
        return Err(Error::from_reason("Empty buffer"));
    }

    let tag = decode_varint(buffer)?;
    let field_number = (tag >> 3) as i64;
    let wire_type = (tag & 0x7) as i64;

    Ok(vec![field_number, wire_type])
}

#[napi]
pub fn encode_field_tag(field_number: i64, wire_type: i64) -> Result<Buffer> {
    if !(1..=MAX_FIELD_NUMBER).contains(&field_number)
        || (RESERVED_RANGE_START..=RESERVED_RANGE_END).contains(&field_number)
        || !(0..=MAX_WIRE_TYPE).contains(&wire_type)
    {
        return Err(Error::from_reason("Invalid field number or wire type"));
    }

    let tag = (field_number << 3) | wire_type;
    encode_varint(tag)
}
