use aoc2023::*;

aoc_main!(
    day: 14,
    test_input: r#"
    O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#....
    "#,
    task_1: task_1,
    expected_1: 136,
    task_2: task_2,
    expected_2: 64,
);

fn task_1(input: &str) -> Result<u64> {
    let mut platform = Array2::from_2d_text(input)?;

    tilt_north(&mut platform);

    Ok(calculate_total_load(&platform))
}

fn task_2(input: &str) -> Result<u64> {
    let mut platform = Array2::from_2d_text(input)?;

    let mut load_values = vec![];

    loop {
        perform_cycle(&mut platform);

        let total_load = calculate_total_load(&platform);
        load_values.push(total_load);

        if let Some((cycle, cycle_offset)) = detect_cycle_and_offset(&load_values) {
            let v = cycle[(1_000_000_000 - cycle_offset - 1) % cycle.len()];
            return Ok(v);
        }
    }
}

fn perform_cycle(platform: &mut Array2<char>) {
    tilt_north(platform);
    tilt_west(platform);
    tilt_south(platform);
    tilt_east(platform);
}

fn tilt_north(platform: &mut Array2<char>) {
    for row in 0..platform.nrows() {
        for col in 0..platform.ncols() {
            if platform[(row, col)] != 'O' {
                continue;
            }

            let new_row = (0..row)
                .rev()
                .take_while(|&row| platform[(row, col)] == '.')
                .min()
                .unwrap_or(row);

            platform[(row, col)] = '.';
            platform[(new_row, col)] = 'O';
        }
    }
}

fn tilt_west(platform: &mut Array2<char>) {
    for row in 0..platform.nrows() {
        for col in 0..platform.ncols() {
            if platform[(row, col)] != 'O' {
                continue;
            }

            let new_col = (0..col)
                .rev()
                .take_while(|&c| platform[(row, c)] == '.')
                .min()
                .unwrap_or(col);

            platform[(row, col)] = '.';
            platform[(row, new_col)] = 'O';
        }
    }
}

fn tilt_south(platform: &mut Array2<char>) {
    for row in (0..platform.nrows()).rev() {
        for col in 0..platform.ncols() {
            if platform[(row, col)] != 'O' {
                continue;
            }

            let new_row = ((row + 1)..platform.nrows())
                .take_while(|&r| platform[(r, col)] == '.')
                .max()
                .unwrap_or(row);

            platform[(row, col)] = '.';
            platform[(new_row, col)] = 'O';
        }
    }
}

fn tilt_east(platform: &mut Array2<char>) {
    for row in 0..platform.nrows() {
        for col in (0..platform.ncols()).rev() {
            if platform[(row, col)] != 'O' {
                continue;
            }

            let new_col = ((col + 1)..platform.ncols())
                .take_while(|&c| platform[(row, c)] == '.')
                .max()
                .unwrap_or(col);

            platform[(row, col)] = '.';
            platform[(row, new_col)] = 'O';
        }
    }
}

fn calculate_total_load(platform: &Array2<char>) -> u64 {
    platform
        .indexed_iter()
        .filter(|&(_, e)| *e == 'O')
        .map(|((row, _), _)| (platform.nrows() - row) as u64)
        .sum()
}

fn detect_cycle_and_offset(values: &[u64]) -> Option<(Vec<u64>, usize)> {
    for cycle_length in 2..(values.len() / 2) {
        for offset in 0..(values.len() - cycle_length - 1) {
            let cycle = values[offset..].iter().take(cycle_length).cycle();
            let remaining_values = &values[offset + cycle_length..];

            let cycle_found = remaining_values
                .iter()
                .eq(cycle.take(remaining_values.len()));

            if cycle_found {
                let cycle = values[offset..]
                    .iter()
                    .take(cycle_length)
                    .copied()
                    .collect::<Vec<u64>>();

                return Some((cycle, offset));
            }
        }
    }

    None
}
