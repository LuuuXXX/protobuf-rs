use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use protobuf_rs::varint::{decode_varint32, decode_varint64, encode_varint32, encode_varint64};

fn bench_varint32_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("varint32_encode");

    let test_values = [
        ("1_byte", 127u32),
        ("2_bytes", 16383),
        ("3_bytes", 2097151),
        ("4_bytes", 268435455),
        ("5_bytes", u32::MAX),
    ];

    for (name, value) in test_values.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(name), value, |b, &value| {
            b.iter(|| {
                let mut buf = [0u8; 5];
                black_box(encode_varint32(&mut buf, black_box(value)));
            });
        });
    }

    group.finish();
}

fn bench_varint64_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("varint64_encode");

    let test_values = [
        ("1_byte", 127u64),
        ("2_bytes", 16383),
        ("5_bytes", u32::MAX as u64),
        ("10_bytes", u64::MAX),
    ];

    for (name, value) in test_values.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(name), value, |b, &value| {
            b.iter(|| {
                let mut buf = [0u8; 10];
                black_box(encode_varint64(&mut buf, black_box(value)));
            });
        });
    }

    group.finish();
}

fn bench_varint32_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("varint32_decode");

    let test_cases = [
        ("1_byte", 127u32),
        ("2_bytes", 16383),
        ("3_bytes", 2097151),
        ("4_bytes", 268435455),
        ("5_bytes", u32::MAX),
    ];

    for (name, value) in test_cases.iter() {
        let mut buf = [0u8; 5];
        encode_varint32(&mut buf, *value);

        group.bench_with_input(BenchmarkId::from_parameter(name), &buf, |b, buf| {
            b.iter(|| {
                black_box(decode_varint32(black_box(buf)).unwrap());
            });
        });
    }

    group.finish();
}

fn bench_varint64_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("varint64_decode");

    let test_cases = [
        ("1_byte", 127u64),
        ("2_bytes", 16383),
        ("5_bytes", u32::MAX as u64),
        ("10_bytes", u64::MAX),
    ];

    for (name, value) in test_cases.iter() {
        let mut buf = [0u8; 10];
        encode_varint64(&mut buf, *value);

        group.bench_with_input(BenchmarkId::from_parameter(name), &buf, |b, buf| {
            b.iter(|| {
                black_box(decode_varint64(black_box(buf)).unwrap());
            });
        });
    }

    group.finish();
}

fn bench_bulk_varint_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("bulk_varint_encode");

    for count in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(count), count, |b, &count| {
            let values: Vec<u32> = (0..count).collect();
            b.iter(|| {
                let mut buf = vec![0u8; (count as usize) * 5];
                let mut offset = 0;
                for &value in &values {
                    let len = encode_varint32(&mut buf[offset..], black_box(value));
                    offset += len;
                }
                black_box(buf);
            });
        });
    }

    group.finish();
}

fn bench_bulk_varint_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("bulk_varint_decode");

    for count in [10, 100, 1000].iter() {
        let mut buf = vec![0u8; (count * 5) as usize];
        let mut offset = 0;
        for i in 0..*count {
            let len = encode_varint32(&mut buf[offset..], i as u32);
            offset += len;
        }
        buf.truncate(offset);

        group.bench_with_input(BenchmarkId::from_parameter(count), &buf, |b, buf| {
            b.iter(|| {
                let mut offset = 0;
                for _ in 0..*count {
                    let (value, len) = decode_varint32(&buf[offset..]).unwrap();
                    black_box(value);
                    offset += len;
                }
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_varint32_encode,
    bench_varint64_encode,
    bench_varint32_decode,
    bench_varint64_decode,
    bench_bulk_varint_encode,
    bench_bulk_varint_decode
);
criterion_main!(benches);
