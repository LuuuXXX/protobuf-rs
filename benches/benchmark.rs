use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use protobuf_rs::{Writer, Reader, WireType};

fn bench_varint_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("varint_encode");
    
    group.bench_function("small_values", |b| {
        b.iter(|| {
            let mut writer = Writer::new();
            for i in 0..100 {
                writer.write_varint32(black_box(i));
            }
            black_box(writer);
        });
    });
    
    group.bench_function("medium_values", |b| {
        b.iter(|| {
            let mut writer = Writer::new();
            for i in 0..100 {
                writer.write_varint32(black_box(i * 1000));
            }
            black_box(writer);
        });
    });
    
    group.bench_function("large_values", |b| {
        b.iter(|| {
            let mut writer = Writer::new();
            for i in 0..100 {
                writer.write_varint64(black_box(i * 1_000_000_000));
            }
            black_box(writer);
        });
    });
    
    group.finish();
}

fn bench_varint_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("varint_decode");
    
    // Prepare test data
    let mut writer = Writer::new();
    for i in 0..1000 {
        writer.write_varint32(i);
    }
    let data = writer.finish();
    
    group.bench_function("decode_1000_varints", |b| {
        b.iter(|| {
            let mut reader = Reader::new(&data);
            for _ in 0..1000 {
                black_box(reader.read_varint32().unwrap());
            }
        });
    });
    
    group.finish();
}

fn bench_string_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_encode");
    
    group.bench_function("short_strings", |b| {
        b.iter(|| {
            let mut writer = Writer::new();
            for _ in 0..100 {
                writer.write_string(black_box("Hello"));
            }
            black_box(writer);
        });
    });
    
    group.bench_function("medium_strings", |b| {
        b.iter(|| {
            let mut writer = Writer::new();
            for _ in 0..100 {
                writer.write_string(black_box("Hello, this is a medium length string for testing"));
            }
            black_box(writer);
        });
    });
    
    group.bench_function("long_strings", |b| {
        let long_str = "a".repeat(1000);
        b.iter(|| {
            let mut writer = Writer::new();
            for _ in 0..10 {
                writer.write_string(black_box(&long_str));
            }
            black_box(writer);
        });
    });
    
    group.finish();
}

fn bench_message_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("message_encode");
    
    group.bench_function("simple_message", |b| {
        b.iter(|| {
            let mut writer = Writer::new();
            writer.write_uint32_field(1, black_box(42));
            writer.write_string_field(2, black_box("test"));
            writer.write_bool_field(3, black_box(true));
            black_box(writer.finish());
        });
    });
    
    group.bench_function("complex_message", |b| {
        b.iter(|| {
            let mut writer = Writer::new();
            writer.write_uint32_field(1, black_box(12345));
            writer.write_string_field(2, black_box("user@example.com"));
            writer.write_bool_field(3, black_box(true));
            writer.write_float_field(4, black_box(3.14));
            writer.write_double_field(5, black_box(2.71828));
            writer.write_bytes_field(6, black_box(&[1, 2, 3, 4, 5]));
            writer.write_sint32_field(7, black_box(-100));
            black_box(writer.finish());
        });
    });
    
    group.finish();
}

fn bench_message_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("message_decode");
    
    // Prepare test message
    let mut writer = Writer::new();
    writer.write_uint32_field(1, 42);
    writer.write_string_field(2, "test");
    writer.write_bool_field(3, true);
    let data = writer.finish();
    
    group.bench_function("simple_message", |b| {
        b.iter(|| {
            let mut reader = Reader::new(black_box(&data));
            let (_, _) = reader.read_tag().unwrap();
            let _ = reader.read_varint32().unwrap();
            let (_, _) = reader.read_tag().unwrap();
            let _ = reader.read_string().unwrap();
            let (_, _) = reader.read_tag().unwrap();
            let _ = reader.read_bool().unwrap();
        });
    });
    
    group.finish();
}

fn bench_zigzag(c: &mut Criterion) {
    use protobuf_rs::zigzag::{encode_zigzag32, decode_zigzag32, encode_zigzag64, decode_zigzag64};
    
    let mut group = c.benchmark_group("zigzag");
    
    group.bench_function("encode_zigzag32", |b| {
        b.iter(|| {
            for i in -1000..1000 {
                black_box(encode_zigzag32(black_box(i)));
            }
        });
    });
    
    group.bench_function("decode_zigzag32", |b| {
        b.iter(|| {
            for i in 0..2000 {
                black_box(decode_zigzag32(black_box(i)));
            }
        });
    });
    
    group.bench_function("encode_zigzag64", |b| {
        b.iter(|| {
            for i in -1000..1000 {
                black_box(encode_zigzag64(black_box(i)));
            }
        });
    });
    
    group.bench_function("decode_zigzag64", |b| {
        b.iter(|| {
            for i in 0..2000 {
                black_box(decode_zigzag64(black_box(i)));
            }
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_varint_encode,
    bench_varint_decode,
    bench_string_encode,
    bench_message_encode,
    bench_message_decode,
    bench_zigzag
);
criterion_main!(benches);
