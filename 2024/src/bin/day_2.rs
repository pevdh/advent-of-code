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
    let reports = input.lines().map(|line| parse_nums(line).unwrap());

    let num_safe_reports = reports
        .filter(|report| is_safe(report.iter().copied()))
        .count();

    Ok(num_safe_reports)
}

fn task_2(input: &str) -> Result<usize> {
    let reports = input.lines().map(|line| parse_nums(line).unwrap());

    let num_safe_reports = reports
        .filter(|report| is_safe(report.iter().copied()) || is_safe_with_tolerance(report))
        .count();

    Ok(num_safe_reports)
}

fn is_safe_with_tolerance(levels: &[i64]) -> bool {
    let is_safe = (0..levels.len()).any(|dropped_idx| {
        let modified_report = levels
            .iter()
            .enumerate()
            .filter(|&(idx, _)| idx != dropped_idx)
            .map(|(_, &level)| level);

        is_safe(modified_report)
    });

    is_safe
}

fn is_safe(report: impl Iterator<Item = i64>) -> bool {
    let (inc, dec, diffs) = report
        .tuple_windows()
        .map(|(a, b)| b - a)
        .fold((true, true, true), |(inc, dec, diffs), d| {
            (inc && d > 0, dec && d < 0, diffs && (1..=3).contains(&d))
        });

    (inc || dec) && diffs
}
