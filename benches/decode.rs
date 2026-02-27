use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_decode(c: &mut Criterion) {
    c.bench_function("albania::decode valid", |b| {
        b.iter(|| nidx::albania::decode(black_box("J00101999W")))
    });

    c.bench_function("albania::decode invalid (short)", |b| {
        b.iter(|| nidx::albania::decode(black_box("short")))
    });

    c.bench_function("albania::decode invalid (bad checksum)", |b| {
        b.iter(|| nidx::albania::decode(black_box("J00101999A")))
    });

    c.bench_function("albania::is_valid", |b| {
        b.iter(|| nidx::albania::is_valid(black_box("J00101999W")))
    });

    c.bench_function("albania::decode lowercase", |b| {
        b.iter(|| nidx::albania::decode(black_box("j00101999w")))
    });
}

criterion_group!(benches, bench_decode);
criterion_main!(benches);
