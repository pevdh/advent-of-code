use anyhow::anyhow;
use aoc2021::*;
use itertools::Itertools;
use ndarray::Array2;
use std::collections::VecDeque;

aoc_main!(
    day: 9,
    test_input: r#"2199943210
3987894921
9856789892
8767896789
9899965678"#,
    parser: parse,
    task_1: task_1,
    expected_1: 15,
    task_2: task_2,
    expected_2: 1134,
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

fn task_1(heightmap: &Array2<u32>) -> Result<u32> {
    let total_height = low_points(heightmap)
        .map(|low_point| heightmap[low_point] + 1)
        .sum();

    Ok(total_height)
}

fn task_2(heightmap: &Array2<u32>) -> Result<i32> {
    let total_size = low_points(heightmap)
        // Find three largest basins and multiply their sizes
        .map(|low_point| basin_size(low_point, heightmap))
        .sorted()
        .rev()
        .take(3)
        .fold(1, |sz, prev| prev * sz);

    Ok(total_size)
}

fn low_points(heightmap: &Array2<u32>) -> impl Iterator<Item = (usize, usize)> + '_ {
    heightmap
        .indexed_iter()
        .filter(move |(pos, &height)| {
            height
                < neighbors(heightmap, *pos)
                    .map(|neighbor| heightmap[neighbor])
                    .min()
                    .unwrap()
        })
        .map(|(pos, _)| pos)
}

fn neighbors(a: &Array2<u32>, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let row = pos.0 as i32;
    let col = pos.1 as i32;
    let mut v = vec![
        (row - 1, col),
        (row, col + 1),
        (row + 1, col),
        (row, col - 1),
    ];

    v.retain(|&(neighbor_row, neighbor_col)| {
        neighbor_row >= 0
            && neighbor_col >= 0
            && neighbor_row < a.nrows() as i32
            && neighbor_col < a.ncols() as i32
    });

    v.into_iter()
        .map(|(pos_i, pos_j)| (pos_i as usize, pos_j as usize))
}

fn basin_size(low_point: (usize, usize), heightmap: &Array2<u32>) -> i32 {
    let mut to_visit = VecDeque::new();
    let mut visited = Array2::zeros((heightmap.nrows(), heightmap.ncols()));

    let mut size = 0;
    to_visit.push_back(low_point);
    while let Some(current) = to_visit.pop_front() {
        if visited[current] == 1 {
            continue;
        }

        visited[current] = 1;
        size += 1;

        to_visit.extend(neighbors(heightmap, current).filter(|&n| heightmap[n] < 9));
    }

    size
}
