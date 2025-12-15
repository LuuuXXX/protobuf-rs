// Copyright (c) 2024 LuuuXXX
// Licensed under the BSD-3-Clause License.
// See LICENSE file in the project root for full license information.

use napi::bindgen_prelude::*;
use napi_derive::napi;
use rayon::prelude::*;

/// 并行编码多个 varint
/// Encode multiple varints in parallel
/// 
/// # 并行化说明 / Parallelization Notes
/// 使用 Rayon 库实现工作窃取式并行，自动利用所有 CPU 核心。
/// Uses Rayon library for work-stealing parallelism, automatically utilizing all CPU cores.
/// 
/// # 性能优势 / Performance Benefits
/// - 多核并行：接近线性扩展（8核 ~7.5x）/ Multi-core parallelism: Near-linear scaling (8 cores ~7.5x)
/// - 无锁设计：每个线程独立工作 / Lock-free design: Each thread works independently
/// - 工作窃取：动态负载均衡 / Work stealing: Dynamic load balancing
/// 
/// # 适用场景 / Suitable Scenarios
/// - 大数据集（>1000 个值）/ Large datasets (>1000 values)
/// - CPU 密集型任务 / CPU-intensive tasks
/// - 多核服务器环境 / Multi-core server environments
/// 
/// # 参数 / Arguments
/// * `values` - 要编码的 i64 值数组 / Array of i64 values to encode
/// 
/// # 返回 / Returns
/// 编码后的缓冲区数组 / Array of encoded buffers
#[napi]
pub fn encode_varints_parallel(values: Vec<i64>) -> Result<Vec<Buffer>> {
    // 使用 Rayon 的并行迭代器
    // Use Rayon's parallel iterator
    let results: Vec<_> = values
        .par_iter()  // 并行迭代 / Parallel iteration
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

/// 并行解码多个 varint
/// Decode multiple varints in parallel
/// 
/// # 并行化策略 / Parallelization Strategy
/// 先将所有缓冲区转换为 owned 数据，然后并行解码。
/// First converts all buffers to owned data, then decodes in parallel.
/// 这避免了跨线程共享引用的问题。
/// This avoids issues with sharing references across threads.
/// 
/// # 参数 / Arguments
/// * `buffers` - 包含 varint 的缓冲区数组 / Array of buffers containing varints
/// 
/// # 返回 / Returns
/// 解码后的 i64 值数组 / Array of decoded i64 values
#[napi]
pub fn decode_varints_parallel(buffers: Vec<Buffer>) -> Result<Vec<i64>> {
    // 转换为 owned 数据以便并行化
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

/// 使用分块并行处理批量 u32 varint
/// Process batch of u32 varints in parallel using chunking
/// 
/// # 分块策略 / Chunking Strategy
/// 将大数据集分成小块，每块并行处理，最后合并结果。
/// Splits large datasets into chunks, processes each in parallel, then merges results.
/// 
/// # 性能调优 / Performance Tuning
/// - 小数据集（<1000）：使用默认块大小 100 / Small datasets (<1000): Use default chunk size 100
/// - 中等数据集（1K-100K）：块大小 1000 / Medium datasets (1K-100K): Chunk size 1000
/// - 大数据集（>100K）：块大小 10000 / Large datasets (>100K): Chunk size 10000
/// 
/// # 参数 / Arguments
/// * `values` - 要处理的 u32 值数组 / Array of u32 values to process
/// * `chunk_size` - 可选的块大小（默认 100）/ Optional chunk size (default 100)
/// 
/// # 返回 / Returns
/// 包含所有编码 varint 的缓冲区 / Buffer containing all encoded varints
/// 
/// # 示例 / Example
/// ```javascript
/// // 处理 100K 个值，使用 1000 的块大小
/// const result = processU32BatchParallel(values, 1000);
/// ```
#[napi]
pub fn process_u32_batch_parallel(values: Vec<u32>, chunk_size: Option<u32>) -> Result<Buffer> {
    let chunk_size = chunk_size.unwrap_or(100) as usize;

    if values.is_empty() {
        return Ok(Vec::new().into());
    }

    // 并行处理分块
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
