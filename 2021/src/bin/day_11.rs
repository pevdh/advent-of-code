use std::collections::BTreeSet;
use std::iter::FromIterator;
use aoc2021::*;

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
    let cols = raw_input
        .lines()
        .next()
        .map(|l| l.len())
        .ok_or(anyhow!("Empty input"))?;
    let rows = raw_input.lines().count();

    let data: Result<Vec<u32>> = raw_input
        .replace('\n', "")
        .chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or(anyhow!("Unable to convert char to digit"))
        })
        .collect();

    Ok(Array2::from_shape_vec((rows, cols), data?)?)
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
    energy_levels.iter_mut()
        .for_each(|e| *e += 1);

    let mut flashes = 0;
    let mut pos_flashed = Array2::zeros((energy_levels.nrows(), energy_levels.ncols()));
    let mut to_visit: BTreeSet<(usize, usize)> = BTreeSet::from_iter(energy_levels
        .indexed_iter()
        .filter(|(_, &val)| val > 9)
        .map(|(pos, _)| pos));

    while let Some(current) = to_visit.iter().next().cloned() {
        to_visit.remove(&current);

        flashes += 1;
        energy_levels[current] = 0;
        pos_flashed[current] = 1;

        let adjacent = neighbors(energy_levels, current)
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

fn neighbors(a: &Array2<u32>, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let row = pos.0 as i32;
    let col = pos.1 as i32;
    let mut v = smallvec![
        (row - 1, col),
        (row, col + 1),
        (row + 1, col),
        (row, col - 1),
        (row - 1, col - 1),
        (row - 1, col + 1),
        (row + 1, col - 1),
        (row + 1, col + 1),
    ];

    v.retain(|&mut (neighbor_row, neighbor_col)| {
        neighbor_row >= 0
            && neighbor_col >= 0
            && neighbor_row < a.nrows() as i32
            && neighbor_col < a.ncols() as i32
    });

    v.into_iter()
        .map(|(pos_i, pos_j)| (pos_i as usize, pos_j as usize))
}
