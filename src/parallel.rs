use napi::bindgen_prelude::*;
use napi_derive::napi;
use rayon::prelude::*;

/// Encode multiple buffers in parallel
#[napi]
pub fn encode_varints_parallel(values: Vec<i64>) -> Result<Vec<Buffer>> {
    let results: Vec<_> = values
        .par_iter()
        .map(|&value| {
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

            result
        })
        .collect();

    Ok(results.into_iter().map(|v| v.into()).collect())
}

/// Decode multiple buffers in parallel
#[napi]
pub fn decode_varints_parallel(buffers: Vec<Buffer>) -> Result<Vec<i64>> {
    // Convert to owned data before parallelization
    let data: Vec<Vec<u8>> = buffers.iter().map(|b| b.to_vec()).collect();

    data.par_iter()
        .map(|bytes| {
            let mut result: u64 = 0;
            let mut shift = 0;

            for (i, &byte) in bytes.iter().enumerate() {
                if i >= 10 {
                    return Err(Error::from_reason("Varint too long"));
                }

                if i == 9 && byte > 1 {
                    return Err(Error::from_reason("Varint overflow"));
                }

                result |= ((byte & 0x7F) as u64) << shift;

                if byte & 0x80 == 0 {
                    return Ok(result as i64);
                }

                shift += 7;
            }

            Err(Error::from_reason("Incomplete varint"))
        })
        .collect()
}

/// Process batch of u32 varints in parallel using chunking
#[napi]
pub fn process_u32_batch_parallel(values: Vec<u32>, chunk_size: Option<u32>) -> Result<Buffer> {
    let chunk_size = chunk_size.unwrap_or(100) as usize;

    if values.is_empty() {
        return Ok(Vec::new().into());
    }

    // Process in parallel chunks
    let results: Vec<Vec<u8>> = values
        .par_chunks(chunk_size)
        .map(|chunk| {
            let mut buffer = Vec::with_capacity(chunk.len() * 5);
            for &value in chunk {
                encode_varint_u32(value, &mut buffer);
            }
            buffer
        })
        .collect();

    // Concatenate results
    let total_size: usize = results.iter().map(|v| v.len()).sum();
    let mut final_buffer = Vec::with_capacity(total_size);
    for buffer in results {
        final_buffer.extend_from_slice(&buffer);
    }

    Ok(final_buffer.into())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_encoding() {
        let values = vec![0, 1, 127, 300, 16384];
        let encoded = encode_varints_parallel(values.clone()).unwrap();

        assert_eq!(encoded.len(), 5);
        assert_eq!(encoded[0].as_ref(), &[0]);
        assert_eq!(encoded[3].as_ref(), &[0xAC, 0x02]); // 300
    }

    #[test]
    fn test_parallel_decoding() {
        let buffers: Vec<Buffer> = vec![
            vec![0].into(),
            vec![1].into(),
            vec![0xAC, 0x02].into(), // 300
        ];

        let decoded = decode_varints_parallel(buffers).unwrap();
        assert_eq!(decoded, vec![0, 1, 300]);
    }

    #[test]
    fn test_round_trip_parallel() {
        let values = vec![0, 1, 127, 128, 300, 16384, 2147483647];
        let encoded = encode_varints_parallel(values.clone()).unwrap();
        let decoded = decode_varints_parallel(encoded).unwrap();
        assert_eq!(values, decoded);
    }

    #[test]
    fn test_process_u32_batch_parallel() {
        let values: Vec<u32> = (0..1000).collect();
        let encoded = process_u32_batch_parallel(values.clone(), Some(100)).unwrap();

        // Verify we can decode it
        let mut pos = 0;
        let mut decoded = Vec::new();
        while pos < encoded.len() {
            let mut result: u32 = 0;
            let mut shift = 0;

            loop {
                let byte = encoded[pos];
                pos += 1;

                result |= ((byte & 0x7F) as u32) << shift;

                if byte & 0x80 == 0 {
                    break;
                }
                shift += 7;
            }
            decoded.push(result);
        }

        assert_eq!(values, decoded);
    }

    #[test]
    fn test_empty_batch() {
        let values: Vec<u32> = vec![];
        let encoded = process_u32_batch_parallel(values, None).unwrap();
        assert_eq!(encoded.len(), 0);
    }
}
