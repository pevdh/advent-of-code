use aoc2021::*;
use std::collections::BTreeSet;
use std::iter::FromIterator;

aoc_main!(
    day: 11,
    test_input: r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#,
    parser: parse,
    task_1: task_1,
    expected_1: 1656,
    task_2: task_2,
    expected_2: 195,
);

fn parse(raw_input: &str) -> Result<Array2<u32>> {
    Array2::from_2d_text(raw_input)
}

fn task_1(energy_levels: &Array2<u32>) -> Result<i32> {
    let mut energy_levels = energy_levels.clone();

    let mut total_flashes = 0;
    for _ in 0..100 {
        total_flashes += simulate_step(&mut energy_levels);
    }

    Ok(total_flashes)
}

fn task_2(energy_levels: &Array2<u32>) -> Result<i32> {
    let mut energy_levels = energy_levels.clone();
    for i in 0.. {
        let flashes = simulate_step(&mut energy_levels);

        if flashes as usize == energy_levels.len() {
            return Ok(i + 1);
        }
    }

    unreachable!()
}

fn simulate_step(energy_levels: &mut Array2<u32>) -> i32 {
    energy_levels.iter_mut().for_each(|e| *e += 1);

    let mut flashes = 0;
    let mut pos_flashed = Array2::zeros((energy_levels.nrows(), energy_levels.ncols()));
    let mut to_visit: BTreeSet<(usize, usize)> = BTreeSet::from_iter(
        energy_levels
            .indexed_iter()
            .filter(|(_, &val)| val > 9)
            .map(|(pos, _)| pos),
    );

    while let Some(current) = to_visit.iter().next().cloned() {
        to_visit.remove(&current);

        flashes += 1;
        energy_levels[current] = 0;
        pos_flashed[current] = 1;

        let adjacent = energy_levels
            .moore_neighborhood(&current)
            .filter(|&neighbor_pos| pos_flashed[neighbor_pos] == 0);

        for neighbor_pos in adjacent {
            energy_levels[neighbor_pos] += 1;

            if energy_levels[neighbor_pos] > 9 {
                to_visit.insert(neighbor_pos);
            }
        }
    }

    flashes
}
