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

fn task_1(input: &str) -> Result<i64> {
    let grid: Mat = Mat::from_single_digits(input)?;

    let trailheads = grid
        .indexed_iter()
        .filter(|(_idx, height)| *height == 0)
        .map(|(idx, _)| idx);

    let total_score = trailheads.map(|idx| calculate_score(idx, &grid)).sum();

    Ok(total_score)
}

fn task_2(input: &str) -> Result<i64> {
    let grid: Mat = Mat::from_single_digits(input)?;

    let peaks = grid
        .indexed_iter()
        .filter(|(_idx, height)| *height == 9)
        .map(|(idx, _)| idx);

    let rating_maps = peaks.map(|peak| build_rating_map(&grid, peak));

    let mut total_rating_map = Mat::zeros(grid.shape());
    for rating_map in rating_maps {
        total_rating_map += rating_map;
    }

    let trailheads = grid
        .indexed_iter()
        .filter(|(_idx, height)| *height == 0)
        .map(|(idx, _)| idx);

    let total_ratings = trailheads.map(|idx| total_rating_map[idx]).sum();

    Ok(total_ratings)
}

fn calculate_score(trailhead: (i64, i64), grid: &Mat) -> i64 {
    let mut q: VecDeque<(i64, i64)> = VecDeque::new();

    q.push_back(trailhead);

    let mut reached: HashSet<(i64, i64)> = HashSet::default();
    let mut seen: HashSet<(i64, i64)> = HashSet::default();

    while let Some(pos) = q.pop_front() {
        if seen.contains(&pos) {
            continue;
        }

        seen.insert(pos);
        if grid[pos] == 9 {
            reached.insert(pos);
        }

        let height = grid[pos];
        let reachable_neighbors = grid
            .indexed_von_neumann_neighborhood(pos)
            .filter(|&(_pos, neighbor_height)| neighbor_height - height == 1);

        for (neighbor, _) in reachable_neighbors {
            if !seen.contains(&neighbor) {
                q.push_back(neighbor);
            }
        }
    }

    reached.len() as i64
}

fn build_rating_map(grid: &Mat, peak: (i64, i64)) -> Mat {
    let mut rating_map = Mat::zeros(grid.shape());

    let mut working_queue: VecDeque<((i64, i64), HashSet<(i64, i64)>)> = VecDeque::default();
    let mut seen = HashSet::default();
    seen.insert(peak);
    working_queue.push_back((peak, seen));

    while let Some((pos, seen)) = working_queue.pop_front() {
        rating_map[pos] += 1;

        let height = grid[pos];
        let reachable_neighbors = grid
            .indexed_von_neumann_neighborhood(pos)
            .filter(|&(_pos, neighbor_height)| neighbor_height - height == -1);

        for (neighbor_pos, _) in reachable_neighbors {
            if !seen.contains(&neighbor_pos) {
                let mut new_seen = seen.clone();
                new_seen.insert(neighbor_pos);
                working_queue.push_back((neighbor_pos, new_seen));
            }
        }
    }

    rating_map
}
