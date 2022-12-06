use aoc2022::*;

aoc_main!(
    day: 4,
    test_input:
    r#"
    2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 2,
    task_2: task_2,
    expected_2: 4,
);

type AssignmentRange = (i32, i32);

fn parse(raw_input: &str) -> Result<Vec<(AssignmentRange, AssignmentRange)>> {
    Ok(raw_input
        .lines()
        .map(|line| {
            let mut s = line.split(',');
            let mut range_1_s = s.next().unwrap().split('-');
            let mut range_2_s = s.next().unwrap().split('-');

            let range_1_min = range_1_s.next().unwrap().parse().unwrap();
            let range_1_max = range_1_s.next().unwrap().parse().unwrap();

            let range_2_min = range_2_s.next().unwrap().parse().unwrap();
            let range_2_max = range_2_s.next().unwrap().parse().unwrap();

            ((range_1_min, range_1_max), (range_2_min, range_2_max))
        })
        .collect())
}

fn task_1(input: &[(AssignmentRange, AssignmentRange)]) -> Result<usize> {
    Ok(input
        .iter()
        .filter(|(a, b)| fully_contains(a, b) || fully_contains(b, a))
        .count())
}

fn fully_contains(a: &AssignmentRange, b: &AssignmentRange) -> bool {
    a.0 >= b.0 && a.1 <= b.1
}

fn task_2(input: &[(AssignmentRange, AssignmentRange)]) -> Result<usize> {
    Ok(input
        .iter()
        .filter(|(a, b)| partially_overlaps(a, b) || partially_overlaps(b, a))
        .count())
}

fn partially_overlaps(a: &AssignmentRange, b: &AssignmentRange) -> bool {
    // b.0 <= a.0 <= b.1 || b.0 <= a.1 <= b.1
    (b.0 <= a.0 && a.0 <= b.1) || (b.0 <= a.1 && a.1 <= b.1)
}
