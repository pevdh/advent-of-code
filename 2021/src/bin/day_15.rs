use aoc2021::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter::FromIterator;

aoc_main!(
    day: 15,
    test_input: r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#,
    parser: parse,
    task_1: task_1,
    expected_1: 40,
    task_2: task_2,
    expected_2: 315,
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

type Position = (usize, usize);

fn task_1(cave: &Array2<u32>) -> Result<u32> {
    let start = (0, 0);
    let end = (cave.nrows() - 1, cave.ncols() - 1);
    let risk = dijkstra(cave, start, end);

    Ok(risk)
}

fn task_2(cave: &Array2<u32>) -> Result<u32> {
    let full_map = generate_full_map(cave);

    let start = (0, 0);
    let end = (full_map.nrows() - 1, full_map.ncols() - 1);
    let risk = dijkstra(&full_map, start, end);

    Ok(risk)
}

fn dijkstra(cave: &Array2<u32>, start: Position, end: Position) -> u32 {
    assert!(cave.nrows() > 1 && cave.ncols() > 1);

    let mut unvisited_nodes = BinaryHeap::new();

    let positions = (0..cave.nrows()).cartesian_product(0..cave.ncols());
    let mut distances: HashMap<Position, u32> =
        HashMap::from_iter(positions.map(|pos| (pos, u32::MAX)));

    unvisited_nodes.push(Reverse((0, start)));
    distances.insert(start, 0);

    while !unvisited_nodes.is_empty() {
        let (risk, position) = unvisited_nodes.pop().unwrap().0;
        for neighbor in neighbors(cave, position) {
            let neighbor_risk = risk + cave[neighbor];

            if neighbor_risk < *distances.get(&neighbor).unwrap() {
                distances.insert(neighbor, neighbor_risk);
                unvisited_nodes.push(Reverse((neighbor_risk, neighbor)));
            }
        }
    }

    return *distances.get(&end).unwrap();
}

fn neighbors(a: &Array2<u32>, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let row = pos.0 as i32;
    let col = pos.1 as i32;
    let mut v = smallvec![
        (row - 1, col),
        (row, col + 1),
        (row + 1, col),
        (row, col - 1),
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

fn generate_full_map(original_map: &Array2<u32>) -> Array2<u32> {
    let mut new_map = Array2::zeros((original_map.nrows() * 5, original_map.ncols() * 5));

    for ((row, col), value) in new_map.indexed_iter_mut() {
        let original_row = row % original_map.nrows();
        let original_col = col % original_map.ncols();

        let original_value = original_map[(original_row, original_col)];

        let row_offset = (row / original_map.nrows()) as u32;
        let col_offset = (col / original_map.ncols()) as u32;
        let mut new_value = original_value + (row_offset + col_offset);

        if new_value > 9 {
            new_value %= 9
        }

        *value = new_value;
    }

    new_map
}
