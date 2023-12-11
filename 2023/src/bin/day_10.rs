use aoc2023::*;

aoc_main!(
    day: 10,
    test_input: r#"
    ..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ..."#,
    test_input_2: r#"
    FF7FSF7F7F7F7F7F---7
    L|LJ||||||||||||F--J
    FL-7LJLJ||||||LJL-77
    F--JF--7||LJLJ7F7FJ-
    L---JF-JLJ.||-FJLJJ7
    |F|F-JF---7F7-L7L|7|
    |FFJF7L7F-JF7|JL---7
    7-L-JL7||F7|L7F-7F7|
    L.L7LFJ|||||FJL7||LJ
    L7JLJL-JLJLJL--JLJ.L
    "#,
    task_1: task_1,
    expected_1: 8,
    task_2: task_2,
    expected_2: 10,
);

fn task_1(input: &str) -> Result<u64> {
    let tiles = Array2::from_2d_text(input)?;

    let (starting_pos, _) = tiles
        .indexed_iter()
        .find(|&(_, e)| *e == 'S')
        .ok_or_parse_error()?;

    let connected_to_start_pos = tiles
        .von_neumann_neighborhood(&starting_pos)
        .map(|neighbor| (neighbor, flip(dir_of(starting_pos, neighbor))))
        .filter(|&(pos, _)| is_connected(&tiles, starting_pos, pos));

    let mut queue: VecDeque<(Dir, (usize, usize), u64)> = connected_to_start_pos
        .map(|(pos, incoming_dir)| (incoming_dir, pos, 1))
        .collect();

    let mut visited: Array2<bool> = Array2::from_elem((tiles.nrows(), tiles.ncols()), false);
    let mut max_dist = 0;

    while let Some((incoming_dir, pos, dist)) = queue.pop_front() {
        if visited[pos] {
            break;
        }

        let outgoing_dir = outgoing_dir(&tiles, pos, incoming_dir);

        let next_pos = apply(pos, outgoing_dir);
        let incoming_dir = flip(outgoing_dir);
        queue.push_back((incoming_dir, next_pos, dist + 1));

        max_dist = std::cmp::max(dist, max_dist);
        visited[pos] = true;
    }

    Ok(max_dist)
}

fn task_2(input: &str) -> Result<u64> {
    let mut tiles = Array2::from_2d_text(input)?;

    let (starting_pos, _) = tiles
        .indexed_iter()
        .find(|&(_, e)| *e == 'S')
        .ok_or_parse_error()?;

    let connected_to_start_pos: Vec<((usize, usize), Dir)> = tiles
        .von_neumann_neighborhood(&starting_pos)
        .map(|neighbor| (neighbor, dir_of(starting_pos, neighbor)))
        .filter(|&(pos, _)| is_connected(&tiles, starting_pos, pos))
        .collect();

    let dirs_connected_to_start_pos: Vec<Dir> = connected_to_start_pos
        .iter()
        .map(|&(_, d)| d)
        .sorted()
        .collect();
    let replace_s_with = match (
        dirs_connected_to_start_pos[0],
        dirs_connected_to_start_pos[1],
    ) {
        (Dir::North, Dir::South) => '|',
        (Dir::East, Dir::West) => '-',
        (Dir::East, Dir::South) => 'F',
        (Dir::North, Dir::East) => 'L',
        (Dir::South, Dir::West) => '7',
        (Dir::North, Dir::West) => 'J',
        _ => panic!(
            "unable to determine start pos from {:?}",
            dirs_connected_to_start_pos
        ),
    };

    tiles[starting_pos] = replace_s_with;

    let mut queue: VecDeque<(Dir, (usize, usize), u64)> = connected_to_start_pos
        .iter()
        .map(|&(pos, dir_from_start)| (flip(dir_from_start), pos, 1))
        .collect();

    let mut pipes: Array2<char> = Array2::from_elem((tiles.nrows(), tiles.ncols()), '.');
    pipes[starting_pos] = replace_s_with;

    while let Some((incoming_dir, pos, dist)) = queue.pop_front() {
        if pipes[pos] != '.' {
            break;
        }

        let outgoing_dir = outgoing_dir(&tiles, pos, incoming_dir);
        let next_pos = apply(pos, outgoing_dir);
        let incoming_dir = flip(outgoing_dir);
        queue.push_back((incoming_dir, next_pos, dist + 1));

        pipes[pos] = tiles[pos];
    }

    let mut to_visit = VecDeque::new();
    to_visit.push_back((0usize, 0usize));

    let mut visited: Array2<bool> =
        Array2::from_elem((tiles.nrows() + 1, tiles.ncols() + 1), false);

    while let Some(current) = to_visit.pop_front() {
        if visited[current] {
            continue;
        }

        visited[current] = true;
        let neighbors = neighbors(&tiles, &current);

        if current.0 < tiles.nrows() && current.1 < tiles.ncols() && pipes[current] == '.' {
            pipes[current] = 'O';
        }

        for neighbor in neighbors {
            if neighbor.0 == tiles.nrows() || neighbor.1 == tiles.ncols() {
                to_visit.push_back(neighbor);
                continue;
            }

            let move_direction = dir_of(current, neighbor);

            let neighbor_pipe = pipes[neighbor];

            let can_move = if move_direction == Dir::South {
                current.0 < (pipes.nrows() - 1)
                    && pipes[current] != '-'
                    && pipes[current] != '7'
                    && pipes[current] != 'J'
            } else if move_direction == Dir::North {
                neighbor_pipe != '-' && neighbor_pipe != '7' && neighbor_pipe != 'J'
            } else if move_direction == Dir::East {
                current.1 < (pipes.ncols() - 1)
                    && pipes[current] != '|'
                    && pipes[current] != 'J'
                    && pipes[current] != 'L'
            } else {
                // west
                neighbor_pipe != '|' && neighbor_pipe != 'J' && neighbor_pipe != 'L'
            };

            if can_move {
                to_visit.push_back(neighbor);
            }
        }
    }

    Ok(pipes.iter().filter(|&e| *e == '.').count() as u64)
}

fn neighbors(tiles: &Array2<char>, pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];

    if pos.0 > 0 {
        neighbors.push((pos.0 - 1, pos.1));
    }

    if pos.0 < tiles.nrows() {
        neighbors.push((pos.0 + 1, pos.1));
    }

    if pos.1 > 0 {
        neighbors.push((pos.0, pos.1 - 1));
    }

    if pos.1 < tiles.ncols() {
        neighbors.push((pos.0, pos.1 + 1));
    }

    neighbors
}

fn is_connected(tiles: &Array2<char>, start: (usize, usize), b: (usize, usize)) -> bool {
    let d = (b.0 as i64 - start.0 as i64, b.1 as i64 - start.1 as i64);
    let tile = tiles[b];

    let north = d == (-1, 0);
    let south = d == (1, 0);
    let east = d == (0, 1);
    let west = d == (0, -1);

    if north {
        return tile == '|' || tile == '7' || tile == 'F';
    }

    if south {
        return tile == '|' || tile == 'L' || tile == 'J';
    }

    if east {
        return tile == '-' || tile == 'J' || tile == '7';
    }

    if west {
        return tile == '-' || tile == 'L' || tile == 'F';
    }

    panic!(
        "unable to determine whether {:?} and {:?} are connected",
        start, b
    );
}

fn dir_of(a: (usize, usize), b: (usize, usize)) -> Dir {
    let d = (b.0 as i64 - a.0 as i64, b.1 as i64 - a.1 as i64);
    match d {
        (-1, 0) => Dir::North,
        (1, 0) => Dir::South,
        (0, 1) => Dir::East,
        (0, -1) => Dir::West,
        _ => panic!("invalid direction: {:?}", d),
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
enum Dir {
    North,
    East,
    South,
    West,
}

fn outgoing_dir(tiles: &Array2<char>, pos: (usize, usize), incoming_dir: Dir) -> Dir {
    let tile = tiles[pos];

    match (tile, incoming_dir) {
        ('|', Dir::North) => Dir::South,
        ('|', Dir::South) => Dir::North,
        ('-', Dir::West) => Dir::East,
        ('-', Dir::East) => Dir::West,
        ('L', Dir::North) => Dir::East,
        ('L', Dir::East) => Dir::North,
        ('J', Dir::North) => Dir::West,
        ('J', Dir::West) => Dir::North,
        ('7', Dir::South) => Dir::West,
        ('7', Dir::West) => Dir::South,
        ('F', Dir::South) => Dir::East,
        ('F', Dir::East) => Dir::South,
        _ => panic!("cannot handle {:?} and {:?}", tile, incoming_dir),
    }
}

fn apply(pos: (usize, usize), dir: Dir) -> (usize, usize) {
    match dir {
        Dir::North => (pos.0 - 1, pos.1),
        Dir::South => (pos.0 + 1, pos.1),
        Dir::West => (pos.0, pos.1 - 1),
        Dir::East => (pos.0, pos.1 + 1),
    }
}

fn flip(dir: Dir) -> Dir {
    match dir {
        Dir::North => Dir::South,
        Dir::South => Dir::North,
        Dir::West => Dir::East,
        Dir::East => Dir::West,
    }
}
