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
    for i in 0..100 {
        total_flashes += simulate_step(&mut energy_levels);
    }

    Ok(total_flashes)
}

fn simulate_step(energy_levels: &mut Array2<u32>) -> i32 {
    energy_levels.iter_mut()
        .for_each(|e| *e += 1);

    let mut flashes = 0;
    let mut pos_flashed = HashSet::new();
    loop {
        let to_flash: Vec<(usize, usize)> = energy_levels.indexed_iter()
            .filter(|(_, &val)| {
                val > 9
            })
            .map(|(pos, _)| pos)
            .collect();

        if to_flash.is_empty() {
            break;
        }

        for to_flash_pos in to_flash {
            if pos_flashed.contains(&to_flash_pos) {
                continue;
            }

            flashes += 1;
            energy_levels[to_flash_pos] = 0;
            pos_flashed.insert(to_flash_pos);

            let adjacent = neighbors(energy_levels, to_flash_pos);
            for adjacent_pos in adjacent {
                if pos_flashed.contains(&adjacent_pos) {
                    continue;
                }

                energy_levels[adjacent_pos] += 1;
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

