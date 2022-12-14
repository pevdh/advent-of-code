use aoc2022::*;

type StructurePath = Vec<(usize, usize)>;

fn parse(raw_input: &str) -> Result<Vec<StructurePath>> {
    Ok(raw_input
        .lines()
        .map(|line| {
            let coords_s = line.split(" -> ");

            coords_s
                .map(|coord_s| {
                    let (x_s, y_s) = coord_s.split_once(',').unwrap();

                    (x_s.parse().unwrap(), y_s.parse().unwrap())
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect())
}

fn task_1(paths: &[StructurePath]) -> Result<usize> {
    let mut map = build_map(paths, (1000, 1000));

    let mut num_sand_units = 0;
    while let Some((sand_unit_x, sand_unit_y)) = simulate_sand_unit(&map) {
        map[[sand_unit_y, sand_unit_x]] = 1;
        num_sand_units += 1;
    }

    Ok(num_sand_units)
}

fn task_2(paths: &[StructurePath]) -> Result<usize> {
    let mut map = build_map(paths, (1000, 1000));

    // Add floor
    let max_y = paths
        .iter()
        .flat_map(|path| path.iter().map(|(_, y)| *y))
        .max()
        .unwrap();

    map.row_mut(max_y + 2).fill(1);

    let mut num_sand_units = 0;
    while let Some((sand_unit_x, sand_unit_y)) = simulate_sand_unit(&map) {
        map[[sand_unit_y, sand_unit_x]] = 1;
        num_sand_units += 1;

        if (sand_unit_x, sand_unit_y) == (500, 0) {
            break;
        }
    }

    Ok(num_sand_units)
}

fn simulate_sand_unit(map: &Array2<u8>) -> Option<(usize, usize)> {
    let is_clear = |x, y| map[[y, x]] == 0;

    let next_pos = |x, y| {
        if is_clear(x, y + 1) {
            Some((x, y + 1))
        } else if is_clear(x - 1, y + 1) {
            Some((x - 1, y + 1))
        } else if is_clear(x + 1, y + 1) {
            Some((x + 1, y + 1))
        } else {
            None
        }
    };

    let (mut x, mut y) = (500, 0usize);
    while let Some((next_x, next_y)) = next_pos(x, y) {
        if next_y == map.nrows() - 1 {
            // We've reached the abyss
            return None;
        }

        x = next_x;
        y = next_y;
    }

    // Sand is at rest
    Some((x, y))
}

fn build_map(structures: &[StructurePath], map_size: (usize, usize)) -> Array2<u8> {
    let mut map = Array2::zeros(map_size);

    for structure in structures {
        for ((from_x, from_y), (to_x, to_y)) in structure.iter().zip(structure[1..].iter()) {
            let step = (
                (*to_x as i64 - *from_x as i64).signum(),
                (*to_y as i64 - *from_y as i64).signum(),
            );
            let mut current = (*from_x as i64, *from_y as i64);

            loop {
                map[(current.1 as usize, current.0 as usize)] = 1;

                if current == (*to_x as i64, *to_y as i64) {
                    break;
                }

                current.0 += step.0;
                current.1 += step.1;
            }
        }
    }

    map
}

aoc_main!(
    day: 14,
    test_input:
    r#"
    498,4 -> 498,6 -> 496,6
    503,4 -> 502,4 -> 502,9 -> 494,9
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 24,
    task_2: task_2,
    expected_2: 93,
);
