#![allow(clippy::approx_constant)]
#![allow(clippy::useless_vec)]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use protobuf_rs::{Reader, Writer};
use std::hint::black_box;

fn create_test_message(size: &str) -> Vec<u8> {
    let mut writer = Writer::new();

    match size {
        "tiny" => {
            writer.write_uint32_field(1, 42);
        }
        "small" => {
            writer.write_uint32_field(1, 12345);
            writer.write_string_field(2, "test");
            writer.write_bool_field(3, true);
        }
        "medium" => {
            writer.write_uint32_field(1, 12345);
            writer.write_string_field(2, "user@example.com");
            writer.write_bool_field(3, true);
            writer.write_float_field(4, 3.14);
            writer.write_double_field(5, 2.71828);
            writer.write_bytes_field(6, &[1, 2, 3, 4, 5]);
        }
        "large" => {
            for i in 1..=20 {
                writer.write_uint32_field(i, i * 1000);
            }
            writer.write_string_field(21, &"x".repeat(100));
            writer.write_bytes_field(22, &vec![0u8; 200]);
        }
        "xlarge" => {
            for i in 1..=50 {
                writer.write_uint32_field(i, i * 1000);
            }
            writer.write_string_field(51, &"x".repeat(500));
            writer.write_bytes_field(52, &vec![0u8; 1000]);
        }
        _ => {}
    }

    writer.finish()
}

fn bench_message_encode_by_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("message_encode_by_size");

    for size in ["tiny", "small", "medium", "large", "xlarge"].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                black_box(create_test_message(black_box(size)));
            });
        });
    }

    group.finish();
}

fn bench_message_decode_by_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("message_decode_by_size");

    for size in ["tiny", "small", "medium", "large", "xlarge"].iter() {
        let data = create_test_message(size);

        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| {
                let mut reader = Reader::new(black_box(data));
                while !reader.is_eof() {
                    let (_, wire_type) = reader.read_tag().unwrap();
                    reader.skip(wire_type).unwrap();
                }
            });
        });
    }

    group.finish();
}

fn bench_message_roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("message_roundtrip");

    for size in ["tiny", "small", "medium", "large"].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let data = create_test_message(black_box(size));
                let mut reader = Reader::new(black_box(&data));
                while !reader.is_eof() {
                    let (_, wire_type) = reader.read_tag().unwrap();
                    reader.skip(wire_type).unwrap();
                }
            });
        });
    }

    group.finish();
}

fn bench_writer_reuse(c: &mut Criterion) {
    let mut group = c.benchmark_group("writer_reuse");

    group.bench_function("reuse", |b| {
        let mut writer = Writer::with_capacity(256);
        b.iter(|| {
            writer.reset();
            writer.write_uint32_field(1, 42);
            writer.write_string_field(2, "test");
            writer.write_bool_field(3, true);
            black_box(writer.as_slice());
        });
    });

    group.bench_function("create_new", |b| {
        b.iter(|| {
            let mut writer = Writer::new();
            writer.write_uint32_field(1, 42);
            writer.write_string_field(2, "test");
            writer.write_bool_field(3, true);
            black_box(writer.finish());
        });
    });

    group.finish();
}

fn bench_repeated_fields(c: &mut Criterion) {
    let mut group = c.benchmark_group("repeated_fields");

    for count in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("encode", count), count, |b, &count| {
            b.iter(|| {
                let mut writer = Writer::new();
                for i in 0..count {
                    writer.write_uint32_field(1, black_box(i as u32));
                }
                black_box(writer.finish());
            });
        });

        // Prepare data for decode benchmark
        let mut writer = Writer::new();
        for i in 0..*count {
            writer.write_uint32_field(1, i as u32);
        }
        let data = writer.finish();

        group.bench_with_input(BenchmarkId::new("decode", count), &data, |b, data| {
            b.iter(|| {
                let mut reader = Reader::new(black_box(data));
                while !reader.is_eof() {
                    let (_, _) = reader.read_tag().unwrap();
                    black_box(reader.read_varint32().unwrap());
                }
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_message_encode_by_size,
    bench_message_decode_by_size,
    bench_message_roundtrip,
    bench_writer_reuse,
    bench_repeated_fields
);
criterion_main!(benches);
