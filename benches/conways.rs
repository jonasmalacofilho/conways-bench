use criterion::*;

use conways_bench::*;

const KB: usize = 1024;

fn prepare<B>(size: usize) -> Game<B>
where
    Grid<B>: GameGrid + Clone,
{
    let mut grid = Grid::new((size, size));
    for i in 0..size {
        for j in 0..size {
            if i % 2 != 0 && j % 2 != 0 {
                grid.set((i, j), true);
            }
        }
    }
    Game::new(grid)
}

fn game_of_life(c: &mut Criterion) {
    let sizes = [8, 64, 256, KB, 4 * KB, 16 * KB];

    let mut group = c.benchmark_group("Game of Life");
    group.sample_size(20);

    for size in sizes {
        let id = BenchmarkId::new("Vec<bool>", format!("{size}×{size}"));
        group.throughput(Throughput::Elements((size * size) as _));

        let mut game = prepare::<Vec<bool>>(size);
        group.bench_with_input(id, &(), |b, &_| {
            b.iter(|| {
                game.update();
            })
        });
        black_box(&game);
    }

    for size in sizes {
        let id = BenchmarkId::new("bitvec::vec::BitVec", format!("{size}×{size}"));
        group.throughput(Throughput::Elements((size * size) as _));

        let mut game = prepare::<bitvec::vec::BitVec>(size);
        group.bench_with_input(id, &(), |b, &_| {
            b.iter(|| {
                game.update();
            })
        });
        black_box(&game);
    }

    for size in sizes {
        let id = BenchmarkId::new("bit_vec::BitVec", format!("{size}×{size}"));
        group.throughput(Throughput::Elements((size * size) as _));

        let mut game = prepare::<bit_vec::BitVec>(size);
        group.bench_with_input(id, &(), |b, &_| {
            b.iter(|| {
                game.update();
            })
        });
        black_box(&game);
    }

    group.finish();
}

criterion_group!(benches, game_of_life,);
criterion_main!(benches);
