use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use minesweeper_core::{Difficulty, Game, Point};

fn select_cell(game: &mut Game) {
    game.selected_at(black_box(Point { x: 5, y: 5 }));
}

fn clear_all(game: &mut Game) {
    {
        game.clear_all();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("User selects cell", |b| {
        b.iter(|| {
            select_cell(&mut black_box(Game::new(Difficulty::Hard)));
        })
    });
}

fn criterion_benchmark_2(c: &mut Criterion) {
    c.bench_function("Clear all cells", |b| {
        b.iter(|| {
            clear_all(&mut black_box(Game::new(Difficulty::Hard)));
        })
    });
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark_2);
criterion_main!(benches);
