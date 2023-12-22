use std::{
    cmp::{max, min},
    collections::HashSet,
};

use aoc2023::*;

aoc_main!(
    day: 22,
    test_input: r#"
    1,0,1~1,2,1
    0,0,2~2,0,2
    0,2,3~2,2,3
    0,0,4~0,2,4
    2,0,5~2,2,5
    0,1,6~2,1,6
    1,1,8~1,1,9"#,
    task_1: task_1,
    expected_1: 5,
    task_2: task_2,
    expected_2: 167409079868000,
);

type Coord = (usize, usize, usize);
type Brick = (Coord, Coord);

fn task_1(input: &str) -> Result<usize> {
    let parse_coord = |coord_str: &str| {
        let mut coord_str_parts = coord_str.split(',');

        let x = coord_str_parts.next().unwrap().parse::<usize>().unwrap();
        let y = coord_str_parts.next().unwrap().parse::<usize>().unwrap();
        let z = coord_str_parts.next().unwrap().parse::<usize>().unwrap();

        (x, y, z)
    };

    let bricks: Vec<Brick> = input
        .lines()
        .map(|line| {
            let (end_1, end_2) = line.split_once('~').unwrap();

            (parse_coord(end_1), parse_coord(end_2))
        })
        .sorted_by_key(|brick| min(brick.0 .2, brick.1 .2))
        .collect();

    let max_x = bricks
        .iter()
        .flat_map(|brick| [brick.0 .0, brick.1 .0])
        .max()
        .unwrap_or(0);

    let max_y = bricks
        .iter()
        .flat_map(|brick| [brick.0 .1, brick.1 .1])
        .max()
        .unwrap_or(0);

    let x_dim = max_x + 1;
    let y_dim = max_y + 1;

    let mut z_height_map: Array2<usize> = Array2::zeros((x_dim, y_dim));
    let mut z_bricks: Array2<Option<usize>> = Array2::from_elem((x_dim, y_dim), None);

    let mut safe_to_disintegrate: HashSet<usize> = (0..bricks.len()).collect();

    for (brick_idx, brick) in bricks.iter().enumerate() {
        let brick_xy = xy_coords(brick);

        let min_z = min(brick.0 .2, brick.1 .2);
        let max_z = max(brick.0 .2, brick.1 .2);

        let height = max_z - min_z + 1;

        let resting_z = brick_xy
            .iter()
            .map(|&(x, y)| z_height_map[(x, y)])
            .max()
            .unwrap_or(0);

        let supporting_blocks: Vec<usize> = brick_xy
            .iter()
            .filter_map(|&(x, y)| z_bricks[(x, y)])
            .unique()
            .filter(|&supporting_brick_idx| {
                let supporting_brick = bricks[supporting_brick_idx];
                let supporting_brick_xy = xy_coords(&supporting_brick);
                let support_z = supporting_brick_xy
                    .intersection(&brick_xy)
                    .map(|&(x, y)| z_height_map[(x, y)])
                    .max()
                    .unwrap_or(0_usize);

                support_z == resting_z
            })
            .collect();

        if supporting_blocks.len() == 1 {
            safe_to_disintegrate.remove(&supporting_blocks[0]);
        }

        for (x, y) in brick_xy {
            assert!(z_height_map[(x, y)] <= resting_z);

            z_height_map[(x, y)] = resting_z + height;
            z_bricks[(x, y)] = Some(brick_idx);
        }

        if true {
            println!("{:?} ~ {:?}", brick.0, brick.1);
            println!();

            for row in 0..z_height_map.nrows() {
                for col in 0..z_height_map.ncols() {
                    print!("{: >5?} ", z_height_map[(row, col)]);
                }
                println!()
            }
            println!();

            for row in 0..z_bricks.nrows() {
                for col in 0..z_bricks.ncols() {
                    if let Some(brick_idx) = z_bricks[(row, col)] {
                        print!("{: >5?} ", brick_idx);
                    } else {
                        print!("{: >5} ", "--");
                    }
                }
                println!()
            }
            println!();
        }
    }

    // println!("{:?}", safe_to_disintegrate);
    // panic!();

    Ok(safe_to_disintegrate.len())
}

fn xy_coords(brick: &Brick) -> HashSet<(usize, usize)> {
    let min_x = min(brick.0 .0, brick.1 .0);
    let min_y = min(brick.0 .1, brick.1 .1);

    let max_x = max(brick.0 .0, brick.1 .0);
    let max_y = max(brick.0 .1, brick.1 .1);

    let x_coords = min_x..=max_x;
    let y_coords = min_y..=max_y;

    x_coords.cartesian_product(y_coords).collect()
}

fn task_2(_input: &str) -> Result<u64> {
    Ok(0)
}
