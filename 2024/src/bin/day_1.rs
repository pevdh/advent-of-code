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
        let mut iter = line.split_whitespace();
        let left = iter.next().unwrap().parse::<i64>()?;
        let right = iter.next().unwrap().parse::<i64>()?;
        left_list.push(left);
        right_list.push(right);
    }

    left_list.sort();
    right_list.sort();

    let mut sum = 0;
    for i in 0..left_list.len() {
        let dist = (left_list[i] - right_list[i]).abs();
        sum += dist;
    }

    Ok(sum)
}

fn task_2(input: &str) -> Result<i64> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let left = iter.next().unwrap().parse::<i64>()?;
        let right = iter.next().unwrap().parse::<i64>()?;
        left_list.push(left);
        right_list.push(right);
    }

    let mut right_list_counts= HashMap::default();
    for right in right_list {
        let count = right_list_counts.entry(right).or_insert(0i64);
        *count += 1;
    }

    let mut similarity_score = 0;
    for i in 0..left_list.len() {
        let left = left_list[i];
        let occurrences = right_list_counts.get(&left).unwrap_or(&0);

        similarity_score += left * occurrences;
    }

    Ok(similarity_score)
}
