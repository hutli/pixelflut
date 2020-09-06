use criterion::{criterion_group, criterion_main, Criterion};
use pixelflut::packet::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("packet_build_nonrandom", |b| {
        let mut packet_to_build = Packet::new(8);
        b.iter(|| {
            packet_to_build.add_letter('▓');
            packet_to_build.reset()
        })
    });
    c.bench_function("packet_build_random", |b| {
        let mut packet_to_build = Packet::new(8);
        b.iter(|| {
            packet_to_build.add_letter(' ');
            packet_to_build.reset()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
