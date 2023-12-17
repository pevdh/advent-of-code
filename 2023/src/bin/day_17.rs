use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use aoc2023::*;

aoc_main!(
    day: 17,
    test_input: r#"
    2413432311323
    3215453535623
    3255245654254
    3446585845452
    4546657867536
    1438598798454
    4457876987766
    3637877979653
    4654967986887
    4564679986453
    1224686865563
    2546548887735
    4322674655533"#,
    task_1: task_1,
    expected_1: 102,
    task_2: task_2,
    expected_2: 94,
);

fn task_1(input: &str) -> Result<u64> {
    let map: Array2<u64> = Array2::from_2d_text_digits(input)?;

    compute_least_heat_loss(&map, (0, 0), (map.nrows() - 1, map.ncols() - 1))
        .ok_or_else(|| eyre!("no solution"))
}

fn task_2(input: &str) -> Result<u64> {
    let map: Array2<u64> = Array2::from_2d_text_digits(input)?;

    compute_least_heat_loss_ultra(&map, (0, 0), (map.nrows() - 1, map.ncols() - 1))
        .ok_or_else(|| eyre!("no solution"))
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    heat_loss: u64,
    pos: (usize, usize),
    dir: Dir,
    steps_into_dir: u32,
}

type Dir = (i64, i64);
const UP: Dir = (-1, 0);
const RIGHT: Dir = (0, 1);
const DOWN: Dir = (1, 0);
const LEFT: Dir = (0, -1);

fn invert(dir: Dir) -> Dir {
    (-dir.0, -dir.1)
}

pub trait PosTools {
    fn add(self, dir: (i64, i64)) -> Option<(usize, usize)>;
}

impl PosTools for (usize, usize) {
    fn add(self, dir: (i64, i64)) -> Option<(usize, usize)> {
        let row = self.0 as i64 + dir.0;
        let col = self.1 as i64 + dir.1;

        if row < 0 || col < 0 {
            return None;
        }

        Some((row as usize, col as usize))
    }
}

fn compute_least_heat_loss(
    map: &Array2<u64>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<u64> {
    let mut min_distances = Array2::from_elem((map.nrows(), map.ncols()), u64::MAX);
    min_distances[start] = 0;

    let mut to_visit = BinaryHeap::new();
    to_visit.push(Reverse(Node {
        heat_loss: 0,
        pos: start,
        dir: RIGHT,
        steps_into_dir: 0,
    }));

    let mut visited: HashSet<((usize, usize), Dir, u32)> = HashSet::default();

    while let Some(Reverse(current)) = to_visit.pop() {
        if visited.contains(&(current.pos, current.dir, current.steps_into_dir)) {
            continue;
        }

        if current.pos == end {
            return Some(min_distances[current.pos]);
        }

        for (dist, neighbor) in neighbors(map, &current) {
            let tentative_dist = dist + current.heat_loss;
            if tentative_dist < min_distances[neighbor.pos] {
                min_distances[neighbor.pos] = tentative_dist;
            }

            to_visit.push(Reverse(neighbor));
        }

        visited.insert((current.pos, current.dir, current.steps_into_dir));
    }

    None
}

fn neighbors(map: &Array2<u64>, node: &Node) -> Vec<(u64, Node)> {
    let mut neighbors = vec![];

    for dir in [UP, RIGHT, DOWN, LEFT] {
        if dir == invert(node.dir) {
            continue; // cannot move backwards
        }

        if dir == node.dir {
            let num_steps_left = 3 - node.steps_into_dir;
            let mut heat_loss = 0_u64;
            let mut cur_pos = node.pos;

            for i in 0..num_steps_left {
                let neighbor = match cur_pos.add(dir) {
                    Some(neighbor) => neighbor,
                    None => continue,
                };

                if neighbor.0 >= map.nrows() || neighbor.1 >= map.ncols() {
                    continue;
                }

                heat_loss += map[neighbor];

                let neighbor_node = Node {
                    pos: neighbor,
                    dir,
                    steps_into_dir: node.steps_into_dir + i + 1,
                    heat_loss: node.heat_loss + heat_loss,
                };
                cur_pos = neighbor_node.pos;

                neighbors.push((heat_loss, neighbor_node));
            }
        } else {
            let neighbor = match node.pos.add(dir) {
                Some(neighbor) => neighbor,
                None => continue,
            };

            if neighbor.0 >= map.nrows() || neighbor.1 >= map.ncols() {
                continue;
            }

            let neighbor_node = Node {
                pos: neighbor,
                dir,
                steps_into_dir: 1,
                heat_loss: node.heat_loss + map[neighbor],
            };

            neighbors.push((map[neighbor], neighbor_node));
        }
    }

    neighbors
}

fn compute_least_heat_loss_ultra(
    map: &Array2<u64>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<u64> {
    let mut min_distances = Array2::from_elem((map.nrows(), map.ncols()), u64::MAX);
    min_distances[start] = 0;

    let mut to_visit = BinaryHeap::new();
    to_visit.push(Reverse(Node {
        heat_loss: 0,
        pos: start,
        dir: RIGHT,
        steps_into_dir: 0,
    }));

    let mut visited: HashSet<((usize, usize), Dir, u32)> = HashSet::default();

    while let Some(Reverse(current)) = to_visit.pop() {
        if visited.contains(&(current.pos, current.dir, current.steps_into_dir)) {
            continue;
        }

        if current.pos == end {
            return Some(min_distances[current.pos]);
        }

        for (dist, neighbor) in neighbors_ultra(map, &current) {
            let tentative_dist = dist + current.heat_loss;
            if tentative_dist < min_distances[neighbor.pos] {
                min_distances[neighbor.pos] = tentative_dist;
            }

            to_visit.push(Reverse(neighbor));
        }

        visited.insert((current.pos, current.dir, current.steps_into_dir));
    }

    None
}

fn neighbors_ultra(map: &Array2<u64>, node: &Node) -> Vec<(u64, Node)> {
    let mut neighbors = vec![];

    let mut dirs = vec![UP, RIGHT, DOWN, LEFT];

    if node.steps_into_dir < 4 {
        dirs = vec![node.dir];
    }

    for dir in dirs {
        if dir == invert(node.dir) {
            continue; // cannot move backwards
        }

        if dir == node.dir {
            let num_steps_left = 10 - node.steps_into_dir;
            let mut heat_loss = 0_u64;
            let mut cur_pos = node.pos;

            for i in 0..num_steps_left {
                let neighbor = match cur_pos.add(dir) {
                    Some(neighbor) => neighbor,
                    None => continue,
                };

                if neighbor.0 >= map.nrows() || neighbor.1 >= map.ncols() {
                    continue;
                }

                heat_loss += map[neighbor];

                let neighbor_node = Node {
                    pos: neighbor,
                    dir,
                    steps_into_dir: node.steps_into_dir + i + 1,
                    heat_loss: node.heat_loss + heat_loss,
                };
                cur_pos = neighbor_node.pos;

                neighbors.push((heat_loss, neighbor_node));
            }
        } else {
            let neighbor = match node.pos.add(dir) {
                Some(neighbor) => neighbor,
                None => continue,
            };

            if neighbor.0 >= map.nrows() || neighbor.1 >= map.ncols() {
                continue;
            }

            let neighbor_node = Node {
                pos: neighbor,
                dir,
                steps_into_dir: 1,
                heat_loss: node.heat_loss + map[neighbor],
            };

            neighbors.push((map[neighbor], neighbor_node));
        }
    }

    neighbors
}
