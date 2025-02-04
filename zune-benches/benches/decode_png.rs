use std::fs::read;
use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use zune_benches::sample_path;

fn decode_ref(data: &[u8]) -> Vec<u8>
{
    let decoder = png::Decoder::new(data);
    let mut reader = decoder.read_info().unwrap();

    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let _ = reader.next_frame(&mut buf).unwrap();

    buf
}

fn decode_zune(data: &[u8]) -> Vec<u8>
{
    zune_png::PngDecoder::new(data).decode_raw().unwrap()
}

fn decode_spng(data: &[u8]) -> Vec<u8>
{
    let cursor = std::io::Cursor::new(data);
    let decoder = spng::Decoder::new(cursor);
    let (_, mut reader) = decoder.read_info().unwrap();
    let output_buffer_size = reader.output_buffer_size();
    let mut out = vec![0; output_buffer_size];
    reader.next_frame(&mut out).unwrap();
    out
}

fn decode_lodepng(data: &[u8]) -> lodepng::Image
{
    lodepng::Decoder::new().decode(data).unwrap()
}

fn decode_test(c: &mut Criterion)
{
    let path = sample_path().join("test-images/png/benchmarks/speed_bench.png");
    let data = read(path).unwrap();

    let mut group = c.benchmark_group("png: PNG decoding baseline");
    group.throughput(Throughput::Bytes(data.len() as u64));

    group.bench_function("zune-png", |b| {
        b.iter(|| black_box(decode_zune(data.as_slice())))
    });

    group.bench_function("image-rs/png", |b| {
        b.iter(|| black_box(decode_ref(data.as_slice())))
    });

    group.bench_function("spng", |b| {
        b.iter(|| black_box(decode_spng(data.as_slice())))
    });

    group.bench_function("lodepng", |b| {
        b.iter(|| black_box(decode_lodepng(data.as_slice())))
    });
}

fn decode_test_interlaced(c: &mut Criterion)
{
    let path = sample_path().join("test-images/png/benchmarks/speed_bench_interlaced.png");

    let data = read(path).unwrap();

    let mut group = c.benchmark_group("png: PNG decoding interlaced 8bpp");
    group.throughput(Throughput::Bytes(data.len() as u64));

    group.bench_function("zune-png", |b| {
        b.iter(|| black_box(decode_zune(data.as_slice())))
    });

    group.bench_function("image-rs/png", |b| {
        b.iter(|| black_box(decode_ref(data.as_slice())))
    });

    group.bench_function("spng", |b| {
        b.iter(|| black_box(decode_spng(data.as_slice())))
    });

    group.bench_function("lodepng", |b| {
        b.iter(|| black_box(decode_lodepng(data.as_slice())))
    });
}

fn decode_test_16_bit(c: &mut Criterion)
{
    let path = sample_path().join("test-images/png/benchmarks/speed_bench_16.png");
    let data = read(path).unwrap();

    let mut group = c.benchmark_group("png: PNG decoding  16 bpp");
    group.throughput(Throughput::Bytes(data.len() as u64));

    group.bench_function("zune-png", |b| {
        b.iter(|| black_box(decode_zune(data.as_slice())))
    });

    group.bench_function("image-rs/png", |b| {
        b.iter(|| black_box(decode_ref(data.as_slice())))
    });

    group.bench_function("spng", |b| {
        b.iter(|| black_box(decode_spng(data.as_slice())))
    });

    group.bench_function("lodepng", |b| {
        b.iter(|| black_box(decode_lodepng(data.as_slice())))
    });
}
criterion_group!(name=benches;
      config={
      let c = Criterion::default();
        c.measurement_time(Duration::from_secs(20))
      };
    targets=decode_test_16_bit,decode_test,decode_test_interlaced);

criterion_main!(benches);
