use aoc2022::*;

aoc_main!(
    day: 1,
    test_input:
    r#"
    1000
    2000
    3000

    4000

    5000
    6000

    7000
    8000
    9000

    10000
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 24000,
    task_2: task_2,
    expected_2: 45000,
);

type ParsedInput = Vec<i32>;

fn parse(raw_input: &str) -> Result<ParsedInput> {
    let mut calories = Vec::new();

    let mut total = 0;
    for line in raw_input.lines() {
        if line == "" {
            calories.push(total);
            total = 0;
        } else {
            total += line.parse::<i32>().unwrap();
        }
    }

    calories.push(total);

    return Ok(calories);
}

fn task_1(input: &ParsedInput) -> Result<i32> {
    let mut calories = input.clone();
    calories.sort();
    calories.reverse();

    return Ok(calories[0]);
}

fn task_2(input: &ParsedInput) -> Result<i32> {
    let mut calories = input.clone();
    calories.sort();
    calories.reverse();

    return Ok(calories[0..3].iter().sum());
}
