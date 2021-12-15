use aoc2021::*;
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

    // let start = (0, 0);
    // let end = (full_map.nrows() - 1, full_map.ncols() - 1);
    // let risk = dijkstra(&full_map, start, end);

    Ok(0)
}

fn dijkstra(cave: &Array2<u32>, start: Position, end: Position) -> u32 {
    assert!(cave.nrows() > 1 && cave.ncols() > 1);

    let mut unvisited_nodes: HashSet<Position> =
        HashSet::from_iter((0..cave.nrows()).cartesian_product(0..cave.ncols()));
    let mut distances: HashMap<Position, u32> =
        HashMap::from_iter(unvisited_nodes.iter().copied().map(|pos| (pos, u32::MAX)));

    distances.insert(start, 0);

    let mut current = (0_usize, 0_usize);

    loop {
        let unvisited_neighbors =
            neighbors(cave, current).filter(|neighbor| unvisited_nodes.contains(neighbor));

        let tentative_distance = *distances.get(&current).unwrap();

        for neighbor in unvisited_neighbors {
            let distance_to_neighbor = tentative_distance + cave[neighbor];

            let neighbor_current_tentative_distance = distances.get_mut(&neighbor).unwrap();

            if *neighbor_current_tentative_distance > distance_to_neighbor {
                *neighbor_current_tentative_distance = distance_to_neighbor;
            }
        }

        unvisited_nodes.remove(&current);

        if !unvisited_nodes.contains(&end) {
            return *distances.get(&end).unwrap();
        }

        current = *unvisited_nodes
            .iter()
            .min_by(|pos_a, pos_b| {
                distances
                    .get(pos_a)
                    .unwrap()
                    .cmp(distances.get(pos_b).unwrap())
            })
            .unwrap();
    }
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

fn print_grid(g: &Array2<u32>) {
    for row in 0..g.nrows() {
        for col in 0..g.ncols() {
            print!("{}", g[[row, col]]);
        }
        println!()
    }
}
