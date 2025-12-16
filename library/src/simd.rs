// Copyright (c) 2024 LuuuXXX
// Licensed under the BSD-3-Clause License.
// See LICENSE file in the project root for full license information.

use napi::bindgen_prelude::*;
use napi_derive::napi;

/// 使用 SIMD 优化批量编码多个 uint32 值
/// Batch encode multiple uint32 values using SIMD optimizations when available
/// 
/// # SIMD 优化说明 / SIMD Optimization Notes
/// 当前使用优化的标量实现。硬件 SIMD（AVX2/NEON）支持将在未来版本中添加。
/// Currently uses optimized scalar implementation. Hardware SIMD (AVX2/NEON)
/// support will be added in a future release.
/// 
/// # 性能优势 / Performance Benefits
/// - 批量处理：减少函数调用开销 / Batch processing: Reduces function call overhead
/// - 预分配缓冲区：减少内存分配 / Pre-allocated buffer: Reduces memory allocations
/// - 内联优化：编译器内联批处理循环 / Inline optimization: Compiler inlines batch loops
/// - 未来 SIMD：4-8 倍加速（v1.1 计划）/ Future SIMD: 4-8x speedup (planned for v1.1)
/// 
/// # 参数 / Arguments
/// * `values` - 要编码的 u32 值数组 / Array of u32 values to encode
/// 
/// # 返回 / Returns
/// 包含所有编码 varint 的缓冲区 / Buffer containing all encoded varints
/// 
/// # 使用场景 / Use Cases
/// - 批量数据导出 / Bulk data export
/// - 重复字段编码 / Repeated field encoding
/// - 数据流处理 / Data stream processing
#[napi]
pub fn encode_varint_batch_simd(values: Vec<u32>) -> Result<Buffer> {
    // TODO: 在 v1.1 中添加硬件 SIMD 支持
    // TODO: Add hardware SIMD support in v1.1
    encode_varint_batch_scalar(&values)
}

/// 使用 SIMD 优化从缓冲区批量解码 varint
/// Batch decode varints from buffer using SIMD optimizations when available
///
/// # 性能优势 / Performance Benefits
/// - 批量解析：一次性解析多个 varint / Batch parsing: Parse multiple varints at once
/// - 减少边界检查：批处理减少重复检查 / Reduced bounds checking: Batch reduces redundant checks
/// - 缓存友好：顺序访问提升缓存命中率 / Cache-friendly: Sequential access improves cache hits
/// 
/// # 参数 / Arguments
/// * `buffer` - 包含多个 varint 的缓冲区 / Buffer containing multiple varints
/// 
/// # 返回 / Returns
/// 解码后的 u32 值数组 / Array of decoded u32 values
/// 
/// # 注意 / Note
/// 当前使用优化的标量实现。硬件 SIMD（AVX2/NEON）支持将在未来版本中添加。
/// Currently uses optimized scalar implementation. Hardware SIMD (AVX2/NEON)
/// support will be added in a future release.
#[napi]
pub fn decode_varint_batch_simd(buffer: Buffer) -> Result<Vec<u32>> {
    // TODO: 在 v1.1 中添加硬件 SIMD 支持
    // TODO: Add hardware SIMD support in v1.1
    decode_varint_batch_scalar(buffer.as_ref())
}

// 批量编码的标量实现
// Scalar implementation for batch encoding
fn encode_varint_batch_scalar(values: &[u32]) -> Result<Buffer> {
    // 预分配容量：每个 u32 varint 最多 5 字节
    // Pre-allocate capacity: Max 5 bytes per u32 varint
    let mut result = Vec::with_capacity(values.len() * 5);

    for &value in values {
        encode_varint_u32(value, &mut result);
    }

    Ok(result.into())
}

// 批量解码的标量实现
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

// 编码单个 u32 varint 的辅助函数
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

// 解码单个 u32 varint 的辅助函数
// Helper function to decode a single u32 varint
// 返回 (值, 读取字节数) 或错误
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
