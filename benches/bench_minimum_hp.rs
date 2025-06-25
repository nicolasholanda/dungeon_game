use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use dungeon_game::Solution;

fn bench_calculate_minimum_hp(c: &mut Criterion) {
    let dungeon = vec![
        vec![-2, -3, 3],
        vec![-5, -10, 1],
        vec![10, 30, -5],
    ];
    c.bench_function("calculate_minimum_hp", |b| {
        b.iter(|| Solution::calculate_minimum_hp(black_box(dungeon.clone())))
    });
}

criterion_group!(benches, bench_calculate_minimum_hp);
criterion_main!(benches); 