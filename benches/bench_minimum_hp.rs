use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use dungeon_game::Solution;

fn bench_calculate_minimum_hp_small_mixed(c: &mut Criterion) {
    // A 3x3 dungeon with mixed values
    let dungeon = vec![
        vec![-2, -3, 3],
        vec![-5, -10, 1],
        vec![10, 30, -5],
    ];
    c.bench_function("calculate_minimum_hp_small_mixed", |b| {
        b.iter(|| Solution::calculate_minimum_hp(black_box(dungeon.clone())))
    });
}

fn bench_calculate_minimum_hp_large_all_negative(c: &mut Criterion) {
    // A 200x200 dungeon filled with -1
    let dungeon = vec![vec![-1; 200]; 200];
    c.bench_function("calculate_minimum_hp_large_all_negative", |b| {
        b.iter(|| Solution::calculate_minimum_hp(black_box(dungeon.clone())))
    });
}

fn bench_calculate_minimum_hp_small_all_positive(c: &mut Criterion) {
    // A 10x10 dungeon filled with 5
    let dungeon = vec![vec![5; 10]; 10];
    c.bench_function("calculate_minimum_hp_small_all_positive", |b| {
        b.iter(|| Solution::calculate_minimum_hp(black_box(dungeon.clone())))
    });
}

fn bench_calculate_minimum_hp_small_all_negative(c: &mut Criterion) {
    // A 10x10 dungeon filled with -5
    let dungeon = vec![vec![-5; 10]; 10];
    c.bench_function("calculate_minimum_hp_small_all_negative", |b| {
        b.iter(|| Solution::calculate_minimum_hp(black_box(dungeon.clone())))
    });
}

fn bench_calculate_minimum_hp_large_mixed(c: &mut Criterion) {
    // A 200x200 dungeon with alternating large negatives and positives
    let size = 200;
    let mut dungeon = vec![vec![0; size]; size];
    for i in 0..size {
        for j in 0..size {
            if (i + j) % 2 == 0 {
                dungeon[i][j] = -1000;
            } else {
                dungeon[i][j] = 1000;
            }
        }
    }
    c.bench_function("calculate_minimum_hp_large_mixed", |b| {
        b.iter(|| Solution::calculate_minimum_hp(black_box(dungeon.clone())))
    });
}

criterion_group!(benches,
    bench_calculate_minimum_hp_small_mixed,
    bench_calculate_minimum_hp_large_all_negative,
    bench_calculate_minimum_hp_small_all_positive,
    bench_calculate_minimum_hp_small_all_negative,
    bench_calculate_minimum_hp_large_mixed,
);
criterion_main!(benches); 