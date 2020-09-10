use criterion::measurement::Measurement;
use criterion::BenchmarkGroup;
use criterion::{criterion_group, criterion_main, Bencher, BenchmarkId, Criterion};
use pixelflut::alphabet::Alphabet;
use pixelflut::alphabet_bit_array_2::BitArrayAlphabet2;
use pixelflut::alphabet_bitarr::BitMapAlphabet;
use pixelflut::alphabet_fast::BinarySearchAlphabet;
use pixelflut::packet::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Non-random Packet Build");
    group.bench_function("Bit Array", |b| {
        create_packet_bench::<BitMapAlphabet, _>(b, '█')
    });
    group.bench_function("Binary Search", |b| {
        create_packet_bench::<BinarySearchAlphabet, _>(b, '█')
    });
    group.bench_function("Binarcary Array 2", |b| {
        create_packet_bench::<BitArrayAlphabet2, _>(b, '█')
    });
    group.finish();

    let mut group = c.benchmark_group("Random Packet Build");
    group.bench_function("Bit Array", |b| {
        create_packet_bench::<BitMapAlphabet, _>(b, ' ')
    });
    group.bench_function("Binary Search", |b| {
        create_packet_bench::<BinarySearchAlphabet, _>(b, ' ')
    });
    group.bench_function("Binarcary Array 2", |b| {
        create_packet_bench::<BitArrayAlphabet2, _>(b, ' ')
    });
    group.finish();
}

fn create_packet_bench<A: Alphabet, M: Measurement>(b: &mut Bencher<M>, packet: char) {
    let mut packet_to_build = Packet::<A>::new(8);
    b.iter(|| {
        packet_to_build.add_letter(packet);
        packet_to_build.reset();
    })
}

// fn add_random_bench_to_group<A: Alphabet, M: Measurement>(g: &mut BenchmarkGroup<M>, name: String) {
//     g.bench_function(BenchmarkId::new(name, |b| {
//         let mut packet_to_build = Packet::<A>::new(8);
//         b.iter(|| {
//             packet_to_build.add_letter(' ');
//             packet_to_build.reset()
//         })
//     }));
// }


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
