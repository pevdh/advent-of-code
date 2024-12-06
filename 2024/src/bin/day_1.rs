use aoc2024::*;

aoc_main!(
    day: 1,
    test_input: r#"
3   4
4   3
2   5
1   3
3   9
3   3
"#,
    task_1: task_1,
    expected_1: 11,
    task_2: task_2,
    expected_2: 31,
);

fn task_1(input: &str) -> Result<i64> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        let (left, right) = parse_num_pair(line)?;
        left_list.push(left);
        right_list.push(right);
    }

    left_list.sort();
    right_list.sort();

    let sum = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(&left, &right)| (left - right).abs())
        .sum();

    Ok(sum)
}

fn task_2(input: &str) -> Result<i64> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        let (left, right) = parse_num_pair(line)?;
        left_list.push(left);
        right_list.push(right);
    }

    let right_list_counts: HashMap<i64, i64> = right_list.into_iter().frequencies();
    let similarity_score = left_list
        .iter()
        .map(|&left| left * right_list_counts.get(&left).unwrap_or(&0));

    Ok(similarity_score.sum())
}
