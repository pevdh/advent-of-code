use aoc2022::*;

aoc_main!(
    day: 22,
    test_input:
    r#"
            ...#
            .#..
            #...
            ....
    ...#.......#
    ........#...
    ..#....#....
    ..........#.
            ...#....
            .....#..
            .#......
            ......#.

    10R5L5R10L4R5L5
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 6032,
    task_2: task_2,
    expected_2: 301,
);

#[derive(Debug)]
struct Input {
    map: Array2<u8>,
    path_description: Vec<DescriptionPart>,
}

#[derive(Debug)]
enum DescriptionPart {
    Forward(i64),
    TurnLeft,
    TurnRight,
}

fn parse(raw_input: &str) -> Result<Input> {
    use nom::branch::alt;
    use nom::character::complete::{char, i64};
    use nom::combinator::map;
    use nom::multi::many0;

    let (map_s, path_description_s) = raw_input.split_once("\n\n").unwrap();

    let rows = map_s.lines().count();
    let cols = map_s.lines().map(|l| l.len()).max().unwrap();
    let mut map_array: Array2<u8> = Array2::from_elem((rows, cols), b' ');
    for (row, line) in map_s.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            map_array[[row, col]] = c as u8;
        }
    }

    let left = map(char('L'), |_| DescriptionPart::TurnLeft);
    let right = map(char('R'), |_| DescriptionPart::TurnRight);
    let forward = map(i64, DescriptionPart::Forward);

    let description = nom_parse(path_description_s, many0(alt((left, right, forward)))).unwrap();

    Ok(Input {
        map: map_array,
        path_description: description,
    })
}

fn task_1(input: &Input) -> Result<usize> {
    let map = &input.map;
    let description = &input.path_description[..];

    // (row, col)
    let initial_position = (
        0usize,
        map.row(0)
            .indexed_iter()
            .filter(|(_, &v)| v == b'.')
            .map(|(c, _)| c)
            .next()
            .unwrap(),
    );

    let ((final_row, final_col), facing) = follow_path(map, description, initial_position, 0);

    Ok(1000 * (final_row + 1) + 4 * (final_col + 1) + facing)
}

// right, down, left, up
const FACINGS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn follow_path(
    map: &Array2<u8>,
    description: &[DescriptionPart],
    initial_position: (usize, usize),
    initial_facing: usize,
) -> ((usize, usize), usize) {
    let rows = map.nrows();
    let cols = map.ncols();

    let mut current_facing_idx = initial_facing;
    let mut current_pos: (i64, i64) = (initial_position.0 as i64, initial_position.1 as i64);

    for part in description {
        match part {
            DescriptionPart::Forward(n) => {
                for _ in 0..*n {
                    let facing = FACINGS[current_facing_idx];
                    let mut next_pos = (current_pos.0 + facing.0, current_pos.1 + facing.1);

                    loop {
                        // Wrap to bottom row
                        if next_pos.0 < 0 {
                            next_pos.0 = (rows - 1) as i64;
                        }
                        // Wrap to first row
                        if next_pos.0 >= rows as i64 {
                            next_pos.0 = 0;
                        }

                        // Wrap to rightmost col
                        if next_pos.1 < 0 {
                            next_pos.1 = (cols - 1) as i64;
                        }

                        // Wrap to leftmost col
                        if next_pos.1 >= cols as i64 {
                            next_pos.1 = 0;
                        }

                        if map[[next_pos.0 as usize, next_pos.1 as usize]] != b' ' {
                            break;
                        }

                        // Keep moving
                        next_pos = (next_pos.0 + facing.0, next_pos.1 + facing.1);
                    }

                    if map[[next_pos.0 as usize, next_pos.1 as usize]] == b'#' {
                        // Encountered a wall
                        break;
                    }

                    current_pos = next_pos;
                }
            }
            DescriptionPart::TurnLeft => {
                current_facing_idx = (FACINGS.len() + current_facing_idx - 1) % FACINGS.len()
            }
            DescriptionPart::TurnRight => {
                current_facing_idx = (current_facing_idx + 1) % FACINGS.len()
            }
        }
    }

    (
        (current_pos.0 as usize, current_pos.1 as usize),
        current_facing_idx,
    )
}

fn task_2(_input: &Input) -> Result<i64> {
    Ok(0)
}
