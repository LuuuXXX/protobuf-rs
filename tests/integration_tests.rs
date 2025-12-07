#![allow(clippy::approx_constant)]

use protobuf_rs::{Reader, WireType, Writer};

#[test]
fn test_simple_message_roundtrip() {
    let mut writer = Writer::new();
    writer.write_uint32_field(1, 42);
    writer.write_string_field(2, "Hello, Protobuf!");
    writer.write_bool_field(3, true);

    let bytes = writer.finish();

    let mut reader = Reader::new(&bytes);

    let (field1, wire1) = reader.read_tag().unwrap();
    assert_eq!(field1, 1);
    assert_eq!(wire1, WireType::Varint);
    assert_eq!(reader.read_varint32().unwrap(), 42);

    let (field2, wire2) = reader.read_tag().unwrap();
    assert_eq!(field2, 2);
    assert_eq!(wire2, WireType::LengthDelimited);
    assert_eq!(reader.read_string().unwrap(), "Hello, Protobuf!");

    let (field3, wire3) = reader.read_tag().unwrap();
    assert_eq!(field3, 3);
    assert_eq!(wire3, WireType::Varint);
    assert!(reader.read_bool().unwrap());

    assert!(reader.is_eof());
}

#[test]
fn test_numeric_types_roundtrip() {
    let mut writer = Writer::new();

    // Unsigned
    writer.write_uint32_field(1, 100);
    writer.write_uint64_field(2, 200);

    // Signed (not zigzag)
    writer.write_int32_field(3, -100);
    writer.write_int64_field(4, -200);

    // Signed (zigzag)
    writer.write_sint32_field(5, -100);
    writer.write_sint64_field(6, -200);

    // Fixed
    writer.write_fixed32_field(7, 300);
    writer.write_fixed64_field(8, 400);
    writer.write_sfixed32_field(9, -300);
    writer.write_sfixed64_field(10, -400);

    // Float/Double
    writer.write_float_field(11, 3.14);
    writer.write_double_field(12, 2.71828);

    let bytes = writer.finish();
    let mut reader = Reader::new(&bytes);

    // Read unsigned
    reader.read_tag().unwrap();
    assert_eq!(reader.read_varint32().unwrap(), 100);

    reader.read_tag().unwrap();
    assert_eq!(reader.read_varint64().unwrap(), 200);

    // Read signed (not zigzag)
    reader.read_tag().unwrap();
    assert_eq!(reader.read_int32().unwrap(), -100);

    reader.read_tag().unwrap();
    assert_eq!(reader.read_int64().unwrap(), -200);

    // Read signed (zigzag)
    reader.read_tag().unwrap();
    assert_eq!(reader.read_sint32().unwrap(), -100);

    reader.read_tag().unwrap();
    assert_eq!(reader.read_sint64().unwrap(), -200);

    // Read fixed
    reader.read_tag().unwrap();
    assert_eq!(reader.read_fixed32().unwrap(), 300);

    reader.read_tag().unwrap();
    assert_eq!(reader.read_fixed64().unwrap(), 400);

    reader.read_tag().unwrap();
    assert_eq!(reader.read_sfixed32().unwrap(), -300);

    reader.read_tag().unwrap();
    assert_eq!(reader.read_sfixed64().unwrap(), -400);

    // Read float/double
    reader.read_tag().unwrap();
    assert!((reader.read_float().unwrap() - 3.14).abs() < 0.001);

    reader.read_tag().unwrap();
    assert!((reader.read_double().unwrap() - 2.71828).abs() < 0.00001);

    assert!(reader.is_eof());
}

#[test]
fn test_skip_fields() {
    let mut writer = Writer::new();
    writer.write_uint32_field(1, 100);
    writer.write_string_field(2, "skip me");
    writer.write_uint32_field(3, 200);

    let bytes = writer.finish();
    let mut reader = Reader::new(&bytes);

    // Read field 1
    let (field1, _) = reader.read_tag().unwrap();
    assert_eq!(field1, 1);
    assert_eq!(reader.read_varint32().unwrap(), 100);

    // Skip field 2
    let (field2, wire2) = reader.read_tag().unwrap();
    assert_eq!(field2, 2);
    reader.skip(wire2).unwrap();

    // Read field 3
    let (field3, _) = reader.read_tag().unwrap();
    assert_eq!(field3, 3);
    assert_eq!(reader.read_varint32().unwrap(), 200);
}

#[test]
fn test_nested_message() {
    // Create inner message
    let mut inner_writer = Writer::new();
    inner_writer.write_uint32_field(1, 42);
    inner_writer.write_string_field(2, "inner");
    let inner_bytes = inner_writer.finish();

    // Create outer message with embedded inner message
    let mut outer_writer = Writer::new();
    outer_writer.write_uint32_field(1, 100);
    outer_writer.write_bytes_field(2, &inner_bytes);
    outer_writer.write_string_field(3, "outer");
    let outer_bytes = outer_writer.finish();

    // Read outer message
    let mut outer_reader = Reader::new(&outer_bytes);

    outer_reader.read_tag().unwrap();
    assert_eq!(outer_reader.read_varint32().unwrap(), 100);

    outer_reader.read_tag().unwrap();
    let inner_msg_bytes = outer_reader.read_bytes().unwrap();

    outer_reader.read_tag().unwrap();
    assert_eq!(outer_reader.read_string().unwrap(), "outer");

    // Read inner message
    let mut inner_reader = Reader::new(inner_msg_bytes);

    inner_reader.read_tag().unwrap();
    assert_eq!(inner_reader.read_varint32().unwrap(), 42);

    inner_reader.read_tag().unwrap();
    assert_eq!(inner_reader.read_string().unwrap(), "inner");
}

#[test]
fn test_writer_reuse() {
    let mut writer = Writer::with_capacity(256);

    // First message
    writer.write_uint32_field(1, 100);
    writer.write_string_field(2, "first");
    let first_msg = writer.as_slice().to_vec();

    // Reuse writer for second message
    writer.reset();
    writer.write_uint32_field(1, 200);
    writer.write_string_field(2, "second");
    let second_msg = writer.as_slice().to_vec();

    // Verify first message
    let mut reader = Reader::new(&first_msg);
    reader.read_tag().unwrap();
    assert_eq!(reader.read_varint32().unwrap(), 100);
    reader.read_tag().unwrap();
    assert_eq!(reader.read_string().unwrap(), "first");

    // Verify second message
    let mut reader = Reader::new(&second_msg);
    reader.read_tag().unwrap();
    assert_eq!(reader.read_varint32().unwrap(), 200);
    reader.read_tag().unwrap();
    assert_eq!(reader.read_string().unwrap(), "second");
}

#[test]
fn test_protobufjs_compatibility() {
    // This test verifies compatibility with protobuf.js wire format
    // Field 1: uint32 = 150
    // Field 2: string = "testing"

    let mut writer = Writer::new();
    writer.write_uint32_field(1, 150);
    writer.write_string_field(2, "testing");

    let bytes = writer.finish();

    // Verify the wire format matches expected protobuf.js output
    // Tag for field 1, varint: (1 << 3) | 0 = 8
    // Value 150 as varint: 150 = 0x96 = 0b10010110
    //   First byte: 10010110 (lower 7 bits) | 0 (no continuation) = 0x96 = 150
    //   Actually 150 requires 2 bytes: 0x96 0x01
    //   Because 150 = 128 + 22, so it's [22 | 0x80, 1] = [150, 1]
    // Tag for field 2, length-delimited: (2 << 3) | 2 = 18
    // Length 7, then "testing"
    let expected = vec![
        8, 150, 1, // field 1 tag and varint value 150
        18, 7, b't', b'e', b's', b't', b'i', b'n', b'g', // field 2 tag, length, and value
    ];

    assert_eq!(bytes, expected);
}

#[test]
fn test_varint_edge_cases() {
    let mut writer = Writer::new();

    // Edge cases for varint encoding
    writer.write_varint32(0);
    writer.write_varint32(127); // Max 1-byte varint
    writer.write_varint32(128); // Min 2-byte varint
    writer.write_varint32(16383); // Max 2-byte varint
    writer.write_varint32(16384); // Min 3-byte varint
    writer.write_varint32(u32::MAX); // Max value

    let bytes = writer.finish();
    let mut reader = Reader::new(&bytes);

    assert_eq!(reader.read_varint32().unwrap(), 0);
    assert_eq!(reader.read_varint32().unwrap(), 127);
    assert_eq!(reader.read_varint32().unwrap(), 128);
    assert_eq!(reader.read_varint32().unwrap(), 16383);
    assert_eq!(reader.read_varint32().unwrap(), 16384);
    assert_eq!(reader.read_varint32().unwrap(), u32::MAX);
}

#[test]
fn test_utf8_strings() {
    let test_strings = [
        "Hello",
        "你好",
        "こんにちは",
        "Привет",
        "🎉🎊",
        "Mixed: Hello 你好 🎉",
    ];

    for test_str in &test_strings {
        let mut writer = Writer::new();
        writer.write_string_field(1, test_str);

        let bytes = writer.finish();
        let mut reader = Reader::new(&bytes);

        reader.read_tag().unwrap();
        assert_eq!(reader.read_string().unwrap(), *test_str);
    }
}

#[test]
fn test_empty_fields() {
    let mut writer = Writer::new();
    writer.write_string_field(1, "");
    writer.write_bytes_field(2, &[]);

    let bytes = writer.finish();
    let mut reader = Reader::new(&bytes);

    reader.read_tag().unwrap();
    assert_eq!(reader.read_string().unwrap(), "");

    reader.read_tag().unwrap();
    assert_eq!(reader.read_bytes().unwrap(), &[]);
}

#[test]
fn test_repeated_fields() {
    let mut writer = Writer::new();

    // Write repeated field (same field number multiple times)
    for i in 1..=5 {
        writer.write_uint32_field(1, i * 10);
    }

    let bytes = writer.finish();
    let mut reader = Reader::new(&bytes);

    for i in 1..=5 {
        let (field, _) = reader.read_tag().unwrap();
        assert_eq!(field, 1);
        assert_eq!(reader.read_varint32().unwrap(), i * 10);
    }

    assert!(reader.is_eof());
}

#[test]
fn test_large_message() {
    let mut writer = Writer::new();

    // Create a large message with many fields
    for i in 1..=100 {
        writer.write_uint32_field(i, i * 1000);
    }

    let bytes = writer.finish();
    let mut reader = Reader::new(&bytes);

    for i in 1..=100 {
        let (field, _) = reader.read_tag().unwrap();
        assert_eq!(field, i);
        assert_eq!(reader.read_varint32().unwrap(), i * 1000);
    }

    assert!(reader.is_eof());
}
