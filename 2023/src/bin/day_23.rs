use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    iter::once,
};

use aoc2023::*;

aoc_main!(
    day: 23,
    test_input: r#"
    #.#####################
    #.......#########...###
    #######.#########.#.###
    ###.....#.>.>.###.#.###
    ###v#####.#v#.###.#.###
    ###.>...#.#.#.....#...#
    ###v###.#.#.#########.#
    ###...#.#.#.......#...#
    #####.#.#.#######.#.###
    #.....#.#.#.......#...#
    #.#####.#.#.#########v#
    #.#...#...#...###...>.#
    #.#.#v#######v###.###v#
    #...#.>.#...>.>.#.###.#
    #####v#.#.###v#.#.###.#
    #.....#...#...#.#.#...#
    #.#########.###.#.#.###
    #...###...#...#...#.###
    ###.###.#.###v#####v###
    #...#...#.#.>.>.#.>.###
    #.###.###.#.###.#.#v###
    #.....###...###...#...#
    #####################.#"#,
    task_1: task_1,
    expected_1: 94,
    task_2: task_2,
    expected_2: 154,
);

fn task_1(input: &str) -> Result<u64> {
    let map = Array2::from_2d_text(input)?;

    let start = map
        .row(0)
        .indexed_iter()
        .find(|&(_pos, el)| *el == '.')
        .map(|(col, _)| (0, col))
        .unwrap();

    let end = map
        .row(map.nrows() - 1)
        .indexed_iter()
        .find(|&(_pos, el)| *el == '.')
        .map(|(col, _)| (map.nrows() - 1, col))
        .unwrap();

    let dist = find_longest_hike(&map, start, end).ok_or(eyre!("no solution"));

    dist
}

fn task_2(input: &str) -> Result<u64> {
    let map = Array2::from_2d_text(input)?;

    let start = map
        .row(0)
        .indexed_iter()
        .find(|&(_pos, el)| *el == '.')
        .map(|(col, _)| (0, col))
        .unwrap();

    let end = map
        .row(map.nrows() - 1)
        .indexed_iter()
        .find(|&(_pos, el)| *el == '.')
        .map(|(col, _)| (map.nrows() - 1, col))
        .unwrap();

    let dist = find_longest_hike2(&map, start, end).ok_or(eyre!("no solution"));

    dist
}

#[derive(Debug, Clone)]
struct Node {
    pos: (usize, usize),
    visited: HashSet<(usize, usize)>,
    slope: Option<char>,
}

fn find_longest_hike(
    map: &Array2<char>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<u64> {
    let mut to_visit = VecDeque::new();
    to_visit.push_back(Node {
        pos: start,
        visited: HashSet::from_iter(once(start)),
        slope: None,
    });

    let mut path_lengths = vec![];

    while let Some(current) = to_visit.pop_front() {
        if current.pos == end {
            // print_visited(map, &current.visited);

            path_lengths.push(current.visited.len() as u64);
            continue;
        }

        for neighbor in neighbors(map, &current) {
            to_visit.push_back(neighbor);
        }
    }

    path_lengths.into_iter().max().map(|n| n - 1)
}

fn print_visited(map: &Array2<char>, visited: &HashSet<(usize, usize)>) {
    for row in 0..map.nrows() {
        for col in 0..map.ncols() {
            if visited.contains(&(row, col)) {
                print!("O");
            } else {
                print!("{}", map[(row, col)]);
            }
        }
        println!();
    }
    println!();
}

fn neighbors(map: &Array2<char>, node: &Node) -> Vec<Node> {
    if let Some(slope) = node.slope {
        let d = match slope {
            '>' => (0_i64, 1_i64),
            '<' => (0, -1),
            'v' => (1, 0),
            '^' => (-1, 0),
            _ => panic!("unrecognized slope: {:?}", slope),
        };

        let pos = (
            (node.pos.0 as i64 + d.0) as usize,
            (node.pos.1 as i64 + d.1) as usize,
        );

        if node.visited.contains(&pos) {
            return vec![];
        }

        let mut visited = node.visited.clone();
        visited.insert(pos);

        return vec![Node {
            pos,
            visited,
            slope: None,
        }];
    }

    map.von_neumann_neighborhood(&node.pos)
        .filter(|&pos| {
            let tile = map[pos];
            let is_slope = tile == '>' || tile == '<' || tile == '^' || tile == 'v';
            (is_slope || tile == '.') && !node.visited.contains(&pos)
        })
        .map(|pos| {
            let mut visited = node.visited.clone();
            visited.insert(pos);

            let tile = map[pos];
            let is_slope = tile == '>' || tile == '<' || tile == '^' || tile == 'v';

            let slope = if is_slope { Some(tile) } else { None };

            Node {
                pos,
                visited,
                slope,
            }
        })
        .collect()
}

fn find_longest_hike2(
    map: &Array2<char>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<u64> {
    let mut to_visit = VecDeque::new();
    to_visit.push_back(Node {
        pos: start,
        visited: HashSet::from_iter(once(start)),
        slope: None,
    });

    let mut path_lengths = vec![];

    while let Some(current) = to_visit.pop_front() {
        if current.pos == end {
            // print_visited(map, &current.visited);

            path_lengths.push(current.visited.len() as u64);
            continue;
        }

        for neighbor in neighbors2(map, &current) {
            to_visit.push_back(neighbor);
        }
    }

    path_lengths.into_iter().max().map(|n| n - 1)
}

fn neighbors2(map: &Array2<char>, node: &Node) -> Vec<Node> {
    map.von_neumann_neighborhood(&node.pos)
        .filter(|&pos| {
            let tile = map[pos];
            let is_slope = tile == '>' || tile == '<' || tile == '^' || tile == 'v';
            (is_slope || tile == '.') && !node.visited.contains(&pos)
        })
        .map(|pos| {
            let mut visited = node.visited.clone();
            visited.insert(pos);

            Node {
                pos,
                visited,
                slope: None,
            }
        })
        .collect()
}
