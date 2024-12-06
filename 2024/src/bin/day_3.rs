use aoc2024::*;
use regex::Regex;

aoc_main!(
    day: 3,
    test_input: r#"
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"#,
    test_input_2: r#"
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"#,
    task_1: task_1,
    expected_1: 161,
    task_2: task_2,
    expected_2: 48,
);

fn task_1(input: &str) -> Result<i64> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let multiplications: Vec<(i64, i64)> = regex
        .captures_iter(input)
        .map(|capture| {
            let left = capture.get(1).unwrap().as_str().parse().unwrap();
            let right = capture.get(2).unwrap().as_str().parse().unwrap();

            (left, right)
        })
        .collect::<Vec<(i64, i64)>>();

    Ok(multiplications
        .iter()
        .map(|(left, right)| left * right)
        .sum())
}

fn task_2(input: &str) -> Result<i64> {
    let regex = Regex::new(r"(do\(\))|(don\'t\(\))|((mul)\((?<left>\d+),(?<right>\d+)\))")?;

    let mut result = 0;
    let mut enabled = true;
    for capture in regex.captures_iter(input) {
        let op = capture.get(0).unwrap().as_str();

        if op.starts_with("do(") {
            enabled = true;
        } else if op.starts_with("don't(") {
            enabled = false;
        } else if op.starts_with("mul(") && enabled {
            let left: i64 = capture.name("left").unwrap().as_str().parse()?;
            let right: i64 = capture.name("right").unwrap().as_str().parse()?;

            result += left * right;
        }
    }

    Ok(result)
}
