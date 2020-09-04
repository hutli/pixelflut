use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pixelflut::packet::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("packet_build", |b| {
        let mut packet_to_build = Packet::new(8);
        b.iter(|| {
            packet_to_build.add_string("███████████████");
            packet_to_build.reset()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
