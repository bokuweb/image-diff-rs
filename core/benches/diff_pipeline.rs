use criterion::{black_box, criterion_group, criterion_main, Criterion};
use image_diff_rs::{diff, diff_rgba, encode_diff, DiffOption, EncodeFormat};

const ACTUAL: &[u8] = include_bytes!("../../fixtures/sample0.webp");
const EXPECTED: &[u8] = include_bytes!("../../fixtures/sample1.webp");

fn bench_diff_pipeline(c: &mut Criterion) {
    let option = DiffOption {
        threshold: Some(0.01),
        include_anti_alias: Some(true),
        ..Default::default()
    };

    let mut group = c.benchmark_group("800x578_webp_input");
    group.bench_function("diff_with_webp_encoding", |b| {
        b.iter(|| diff(black_box(ACTUAL), black_box(EXPECTED), black_box(&option)))
    });
    group.bench_function("diff_rgba_without_encoding", |b| {
        b.iter(|| diff_rgba(black_box(ACTUAL), black_box(EXPECTED), black_box(&option)))
    });

    let rgba = diff_rgba(ACTUAL, EXPECTED, &option).unwrap();
    group.bench_function("webp_encoding_only", |b| {
        b.iter(|| encode_diff(black_box(&rgba), EncodeFormat::Webp))
    });
    group.bench_function("png_encoding_only", |b| {
        b.iter(|| encode_diff(black_box(&rgba), EncodeFormat::Png))
    });
    group.finish();
}

criterion_group!(benches, bench_diff_pipeline);
criterion_main!(benches);
