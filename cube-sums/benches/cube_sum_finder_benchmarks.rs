use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn awful(range: usize) {
//    let finder = AwfulCubeSubFinder::new();
//    let cube_sums = finder.find_cube_sums(range).collect();
}

fn cube_sum_finder_benchmarks(c: &mut Criterion) {
    c.bench_function("awful 100", |b| b.iter(|| awful(black_box(100))));
}

criterion_group!(benches, cube_sum_finder_benchmarks);
criterion_main!(benches);