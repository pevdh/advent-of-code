use std::collections::HashSet;

use aoc2023::*;

aoc_main!(
    day: 21,
    test_input: r#"
    ...........
    .....###.#.
    .###.##..#.
    ..#.#...#..
    ....#.#....
    .##..S####.
    .##..#...#.
    .......##..
    .##.#.####.
    .##..##.##.
    ..........."#,
    task_1: task_1,
    expected_1: 16,
    task_2: task_2,
    expected_2: 0,
);

fn task_1(input: &str) -> Result<usize> {
    let map = Array2::from_2d_text(input)?;

    let (initial_position, _) = map.indexed_iter()
        .find(|&(_, e)| *e == 'S')
        .ok_or_parse_error()?;

    let mut to_visit: HashSet<((usize, usize), u64)> = HashSet::default();
    to_visit.insert((initial_position, 0_u64));

    // let mut garden_plots_reached: HashSet<(usize, usize)> = HashSet::default();
    // garden_plots_reached.insert(initial_position);

    let mut current_positions: HashSet<(usize, usize)> = HashSet::default();
    current_positions.insert(initial_position);

    for _ in 0..64 {
        let mut next_current_positions = HashSet::default();

        for &(row, col) in current_positions.iter() {
            for neighbor in map.von_neumann_neighborhood(&(row, col)) {
                if map[neighbor] == '.' {
                    next_current_positions.insert(neighbor);
                }
            }
        }

        current_positions = next_current_positions;
    }

    Ok(current_positions.len() + 1)
}

fn task_2(input: &str) -> Result<u64> {
    Ok(0)
}
