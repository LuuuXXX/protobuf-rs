//! WebAssembly bindings for protobuf-rs.
//! protobuf-rs 的 WebAssembly 绑定。
//!
//! This module provides JavaScript-friendly wrappers around the core protobuf
//! functionality, designed to be compiled to WebAssembly.
//!
//! 本模块提供围绕核心 protobuf 功能的 JavaScript 友好包装器，
//! 设计用于编译为 WebAssembly。

use crate::{Reader, WireType, Writer};
use wasm_bindgen::prelude::*;

/// WebAssembly wrapper for Writer.
/// Writer 的 WebAssembly 包装器。
#[wasm_bindgen]
pub struct WasmWriter {
    inner: Writer,
}

#[wasm_bindgen]
impl WasmWriter {
    /// Create a new WasmWriter.
    /// 创建新的 WasmWriter。
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Writer::new(),
        }
    }

    /// Create a new WasmWriter with the specified capacity.
    /// 创建具有指定容量的新 WasmWriter。
    #[wasm_bindgen(js_name = withCapacity)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Writer::with_capacity(capacity),
        }
    }

    /// Get the current length.
    /// 获取当前长度。
    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        self.inner.len()
    }

    /// Reset the writer.
    /// 重置写入器。
    pub fn reset(&mut self) {
        self.inner.reset();
    }

    /// Finish writing and return bytes.
    /// 完成写入并返回字节。
    pub fn finish(self) -> Vec<u8> {
        self.inner.finish()
    }

    /// Get bytes as a copy.
    /// 获取字节副本。
    #[wasm_bindgen(js_name = asBytes)]
    pub fn as_bytes(&self) -> Vec<u8> {
        self.inner.as_slice().to_vec()
    }

    /// Write a tag.
    /// 写入标签。
    #[wasm_bindgen(js_name = writeTag)]
    pub fn write_tag(&mut self, field_number: u32, wire_type: u8) {
        let wt = WireType::from_u8(wire_type).expect("Invalid wire type");
        self.inner.write_tag(field_number, wt);
    }

    /// Write a varint32.
    /// 写入 varint32。
    #[wasm_bindgen(js_name = writeVarint32)]
    pub fn write_varint32(&mut self, value: u32) {
        self.inner.write_varint32(value);
    }

    /// Write a varint64.
    /// 写入 varint64。
    #[wasm_bindgen(js_name = writeVarint64)]
    pub fn write_varint64(&mut self, value: u64) {
        self.inner.write_varint64(value);
    }

    /// Write a sint32.
    /// 写入 sint32。
    #[wasm_bindgen(js_name = writeSint32)]
    pub fn write_sint32(&mut self, value: i32) {
        self.inner.write_sint32(value);
    }

    /// Write a sint64.
    /// 写入 sint64。
    #[wasm_bindgen(js_name = writeSint64)]
    pub fn write_sint64(&mut self, value: i64) {
        self.inner.write_sint64(value);
    }

    /// Write a fixed32.
    /// 写入 fixed32。
    #[wasm_bindgen(js_name = writeFixed32)]
    pub fn write_fixed32(&mut self, value: u32) {
        self.inner.write_fixed32(value);
    }

    /// Write a fixed64.
    /// 写入 fixed64。
    #[wasm_bindgen(js_name = writeFixed64)]
    pub fn write_fixed64(&mut self, value: u64) {
        self.inner.write_fixed64(value);
    }

    /// Write an sfixed32.
    /// 写入 sfixed32。
    #[wasm_bindgen(js_name = writeSfixed32)]
    pub fn write_sfixed32(&mut self, value: i32) {
        self.inner.write_sfixed32(value);
    }

    /// Write an sfixed64.
    /// 写入 sfixed64。
    #[wasm_bindgen(js_name = writeSfixed64)]
    pub fn write_sfixed64(&mut self, value: i64) {
        self.inner.write_sfixed64(value);
    }

    /// Write a float.
    /// 写入浮点数。
    #[wasm_bindgen(js_name = writeFloat)]
    pub fn write_float(&mut self, value: f32) {
        self.inner.write_float(value);
    }

    /// Write a double.
    /// 写入双精度浮点数。
    #[wasm_bindgen(js_name = writeDouble)]
    pub fn write_double(&mut self, value: f64) {
        self.inner.write_double(value);
    }

    /// Write a bool.
    /// 写入布尔值。
    #[wasm_bindgen(js_name = writeBool)]
    pub fn write_bool(&mut self, value: bool) {
        self.inner.write_bool(value);
    }

    /// Write a string.
    /// 写入字符串。
    #[wasm_bindgen(js_name = writeString)]
    pub fn write_string(&mut self, value: &str) {
        self.inner.write_string(value);
    }

    /// Write bytes.
    /// 写入字节。
    #[wasm_bindgen(js_name = writeBytes)]
    pub fn write_bytes(&mut self, value: &[u8]) {
        self.inner.write_bytes(value);
    }

    /// Write raw bytes.
    /// 写入原始字节。
    #[wasm_bindgen(js_name = writeRawBytes)]
    pub fn write_raw_bytes(&mut self, value: &[u8]) {
        self.inner.write_raw_bytes(value);
    }

    // Field writers
    // 字段写入器

    /// Write a uint32 field.
    /// 写入 uint32 字段。
    #[wasm_bindgen(js_name = writeUint32Field)]
    pub fn write_uint32_field(&mut self, field_number: u32, value: u32) {
        self.inner.write_uint32_field(field_number, value);
    }

    /// Write a uint64 field.
    /// 写入 uint64 字段。
    #[wasm_bindgen(js_name = writeUint64Field)]
    pub fn write_uint64_field(&mut self, field_number: u32, value: u64) {
        self.inner.write_uint64_field(field_number, value);
    }

    /// Write an int32 field.
    /// 写入 int32 字段。
    #[wasm_bindgen(js_name = writeInt32Field)]
    pub fn write_int32_field(&mut self, field_number: u32, value: i32) {
        self.inner.write_int32_field(field_number, value);
    }

    /// Write an int64 field.
    /// 写入 int64 字段。
    #[wasm_bindgen(js_name = writeInt64Field)]
    pub fn write_int64_field(&mut self, field_number: u32, value: i64) {
        self.inner.write_int64_field(field_number, value);
    }

    /// Write a sint32 field.
    /// 写入 sint32 字段。
    #[wasm_bindgen(js_name = writeSint32Field)]
    pub fn write_sint32_field(&mut self, field_number: u32, value: i32) {
        self.inner.write_sint32_field(field_number, value);
    }

    /// Write a sint64 field.
    /// 写入 sint64 字段。
    #[wasm_bindgen(js_name = writeSint64Field)]
    pub fn write_sint64_field(&mut self, field_number: u32, value: i64) {
        self.inner.write_sint64_field(field_number, value);
    }

    /// Write a string field.
    /// 写入字符串字段。
    #[wasm_bindgen(js_name = writeStringField)]
    pub fn write_string_field(&mut self, field_number: u32, value: &str) {
        self.inner.write_string_field(field_number, value);
    }

    /// Write a bytes field.
    /// 写入字节字段。
    #[wasm_bindgen(js_name = writeBytesField)]
    pub fn write_bytes_field(&mut self, field_number: u32, value: &[u8]) {
        self.inner.write_bytes_field(field_number, value);
    }

    /// Write a bool field.
    /// 写入布尔字段。
    #[wasm_bindgen(js_name = writeBoolField)]
    pub fn write_bool_field(&mut self, field_number: u32, value: bool) {
        self.inner.write_bool_field(field_number, value);
    }

    /// Write a float field.
    /// 写入浮点字段。
    #[wasm_bindgen(js_name = writeFloatField)]
    pub fn write_float_field(&mut self, field_number: u32, value: f32) {
        self.inner.write_float_field(field_number, value);
    }

    /// Write a double field.
    /// 写入双精度浮点字段。
    #[wasm_bindgen(js_name = writeDoubleField)]
    pub fn write_double_field(&mut self, field_number: u32, value: f64) {
        self.inner.write_double_field(field_number, value);
    }

    /// Write a fixed32 field.
    /// 写入 fixed32 字段。
    #[wasm_bindgen(js_name = writeFixed32Field)]
    pub fn write_fixed32_field(&mut self, field_number: u32, value: u32) {
        self.inner.write_fixed32_field(field_number, value);
    }

    /// Write a fixed64 field.
    /// 写入 fixed64 字段。
    #[wasm_bindgen(js_name = writeFixed64Field)]
    pub fn write_fixed64_field(&mut self, field_number: u32, value: u64) {
        self.inner.write_fixed64_field(field_number, value);
    }

    /// Write an sfixed32 field.
    /// 写入 sfixed32 字段。
    #[wasm_bindgen(js_name = writeSfixed32Field)]
    pub fn write_sfixed32_field(&mut self, field_number: u32, value: i32) {
        self.inner.write_sfixed32_field(field_number, value);
    }

    /// Write an sfixed64 field.
    /// 写入 sfixed64 字段。
    #[wasm_bindgen(js_name = writeSfixed64Field)]
    pub fn write_sfixed64_field(&mut self, field_number: u32, value: i64) {
        self.inner.write_sfixed64_field(field_number, value);
    }
}

/// WebAssembly wrapper for Reader.
/// Reader 的 WebAssembly 包装器。
#[wasm_bindgen]
pub struct WasmReader {
    inner: Reader<'static>,
    _data: Vec<u8>,
}

#[wasm_bindgen]
impl WasmReader {
    /// Create a new WasmReader.
    /// 创建新的 WasmReader。
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<u8>) -> Self {
        // We need to store the data to keep it alive
        // We use unsafe to create a static lifetime reference
        // This is safe because we control the lifetime through the struct
        let reader = unsafe {
            let ptr = data.as_ptr();
            let len = data.len();
            Reader::new(core::slice::from_raw_parts(ptr, len))
        };

        Self {
            inner: reader,
            _data: data,
        }
    }

    /// Get current position.
    /// 获取当前位置。
    #[wasm_bindgen(getter)]
    pub fn pos(&self) -> usize {
        self.inner.pos()
    }

    /// Get total length.
    /// 获取总长度。
    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        self.inner.len()
    }

    /// Get remaining bytes.
    /// 获取剩余字节数。
    pub fn remaining(&self) -> usize {
        self.inner.remaining()
    }

    /// Check if EOF.
    /// 检查是否到达文件末尾。
    #[wasm_bindgen(js_name = isEof)]
    pub fn is_eof(&self) -> bool {
        self.inner.is_eof()
    }

    /// Reset to beginning.
    /// 重置到开头。
    pub fn reset(&mut self) {
        self.inner.reset();
    }

    /// Read a tag.
    /// 读取标签。
    /// Returns [field_number, wire_type]
    #[wasm_bindgen(js_name = readTag)]
    pub fn read_tag(&mut self) -> Result<Vec<u32>, JsValue> {
        let (field_number, wire_type) = self
            .inner
            .read_tag()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(vec![field_number, wire_type.as_u8() as u32])
    }

    /// Read a varint32.
    /// 读取 varint32。
    #[wasm_bindgen(js_name = readVarint32)]
    pub fn read_varint32(&mut self) -> Result<u32, JsValue> {
        self.inner
            .read_varint32()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Read a varint64.
    /// 读取 varint64。
    #[wasm_bindgen(js_name = readVarint64)]
    pub fn read_varint64(&mut self) -> Result<u64, JsValue> {
        self.inner
            .read_varint64()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Read a sint32.
    /// 读取 sint32。
    #[wasm_bindgen(js_name = readSint32)]
    pub fn read_sint32(&mut self) -> Result<i32, JsValue> {
        self.inner
            .read_sint32()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Read a sint64.
    /// 读取 sint64。
    #[wasm_bindgen(js_name = readSint64)]
    pub fn read_sint64(&mut self) -> Result<i64, JsValue> {
        self.inner
            .read_sint64()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Read an int32.
    /// 读取 int32。
    #[wasm_bindgen(js_name = readInt32)]
    pub fn read_int32(&mut self) -> Result<i32, JsValue> {
        self.inner
            .read_int32()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Read an int64.
    /// 读取 int64。
    #[wasm_bindgen(js_name = readInt64)]
    pub fn read_int64(&mut self) -> Result<i64, JsValue> {
        self.inner
            .read_int64()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Read a fixed32.
    /// 读取 fixed32。
    #[wasm_bindgen(js_name = readFixed32)]
    pub fn read_fixed32(&mut self) -> Result<u32, JsValue> {
        self.inner
            .read_fixed32()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Read a fixed64.
    /// 读取 fixed64。
    #[wasm_bindgen(js_name = readFixed64)]
    pub fn read_fixed64(&mut self) -> Result<u64, JsValue> {
        self.inner
            .read_fixed64()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Read an sfixed32.
    /// 读取 sfixed32。
    #[wasm_bindgen(js_name = readSfixed32)]
    pub fn read_sfixed32(&mut self) -> Result<i32, JsValue> {
        self.inner
            .read_sfixed32()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Read an sfixed64.
    /// 读取 sfixed64。
    #[wasm_bindgen(js_name = readSfixed64)]
    pub fn read_sfixed64(&mut self) -> Result<i64, JsValue> {
        self.inner
            .read_sfixed64()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Read a float.
    /// 读取浮点数。
    #[wasm_bindgen(js_name = readFloat)]
    pub fn read_float(&mut self) -> Result<f32, JsValue> {
        self.inner
            .read_float()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Read a double.
    /// 读取双精度浮点数。
    #[wasm_bindgen(js_name = readDouble)]
    pub fn read_double(&mut self) -> Result<f64, JsValue> {
        self.inner
            .read_double()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Read a bool.
    /// 读取布尔值。
    #[wasm_bindgen(js_name = readBool)]
    pub fn read_bool(&mut self) -> Result<bool, JsValue> {
        self.inner
            .read_bool()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Read a string.
    /// 读取字符串。
    #[wasm_bindgen(js_name = readString)]
    pub fn read_string(&mut self) -> Result<String, JsValue> {
        self.inner
            .read_string()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Read bytes.
    /// 读取字节。
    #[wasm_bindgen(js_name = readBytes)]
    pub fn read_bytes(&mut self) -> Result<Vec<u8>, JsValue> {
        self.inner
            .read_bytes()
            .map(|b| b.to_vec())
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Skip a field.
    /// 跳过字段。
    pub fn skip(&mut self, wire_type: u8) -> Result<(), JsValue> {
        let wt =
            WireType::from_u8(wire_type).ok_or_else(|| JsValue::from_str("Invalid wire type"))?;
        self.inner
            .skip(wt)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Skip n bytes.
    /// 跳过 n 个字节。
    #[wasm_bindgen(js_name = skipBytes)]
    pub fn skip_bytes(&mut self, n: usize) -> Result<(), JsValue> {
        self.inner
            .skip_bytes(n)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

/// Create a tag from field number and wire type.
/// 从字段号和线类型创建标签。
#[wasm_bindgen(js_name = makeTag)]
pub fn make_tag_util(field_number: u32, wire_type: u8) -> u32 {
    let wt = WireType::from_u8(wire_type).expect("Invalid wire type");
    crate::wire::make_tag(field_number, wt)
}

/// Parse a tag into [field_number, wire_type].
/// 将标签解析为 [字段号, 线类型]。
#[wasm_bindgen(js_name = parseTag)]
pub fn parse_tag_util(tag: u32) -> Result<Vec<u32>, JsValue> {
    let (field_number, wire_type) =
        crate::wire::parse_tag(tag).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(vec![field_number, wire_type.as_u8() as u32])
}

/// Encode a signed 32-bit integer using ZigZag.
/// 使用 ZigZag 编码有符号 32 位整数。
#[wasm_bindgen(js_name = encodeZigZag32)]
pub fn encode_zigzag32_util(n: i32) -> u32 {
    crate::zigzag::encode_zigzag32(n)
}

/// Decode a ZigZag-encoded 32-bit integer.
/// 解码 ZigZag 编码的 32 位整数。
#[wasm_bindgen(js_name = decodeZigZag32)]
pub fn decode_zigzag32_util(n: u32) -> i32 {
    crate::zigzag::decode_zigzag32(n)
}

/// Encode a signed 64-bit integer using ZigZag.
/// 使用 ZigZag 编码有符号 64 位整数。
#[wasm_bindgen(js_name = encodeZigZag64)]
pub fn encode_zigzag64_util(n: i64) -> u64 {
    crate::zigzag::encode_zigzag64(n)
}

/// Decode a ZigZag-encoded 64-bit integer.
/// 解码 ZigZag 编码的 64 位整数。
#[wasm_bindgen(js_name = decodeZigZag64)]
pub fn decode_zigzag64_util(n: u64) -> i64 {
    crate::zigzag::decode_zigzag64(n)
}

/// Wire type constants for JavaScript.
/// JavaScript 的线类型常量。
#[wasm_bindgen]
pub struct WireTypes;

#[wasm_bindgen]
impl WireTypes {
    #[wasm_bindgen(getter, js_name = VARINT)]
    pub fn varint() -> u8 {
        WireType::Varint.as_u8()
    }

    #[wasm_bindgen(getter, js_name = FIXED64)]
    pub fn fixed64() -> u8 {
        WireType::Fixed64.as_u8()
    }

    #[wasm_bindgen(getter, js_name = LENGTH_DELIMITED)]
    pub fn length_delimited() -> u8 {
        WireType::LengthDelimited.as_u8()
    }

    #[wasm_bindgen(getter, js_name = START_GROUP)]
    pub fn start_group() -> u8 {
        WireType::StartGroup.as_u8()
    }

    #[wasm_bindgen(getter, js_name = END_GROUP)]
    pub fn end_group() -> u8 {
        WireType::EndGroup.as_u8()
    }

    #[wasm_bindgen(getter, js_name = FIXED32)]
    pub fn fixed32() -> u8 {
        WireType::Fixed32.as_u8()
    }
}
