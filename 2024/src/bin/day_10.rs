use aoc2024::*;

aoc_main!(
    day: 10,
    test_input: r#"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#,
    task_1: task_1,
    expected_1: 36,
    task_2: task_2,
    expected_2: 81,
);

fn task_1(input: &str) -> Result<usize> {
    let grid: Mat = Mat::from_digits(input)?;

    let trailheads = grid
        .indexed_iter()
        .filter(|(_idx, height)| *height == 0)
        .map(|(idx, _)| idx);

    let total_score = trailheads
        .map(|trailhead| reachable_peaks(trailhead, &grid).iter().unique().count())
        .sum();

    Ok(total_score)
}

fn task_2(input: &str) -> Result<usize> {
    let grid: Mat = Mat::from_digits(input)?;

    let trailheads = grid
        .indexed_iter()
        .filter(|(_idx, height)| *height == 0)
        .map(|(idx, _)| idx);

    let total_ratings = trailheads
        .map(|trailhead| reachable_peaks(trailhead, &grid).len())
        .sum();

    Ok(total_ratings)
}

fn reachable_peaks(trailhead: (i64, i64), grid: &Mat) -> Vec<(i64, i64)> {
    let mut to_visit = vec![(trailhead, 0)];
    let mut peaks = vec![];

    while let Some((pos, height)) = to_visit.pop() {
        if height == 9 {
            peaks.push(pos);
            continue;
        }

        let reachable_neighbors = grid
            .indexed_von_neumann_neighborhood(pos)
            .filter(|&(_pos, neighbor_height)| neighbor_height - height == 1);

        to_visit.extend(reachable_neighbors);
    }

    peaks
}
