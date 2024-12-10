use aoc2024::*;

aoc_main!(
    day: 8,
    test_input: r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#,
    task_1: task_1,
    expected_1: 14,
    task_2: task_2,
    expected_2: 34,
);

fn task_1(input: &str) -> Result<usize> {
    let antenna_map = CharGrid::from_text(input)?;

    let mut frequencies: HashMap<char, Vec<(i64, i64)>> = HashMap::default();

    for (row, col) in antenna_map.indices() {
        if antenna_map[(row, col)] != '.' {
            frequencies
                .entry(antenna_map[(row, col)])
                .or_default()
                .push((row, col))
        }
    }

    let mut antinode_locations: HashSet<(i64, i64)> = HashSet::default();

    let antenna_pairs = frequencies
        .iter()
        .flat_map(|(_freq, locations)| locations.iter().tuple_combinations());

    for (a, b) in antenna_pairs {
        let dist_a_to_b = (b.0 - a.0, b.1 - a.1);
        let first_antinode = (b.0 + dist_a_to_b.0, b.1 + dist_a_to_b.1);
        if antenna_map.in_bounds(first_antinode) {
            antinode_locations.insert(first_antinode);
        }

        let dist_b_to_a = (a.0 - b.0, a.1 - b.1);
        let second_antinode = (a.0 + dist_b_to_a.0, a.1 + dist_b_to_a.1);

        if antenna_map.in_bounds(second_antinode) {
            antinode_locations.insert(second_antinode);
        }
    }

    Ok(antinode_locations.len())
}

fn task_2(input: &str) -> Result<usize> {
    let antenna_map = CharGrid::from_text(input)?;

    let mut frequencies: HashMap<char, Vec<(i64, i64)>> = HashMap::default();

    for (row, col) in antenna_map.indices() {
        if antenna_map[(row, col)] != '.' {
            frequencies
                .entry(antenna_map[(row, col)])
                .or_default()
                .push((row, col))
        }
    }

    let antenna_pairs = frequencies
        .iter()
        .flat_map(|(_freq, locations)| locations.iter().tuple_combinations());

    let unique_antinode_locations = antenna_pairs
        .flat_map(|(a, b)| points_on_line(&antenna_map, *a, *b))
        .unique();

    Ok(unique_antinode_locations.count())
}

fn points_on_line(
    grid: &CharGrid,
    from: (i64, i64),
    to: (i64, i64),
) -> impl Iterator<Item = (i64, i64)> + '_ {
    let (dy, dx) = (to.0 - from.0, to.1 - from.1);
    let gcd = euclid_gcd(dy, dx);
    let (dy, dx) = (dy / gcd, dx / gcd);

    let mut k = 0;
    let a = std::iter::from_fn(move || {
        let (row, col) = (from.0 + dy * k, from.1 + dx * k);
        k += 1;

        Some((row, col))
    })
    .take_while(|&p| grid.in_bounds(p));

    let b = std::iter::from_fn(move || {
        let (row, col) = (from.0 + -dy * k, from.1 + -dx * k);
        k += 1;

        Some((row, col))
    })
    .take_while(|&p| grid.in_bounds(p));

    a.chain(b)
}

fn euclid_gcd(a: i64, b: i64) -> i64 {
    let mut r1 = a;
    let mut r2 = b;

    while r2 != 0 {
        let new_rem = r1.rem_euclid(r2);
        r1 = r2;
        r2 = new_rem;
    }

    r1
}
