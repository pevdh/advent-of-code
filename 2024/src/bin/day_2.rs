use aoc2024::*;

aoc_main!(
    day: 2,
    test_input: r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#,
    task_1: task_1,
    expected_1: 2,
    task_2: task_2,
    expected_2: 4,
);

fn task_1(input: &str) -> Result<usize> {
    let reports = input.lines()
            .map(|line| {
                    line
                        .split_whitespace()
                        .map(|l| l.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>()
            });

    let num_safe_reports = reports
            .filter(|report| is_safe(report.iter().copied()))
            .count();

    Ok(num_safe_reports)
}

fn task_2(input: &str) -> Result<usize> {
    let reports = input.lines()
        .map(|line| {
                line
                    .split_whitespace()
                    .map(|l| l.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
        });

    let num_safe_reports = reports
            .filter(|report| is_safe(report.iter().copied()) || is_safe_with_tolerance(&report))
            .count();

    Ok(num_safe_reports)
}

fn is_safe_with_tolerance(levels: &[i64]) -> bool {
    let is_safe = (0..levels.len())
        .map(|dropped_idx| {
            let modified = levels.iter()
                .enumerate()
                .filter(|&(idx, _)| idx != dropped_idx)
                .map(|(_, &level)| level);

            return is_safe(modified);
        })
        .any(|is_safe| is_safe);

    return is_safe;
}

fn is_safe<L>(levels: L) -> bool 
where L: Iterator<Item = i64>
{
    let differences: Vec<i64> = levels.tuple_windows()
        .map(|(a, b)| {
            b - a
        })
        .collect();

    let all_increasing = differences.iter()
        .all(|&d| d > 0);

    let all_decreasing = differences.iter()
        .all(|&d| d < 0);

    let small_differences = differences.iter()
        .map(|&d| d.abs())
        .all(|d| d >= 1 && d <= 3);

    let is_safe = (all_increasing || all_decreasing) && small_differences;

    return is_safe;
}
