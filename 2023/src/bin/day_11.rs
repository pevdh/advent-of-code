use aoc2023::*;

aoc_main!(
    day: 11,
    test_input: r#"
    ...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#....."#,
    task_1: task_1,
    expected_1: 374,
    task_2: task_2,
    expected_2: 82000210,
);

fn task_1(input: &str) -> Result<u64> {
    let mut image: Vec<char> = vec![];
    let mut nrows = 0;
    let mut ncols = 0;

    let mut columns_to_duplicate = vec![];
    for (col_idx, column) in input.columns().enumerate() {
        ncols += 1;

        if column.chars().all(|ch| ch == '.') {
            ncols += 1;
            columns_to_duplicate.push(col_idx);
        }
    }

    for line in input.lines() {
        let mut row = vec![];
        for (ch_idx, ch) in line.chars().enumerate() {
            row.push(ch);

            if columns_to_duplicate.contains(&ch_idx) {
                row.push(ch);
            }
        }

        if row.iter().all(|&c| c == '.') {
            nrows += 1;
            image.extend(row.iter());
        }

        nrows += 1;
        image.extend(row.iter());
    }

    let image = Array2::from_shape_vec((nrows, ncols), image)?;

    let galaxies = image
        .indexed_iter()
        .filter(|&(_pos, e)| *e == '#')
        .map(|(pos, _)| pos);

    let total_distance = galaxies
        .tuple_combinations()
        .map(|(from, to)| distance(from, to))
        .sum();

    Ok(total_distance)
}

fn distance(a: (usize, usize), b: (usize, usize)) -> u64 {
    let hor_dist = ((b.1 as i64) - (a.1 as i64)).abs();
    let vert_dist = ((b.0 as i64) - (a.0 as i64)).abs();

    hor_dist as u64 + vert_dist as u64
}

fn task_2(input: &str) -> Result<u64> {
    let image = Array2::from_2d_text(input)?;

    let columns_to_duplicate: Vec<usize> = image
        .columns()
        .into_iter()
        .enumerate()
        .filter(|(_col_idx, col)| col.iter().all(|&v| v == '.'))
        .map(|(col_idx, _)| col_idx)
        .collect();

    let rows_to_duplicate: Vec<usize> = image
        .rows()
        .into_iter()
        .enumerate()
        .filter(|(_row_idx, row)| row.iter().all(|&v| v == '.'))
        .map(|(row_idx, _)| row_idx)
        .collect();

    let galaxies = image
        .indexed_iter()
        .filter(|&(_pos, e)| *e == '#')
        .map(|(pos, _)| pos);

    let total_distance = galaxies
        .tuple_combinations()
        .map(|(from, to)| distance2(&columns_to_duplicate, &rows_to_duplicate, from, to))
        .sum();

    Ok(total_distance)
}

fn distance2(
    duplicate_cols: &[usize],
    duplicate_rows: &[usize],
    from: (usize, usize),
    to: (usize, usize),
) -> u64 {
    use std::cmp::{max, min};

    let min_col = min(from.1, to.1);
    let min_row = min(from.0, to.0);
    let max_col = max(from.1, to.1);
    let max_row = max(from.0, to.0);

    let mut num_duplicate_rows = 0;
    for &duplicate_row in duplicate_rows {
        if duplicate_row > min_row && duplicate_row < max_row {
            num_duplicate_rows += 1;
        }
    }

    let mut num_duplicate_cols = 0;
    for &duplicate_col in duplicate_cols {
        if duplicate_col > min_col && duplicate_col < max_col {
            num_duplicate_cols += 1;
        }
    }

    let scale = 1_000_000;
    let hor_dist = ((to.1 as i64) - (from.1 as i64)).abs() + num_duplicate_cols * (scale - 1);
    let vert_dist = ((to.0 as i64) - (from.0 as i64)).abs() + num_duplicate_rows * (scale - 1);

    (hor_dist + vert_dist) as u64
}
