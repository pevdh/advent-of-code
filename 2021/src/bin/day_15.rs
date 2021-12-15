use aoc2021::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::hash::Hash;

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

fn task_1(cave_risk_map: &Array2<u32>) -> Result<u32> {
    let start = (0, 0);
    let end = (cave_risk_map.nrows() - 1, cave_risk_map.ncols() - 1);

    let neighbor_fn = |pos| cave_risk_map.von_neumann_neighborhood(&pos);
    let weight_fn = |pos| cave_risk_map[pos];

    let risk = dijkstra(start, end, neighbor_fn, weight_fn);

    risk.ok_or_else(|| anyhow!("Unable to find path from {:?} to {:?}", start, end))
}

fn task_2(cave_risk_map: &Array2<u32>) -> Result<u32> {
    let full_cave_risk_map = generate_full_map(cave_risk_map);

    let start = (0, 0);
    let end = (
        full_cave_risk_map.nrows() - 1,
        full_cave_risk_map.ncols() - 1,
    );

    let neighbor_fn = |pos| full_cave_risk_map.von_neumann_neighborhood(&pos);
    let weight_fn = |pos| full_cave_risk_map[pos];

    let total_risk = dijkstra(start, end, neighbor_fn, weight_fn);

    total_risk.ok_or_else(|| anyhow!("Unable to find path from {:?} to {:?}", start, end))
}

fn dijkstra<T, NeighborFn, NeighborFnRet, WeightFn>(
    start: T,
    end: T,
    neighbor_fn: NeighborFn,
    weight_fn: WeightFn,
) -> Option<u32>
where
    T: Ord + Hash + PartialEq + Eq + Copy,
    NeighborFn: Fn(T) -> NeighborFnRet,
    NeighborFnRet: Iterator<Item = T>,
    WeightFn: Fn(T) -> u32,
{
    let mut unvisited_nodes = BinaryHeap::new();
    let mut weights = HashMap::new();

    unvisited_nodes.push(Reverse((0, start)));
    weights.insert(start, 0);

    while !unvisited_nodes.is_empty() {
        let (risk, position) = unvisited_nodes.pop().unwrap().0;
        for neighbor in neighbor_fn(position) {
            let neighbor_risk = risk + weight_fn(neighbor);

            if neighbor_risk < *weights.get(&neighbor).unwrap_or(&u32::MAX) {
                weights.insert(neighbor, neighbor_risk);
                unvisited_nodes.push(Reverse((neighbor_risk, neighbor)));
            }
        }
    }

    return weights.get(&end).copied();
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
