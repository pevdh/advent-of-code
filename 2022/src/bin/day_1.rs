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

fn parse(raw_input: &str) -> Result<Vec<Vec<i32>>> {
    Ok(raw_input.split("\n\n").fold(Vec::new(), |mut acc, p| {
        acc.push(p.lines().map(|l| l.parse().unwrap()).collect());

        acc
    }))
}

fn task_1(input: &[Vec<i32>]) -> Result<i32> {
    input
        .iter()
        // Sum each elf inventory
        .map(|calories| calories.iter().sum::<i32>())
        // Compute the highest total number of calories
        .sorted()
        .rev()
        .next()
        .ok_or_else(|| anyhow!("No solution"))
}

fn task_2(input: &[Vec<i32>]) -> Result<i32> {
    Ok(input
        .iter()
        // Sum each elf inventory
        .map(|calories| calories.iter().sum::<i32>())
        // Compute three highest total number of calories and sum them
        .sorted()
        .rev()
        .take(3)
        .sum())
}
