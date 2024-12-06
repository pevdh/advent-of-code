use std::collections::HashSet;

use aoc2024::*;

aoc_main!(
    day: 6,
    test_input: r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#,
    task_1: task_1,
    expected_1: 41,
    task_2: task_2,
    expected_2:6,
);

fn task_1(input: &str) -> Result<usize> {
    let grid = Array2::from_2d_text(input)?;

    let (initial_position, initial_dir) = grid
        .indexed_iter()
        .find(|&(_, v)| ['^', 'v', '>', '<'].contains(v))
        .unwrap();

    let (visited, _did_loop) = simulate(&grid, initial_position, *initial_dir);

    Ok(visited.iter().map(|&(_, pos)| pos).unique().count())
}

fn task_2(input: &str) -> Result<usize> {
    let mut grid = Array2::from_2d_text(input)?;

    let (initial_position, initial_dir) = grid
        .indexed_iter()
        .map(|(pos, v)| (pos, *v))
        .find(|&(_, v)| ['^', 'v', '>', '<'].contains(&v))
        .unwrap();

    let (visited, _) = simulate(&grid, initial_position, initial_dir);

    let num_obstacle_locations = visited
        .iter()
        .map(|(_dir, pos)| pos)
        .unique()
        .filter(|&pos| *pos != initial_position)
        .filter(|&pos| {
            grid[*pos] = '#';
            let (_, did_loop) = simulate(&grid, initial_position, initial_dir);
            grid[*pos] = '.';

            did_loop
        })
        .count();

    Ok(num_obstacle_locations)
}

#[allow(clippy::type_complexity)]
fn simulate(
    grid: &Array2<char>,
    initial_position: (usize, usize),
    initial_dir: char,
) -> (HashSet<(char, (usize, usize))>, bool) {
    let mut current_position = initial_position;
    let mut current_dir = initial_dir;
    let mut visited: HashSet<_> = HashSet::default();
    visited.insert((current_dir, current_position));

    loop {
        match try_move_into_direction(grid, &current_position, current_dir) {
            Some(new_pos) => {
                if grid[new_pos] == '#' {
                    current_dir = turn_right(current_dir);
                } else {
                    current_position = new_pos;

                    if visited.contains(&(current_dir, current_position)) {
                        return (visited, true);
                    }
                    visited.insert((current_dir, current_position));
                }
            }
            None => return (visited, false),
        }
    }
}

fn try_move_into_direction(
    grid: &Array2<char>,
    pos: &(usize, usize),
    dir: char,
) -> Option<(usize, usize)> {
    match dir {
        '^' if pos.0 > 0 => Some((pos.0 - 1, pos.1)),
        'v' if pos.0 + 1 < grid.nrows() => Some((pos.0 + 1, pos.1)),
        '<' if pos.1 > 0 => Some((pos.0, pos.1 - 1)),
        '>' if pos.1 + 1 < grid.ncols() => Some((pos.0, pos.1 + 1)),
        _ => None,
    }
}

fn turn_right(dir: char) -> char {
    match dir {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => panic!("invalid dir: {}", dir),
    }
}
