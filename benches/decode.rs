use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_albania(c: &mut Criterion) {
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

fn bench_kosovo(c: &mut Criterion) {
    c.bench_function("kosovo::validate valid", |b| {
        b.iter(|| nidx::kosovo::validate(black_box("1234567892")))
    });

    c.bench_function("kosovo::validate prefix-9 bypass", |b| {
        b.iter(|| nidx::kosovo::validate(black_box("9000000001")))
    });

    c.bench_function("kosovo::validate invalid (short)", |b| {
        b.iter(|| nidx::kosovo::validate(black_box("short")))
    });

    c.bench_function("kosovo::validate invalid (bad checksum)", |b| {
        b.iter(|| nidx::kosovo::validate(black_box("1234567890")))
    });

    c.bench_function("kosovo::is_valid", |b| {
        b.iter(|| nidx::kosovo::is_valid(black_box("1234567892")))
    });
}

criterion_group!(benches, bench_albania, bench_kosovo);
criterion_main!(benches);
