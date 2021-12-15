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

fn parse(raw_input: &str) -> Result<Array2<u8>> {
    Array2::from_2d_text(raw_input)
}

fn task_1(cave_risk_map: &Array2<u8>) -> Result<u32> {
    let start = (0, 0);
    let goal = (cave_risk_map.nrows() - 1, cave_risk_map.ncols() - 1);

    let neighbor_fn = |pos| cave_risk_map.von_neumann_neighborhood(&pos);
    let weight_fn = |pos| cave_risk_map[pos] as u32;

    let risk = dijkstra(start, goal, neighbor_fn, weight_fn);

    risk.ok_or_else(|| anyhow!("Unable to find path from {:?} to {:?}", start, goal))
}

fn task_2(cave_risk_map: &Array2<u8>) -> Result<u32> {
    let full_cave_risk_map = generate_full_map(cave_risk_map, 5);

    let start = (0, 0);
    let goal = (
        full_cave_risk_map.nrows() - 1,
        full_cave_risk_map.ncols() - 1,
    );

    let neighbor_fn = |pos| full_cave_risk_map.von_neumann_neighborhood(&pos);
    let weight_fn = |pos| full_cave_risk_map[pos] as u32;

    let total_risk = dijkstra(start, goal, neighbor_fn, weight_fn);

    total_risk.ok_or_else(|| anyhow!("Unable to find path from {:?} to {:?}", start, goal))
}

fn dijkstra<T, NeighborFn, NeighborFnRet, WeightFn>(
    start: T,
    goal: T,
    neighbor_fn: NeighborFn,
    weight_fn: WeightFn,
) -> Option<u32>
where
    T: Ord + Hash + PartialEq + Eq + Copy,
    NeighborFn: Fn(T) -> NeighborFnRet,
    NeighborFnRet: Iterator<Item = T>,
    WeightFn: Fn(T) -> u32,
{
    let mut heap = BinaryHeap::new();
    let mut weights = HashMap::new();

    heap.push(Reverse((0, start)));
    weights.insert(start, 0);

    while let Some(Reverse((risk, position))) = heap.pop() {
        if position == goal {
            return Some(risk);
        }

        for neighbor in neighbor_fn(position) {
            let neighbor_risk = risk + weight_fn(neighbor);

            if neighbor_risk < *weights.get(&neighbor).unwrap_or(&u32::MAX) {
                weights.insert(neighbor, neighbor_risk);
                heap.push(Reverse((neighbor_risk, neighbor)));
            }
        }
    }

    None
}

fn generate_full_map(original_map: &Array2<u8>, multiplier: usize) -> Array2<u8> {
    let mut new_map = Array2::zeros((
        original_map.nrows() * multiplier,
        original_map.ncols() * multiplier,
    ));

    for ((row, col), value) in new_map.indexed_iter_mut() {
        let original_row = row % original_map.nrows();
        let original_col = col % original_map.ncols();

        let original_value = original_map[(original_row, original_col)] as u32;

        let row_offset = (row / original_map.nrows()) as u32;
        let col_offset = (col / original_map.ncols()) as u32;
        let mut new_value = original_value + (row_offset + col_offset);

        if new_value > 9 {
            new_value %= 9
        }

        *value = new_value as u8;
    }

    new_map
}
