use anyhow::Context;
use aoc2022::*;

fn parse(raw_input: &str) -> Result<Array2<u8>> {
    let cols = raw_input
        .lines()
        .next()
        .map(|l| l.len())
        .ok_or_else(|| anyhow!("Empty input"))?;

    let rows = raw_input.lines().count();

    let data: Vec<u8> = raw_input
        .replace('\n', "")
        .chars()
        .map(|c| c as u8)
        .collect();

    Array2::from_shape_vec((rows, cols), data).with_context(|| "Unable to build Array2 from input")
}

fn task_1(height_map: &Array2<u8>) -> Result<usize> {
    let start: (usize, usize) = height_map
        .indexed_iter()
        .find(|&(_, v)| *v == b'S')
        .unwrap()
        .0;

    let end: (usize, usize) = height_map
        .indexed_iter()
        .find(|&(_, v)| *v == b'E')
        .unwrap()
        .0;

    let start = [start];
    shortest_path_from_multiple(height_map, &start, end).ok_or_else(|| anyhow!("No solution"))
}

fn task_2(height_map: &Array2<u8>) -> Result<usize> {
    let start: Vec<(usize, usize)> = height_map
        .indexed_iter()
        .filter(|&(_, v)| *v == b'S' || *v == b'a')
        .map(|(pos, _)| pos)
        .collect();

    let end: (usize, usize) = height_map
        .indexed_iter()
        .find(|&(_, v)| *v == b'E')
        .unwrap()
        .0;

    shortest_path_from_multiple(height_map, &start, end).ok_or_else(|| anyhow!("No solution"))
}

fn shortest_path_from_multiple(
    height_map: &Array2<u8>,
    from: &[(usize, usize)],
    to: (usize, usize),
) -> Option<usize> {
    let mut to_visit: VecDeque<((usize, usize), usize)> =
        VecDeque::from_iter(from.iter().map(|start| (*start, 0)));

    let mut visited: HashSet<(usize, usize)> = HashSet::from_iter(from.iter().copied());

    while let Some((pos, steps_taken)) = to_visit.pop_front() {
        if pos == to {
            return Some(steps_taken);
        }

        let neighbors = height_map.von_neumann_neighborhood(&pos);

        for neighbor in neighbors {
            if can_reach(height_map[pos], height_map[neighbor]) && !visited.contains(&neighbor) {
                to_visit.push_back((neighbor, steps_taken + 1));
                visited.insert(neighbor);
            }
        }
    }

    None
}

fn can_reach(from: u8, to: u8) -> bool {
    let from = if from == b'S' { b'a' } else { from };
    let to = if to == b'E' { b'z' } else { to };

    from >= to || (from + 1 == to)
}

aoc_main!(
    day: 12,
    test_input:
    r#"
    Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 31,
    task_2: task_2,
    expected_2: 29,
);
