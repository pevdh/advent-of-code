use ndarray::Array2;
use aoc2021::*;
use anyhow::anyhow;

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
    let cols = raw_input.lines().next().map(|l| l.len())
        .ok_or(anyhow!("Empty input"))?;
    let rows = raw_input.lines().count();


    let data: Result<Vec<u32>> = raw_input
        .replace('\n', "")
        .chars()
        .map(|c| c.to_digit(10).ok_or(anyhow!("Unable to convert char to digit")))
        .collect();

    Ok(Array2::from_shape_vec((rows, cols), data?)?)
}

fn task_1(input: &Array2<u32>) -> Result<u32> {
    let mut result = 0;
    for low_point in low_points(input) {
        result += input[low_point] + 1;
    }

    Ok(result)
}

fn low_points(input: &Array2<u32>) -> Vec<(usize, usize)> {
    let mut low_points = vec![];

    for i in 0..input.nrows() {
        for j in 0..input.ncols() {
            let neighbors = neighbors((i, j), input.nrows(), input.ncols());
            let height = input[[i, j]];

            let neighbor_min = neighbors.iter()
                .map(|&(neighbor_row, neighbor_col)| {
                    input[[neighbor_row, neighbor_col]]
                })
                .min()
                .unwrap();

            if height < neighbor_min {
                low_points.push((i, j));
            }
        }
    }

    low_points
}

fn neighbors(pos: (usize, usize), nrows: usize, ncols: usize) -> Vec<(usize, usize)> {
    let row = pos.0 as i32;
    let col = pos.1 as i32;
    let mut v = vec![
        (row - 1, col),
        (row, col + 1),
        (row + 1, col),
        (row, col - 1),
    ];

    v.retain(|&(neighbor_row, neighbor_col)| {
        neighbor_row >= 0 && neighbor_col >= 0 && neighbor_row < nrows as i32 && neighbor_col < ncols as i32
    });

    v.iter().map(|&(a, b)| (a as usize, b as usize)).collect()
}

fn task_2(input: &Array2<u32>) -> Result<i32> {
    let mut basin_sizes = vec![];

    for low_point in low_points(input) {
        basin_sizes.push(basin_size(low_point, input));
    }

    basin_sizes.sort_unstable();

    // Find three largest basins and multiply their sizes
    Ok(basin_sizes.iter().rev()
        .take(3)
        .fold(1, |sz, prev| prev * sz))
}

fn basin_size(low_point: (usize, usize), input: &Array2<u32>) -> i32 {
    fn basin_size_rec(pos: (usize, usize), input: &Array2<u32>, visited: &mut HashSet<(usize, usize)>) -> i32 {
        visited.insert(pos.clone());

        let mut neighbor_sizes = vec![];
        for neighbor in neighbors(pos, input.nrows(), input.ncols()).into_iter()
            .filter(|&neighbor| input[neighbor] > input[pos] && input[neighbor] < 9) {

            if !visited.contains(&neighbor) {
                neighbor_sizes.push(basin_size_rec(neighbor, input, visited));
            }
        }

        return neighbor_sizes.into_iter().sum::<i32>() + 1;
    }

    let mut visited = HashSet::new();
    return basin_size_rec(low_point, input, &mut visited);
}
