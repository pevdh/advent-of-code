use aoc2022::*;

aoc_main!(
    day: 6,
    test_input:
    r#"
    mjqjpqmgbljsphdztnvjfqwrcgsmlb
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 7,
    task_2: task_2,
    expected_2: 19,
);

fn parse(raw_input: &str) -> Result<Vec<char>> {
    Ok(raw_input.chars().collect())
}

fn task_1(input: &[char]) -> Result<usize> {
    let pos = (4..input.len()).find(|i| {
        let set: HashSet<char> = HashSet::from_iter(input[i - 4..*i].iter().cloned());

        set.len() == 4
    });

    pos.ok_or_else(|| anyhow!("No solution"))
}

fn task_2(input: &[char]) -> Result<usize> {
    let pos = (14..input.len()).find(|i| {
        let set: HashSet<char> = HashSet::from_iter(input[i - 14..*i].iter().cloned());

        set.len() == 14
    });

    pos.ok_or_else(|| anyhow!("No solution"))
}
