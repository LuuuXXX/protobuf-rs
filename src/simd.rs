use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Batch encode multiple uint32 values using SIMD optimizations when available
#[napi]
pub fn encode_varint_batch_simd(values: Vec<u32>) -> Result<Buffer> {
    // For now, use scalar implementation with batching optimization
    // SIMD implementation would require platform-specific code
    encode_varint_batch_scalar(&values)
}

/// Batch decode varints from buffer using SIMD optimizations when available
#[napi]
pub fn decode_varint_batch_simd(buffer: Buffer) -> Result<Vec<u32>> {
    // For now, use scalar implementation with batching optimization
    decode_varint_batch_scalar(buffer.as_ref())
}

// Scalar implementation for batch encoding
fn encode_varint_batch_scalar(values: &[u32]) -> Result<Buffer> {
    let mut result = Vec::with_capacity(values.len() * 5); // Max 5 bytes per u32 varint

    for &value in values {
        encode_varint_u32(value, &mut result);
    }

    Ok(result.into())
}

// Scalar implementation for batch decoding
fn decode_varint_batch_scalar(buffer: &[u8]) -> Result<Vec<u32>> {
    let mut result = Vec::new();
    let mut pos = 0;

    while pos < buffer.len() {
        match decode_varint_u32(&buffer[pos..]) {
            Ok((value, bytes_read)) => {
                result.push(value);
                pos += bytes_read;
            }
            Err(e) => return Err(Error::from_reason(e)),
        }
    }

    Ok(result)
}

// Helper function to encode a single u32 varint
fn encode_varint_u32(mut value: u32, output: &mut Vec<u8>) {
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;

        if value != 0 {
            byte |= 0x80;
        }

        output.push(byte);

        if value == 0 {
            break;
        }
    }
}

// Helper function to decode a single u32 varint
// Returns (value, bytes_read) or error
fn decode_varint_u32(buffer: &[u8]) -> std::result::Result<(u32, usize), &'static str> {
    let mut result: u32 = 0;
    let mut shift = 0;

    for (i, &byte) in buffer.iter().enumerate() {
        if i >= 5 {
            return Err("Varint too long for u32");
        }

        // On the 5th byte (index 4), only 4 bits should be set for u32
        if i == 4 && byte > 0x0F {
            return Err("Varint overflow for u32");
        }

        result |= ((byte & 0x7F) as u32) << shift;

        if byte & 0x80 == 0 {
            return Ok((result, i + 1));
        }

        shift += 7;
    }

    Err("Incomplete varint")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_varint_u32() {
        let mut output = Vec::new();
        encode_varint_u32(0, &mut output);
        assert_eq!(output, vec![0]);

        output.clear();
        encode_varint_u32(300, &mut output);
        assert_eq!(output, vec![0xAC, 0x02]);

        output.clear();
        encode_varint_u32(u32::MAX, &mut output);
        assert_eq!(output.len(), 5);
    }

    #[test]
    fn test_decode_varint_u32() {
        assert_eq!(decode_varint_u32(&[0]), Ok((0, 1)));
        assert_eq!(decode_varint_u32(&[0xAC, 0x02]), Ok((300, 2)));
        assert!(decode_varint_u32(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF]).is_err()); // overflow
        assert!(decode_varint_u32(&[0x80]).is_err()); // incomplete
    }

    #[test]
    fn test_batch_encoding() {
        let values = vec![0, 1, 127, 128, 300, 16384];
        let encoded = encode_varint_batch_scalar(&values).unwrap();
        let decoded = decode_varint_batch_scalar(encoded.as_ref()).unwrap();
        assert_eq!(values, decoded);
    }

    #[test]
    fn test_empty_batch() {
        let values: Vec<u32> = vec![];
        let encoded = encode_varint_batch_scalar(&values).unwrap();
        assert_eq!(encoded.len(), 0);

        let decoded = decode_varint_batch_scalar(&[]).unwrap();
        assert_eq!(decoded.len(), 0);
    }

    #[test]
    fn test_large_batch() {
        let values: Vec<u32> = (0..1000).collect();
        let encoded = encode_varint_batch_scalar(&values).unwrap();
        let decoded = decode_varint_batch_scalar(encoded.as_ref()).unwrap();
        assert_eq!(values, decoded);
    }
}
