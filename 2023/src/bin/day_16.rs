use rayon::prelude::*;
use std::collections::HashSet;

use aoc2023::*;

aoc_main!(
    day: 16,
    test_input: r#"
    .|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|...."#,
    task_1: task_1,
    expected_1: 46,
    task_2: task_2,
    expected_2: 51,
);

fn task_1(input: &str) -> Result<u64> {
    let layout = Array2::from_2d_text(input)?;

    Ok(energize_tiles(&layout, (0_i64, 0_i64), Dir::Left))
}

fn task_2(input: &str) -> Result<u64> {
    let layout = Array2::from_2d_text(input)?;

    let first_col = (0..layout.nrows()).map(|row| ((row as i64, 0_i64), Dir::Left));
    let first_row = (0..layout.ncols()).map(|col| ((0_i64, col as i64), Dir::Up));
    let last_col =
        (0..layout.nrows()).map(|row| ((row as i64, (layout.ncols() - 1) as i64), Dir::Right));
    let last_row =
        (0..layout.ncols()).map(|col| (((layout.nrows() - 1) as i64, col as i64), Dir::Down));

    let starting_positions = first_col.chain(first_row).chain(last_col).chain(last_row);

    let max_energized_tiles = starting_positions
        .par_bridge()
        .map(|(start_pos, start_incoming_dir)| {
            energize_tiles(&layout, start_pos, start_incoming_dir)
        })
        .max()
        .unwrap_or(0);

    Ok(max_energized_tiles)
}

#[derive(Default, Clone)]
struct Beam {
    out_dirs: HashSet<Dir>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

fn invert(dir: Dir) -> Dir {
    match dir {
        Dir::Left => Dir::Right,
        Dir::Right => Dir::Left,
        Dir::Up => Dir::Down,
        Dir::Down => Dir::Up,
    }
}

fn energize_tiles(
    layout: &Array2<char>,
    start_pos: (i64, i64),
    start_incoming_direction: Dir,
) -> u64 {
    let mut beams = Array2::from_elem((layout.nrows(), layout.ncols()), Beam::default());
    let mut visited: HashSet<((i64, i64), Dir)> = HashSet::default();

    let mut to_visit = VecDeque::new();
    to_visit.push_back((start_pos, start_incoming_direction));

    while let Some(((row, col), incoming_dir)) = to_visit.pop_front() {
        if visited.contains(&((row, col), incoming_dir)) {
            continue;
        }

        visited.insert(((row, col), incoming_dir));

        let outgoing_beams = match (layout[(row, col).into_index()], incoming_dir) {
            ('.', Dir::Left) => vec![((row, col + 1), Dir::Right)],
            ('.', Dir::Right) => vec![((row, col - 1), Dir::Left)],
            ('.', Dir::Up) => vec![((row + 1, col), Dir::Down)],
            ('.', Dir::Down) => vec![((row - 1, col), Dir::Up)],

            ('/', Dir::Left) => vec![((row - 1, col), Dir::Up)],
            ('/', Dir::Right) => vec![((row + 1, col), Dir::Down)],
            ('/', Dir::Up) => vec![((row, col - 1), Dir::Left)],
            ('/', Dir::Down) => vec![((row, col + 1), Dir::Right)],

            ('\\', Dir::Left) => vec![((row + 1, col), Dir::Down)],
            ('\\', Dir::Right) => vec![((row - 1, col), Dir::Up)],
            ('\\', Dir::Up) => vec![((row, col + 1), Dir::Right)],
            ('\\', Dir::Down) => vec![((row, col - 1), Dir::Left)],

            // Pointy ends of splitters
            ('|', Dir::Up) => vec![((row + 1, col), Dir::Down)],
            ('|', Dir::Down) => vec![((row - 1, col), Dir::Up)],
            ('-', Dir::Left) => vec![((row, col + 1), Dir::Right)],
            ('-', Dir::Right) => vec![((row, col - 1), Dir::Left)],

            // flat side of splitters
            ('|', Dir::Left) => vec![((row - 1, col), Dir::Up), ((row + 1, col), Dir::Down)],
            ('|', Dir::Right) => vec![((row - 1, col), Dir::Up), ((row + 1, col), Dir::Down)],
            ('-', Dir::Up) => vec![((row, col - 1), Dir::Left), ((row, col + 1), Dir::Right)],
            ('-', Dir::Down) => vec![((row, col - 1), Dir::Left), ((row, col + 1), Dir::Right)],

            (ch, incoming_dir) => panic!("unhandled case: {ch:?} {incoming_dir:?}"),
        };

        for ((next_row, next_col), outgoing_dir) in outgoing_beams {
            beams[(row, col).into_index()].out_dirs.insert(outgoing_dir);

            if next_row >= 0
                && next_row < layout.nrows() as i64
                && next_col >= 0
                && next_col < layout.ncols() as i64
            {
                to_visit.push_back(((next_row, next_col), invert(outgoing_dir)));
            }
        }
    }

    let energized_tiles = beams.iter().filter(|b| !b.out_dirs.is_empty()).count() as u64;

    energized_tiles
}
