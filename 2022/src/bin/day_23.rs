use aoc2022::*;

aoc_main!(
    day: 23,
    test_input:
    r#"
    ....#..
    ..###.#
    #...#.#
    .#...##
    #.###..
    ##.#.##
    .#..#..
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 110,
    task_2: task_2,
    expected_2: 20,
);

fn parse(raw_input: &str) -> Result<HashSet<Coord>> {
    Ok(raw_input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as i64, y as i64))
        })
        .collect())
}

fn task_1(elves: &HashSet<Coord>) -> Result<i64> {
    let mut elves = elves.clone();

    for round in 1..=10 {
        simulate_round(&mut elves, round);
    }

    let min_x = elves.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = elves.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = elves.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = elves.iter().map(|(_, y)| *y).max().unwrap();

    Ok(((max_x - min_x + 1) * (max_y - min_y + 1)) - (elves.len() as i64))
}

fn task_2(elves: &HashSet<Coord>) -> Result<usize> {
    let mut elves = elves.clone();

    for round in 1.. {
        let num_moved = simulate_round(&mut elves, round);

        if num_moved == 0 {
            return Ok(round);
        }
    }

    Err(anyhow!("No solution"))
}

fn simulate_round(elves: &mut HashSet<Coord>, round: usize) -> usize {
    let mut proposals = vec![];
    let mut proposal_count = HashMap::default();
    let first_direction_idx = (round - 1) % DIRECTIONS.len();

    // First half of round
    for elf in elves.iter() {
        let adjacent_elves = [N, NE, E, SE, S, SW, W, NW]
            .iter()
            .map(|d| (elf.0 + d.0, elf.1 + d.1))
            .filter(|c| elves.contains(c))
            .count();

        if adjacent_elves == 0 {
            continue;
        }

        let directions = (first_direction_idx..)
            .map(|idx| DIRECTIONS[idx % DIRECTIONS.len()])
            .take(DIRECTIONS.len());

        for (move_to, check1, check2) in directions {
            let proposed_elf_location = (elf.0 + move_to.0, elf.1 + move_to.1);

            if !elves.contains(&proposed_elf_location)
                && !elves.contains(&(elf.0 + check1.0, elf.1 + check1.1))
                && !elves.contains(&(elf.0 + check2.0, elf.1 + check2.1))
            {
                *proposal_count.entry(proposed_elf_location).or_insert(0) += 1;
                proposals.push((*elf, proposed_elf_location));

                break;
            }
        }
    }

    // Second half of round
    let mut num_moved = 0;

    proposals
        .iter()
        .filter(|(_, new_elf_location)| proposal_count.get(new_elf_location) == Some(&1))
        .for_each(|(old_elf_location, new_elf_location)| {
            elves.remove(old_elf_location);
            elves.insert(*new_elf_location);
            num_moved += 1;
        });

    num_moved
}

type Coord = (i64, i64);
const N: Coord = (0, -1);
const NE: Coord = (1, -1);
const E: Coord = (1, 0);
const SE: Coord = (1, 1);
const S: Coord = (0, 1);
const SW: Coord = (-1, 1);
const W: Coord = (-1, 0);
const NW: Coord = (-1, -1);

const DIRECTIONS: [(Coord, Coord, Coord); 4] = [(N, NE, NW), (S, SE, SW), (W, NW, SW), (E, NE, SE)];
