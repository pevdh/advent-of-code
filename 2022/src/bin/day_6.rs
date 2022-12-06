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

fn parse(raw_input: &str) -> Result<Vec<u8>> {
    Ok(raw_input.chars().map(|c| c as u8).collect())
}

fn task_1(input: &[u8]) -> Result<usize> {
    let desired_unique_count = 4;
    let pos = input
        .windows(desired_unique_count)
        .position(all_unique);

    pos.map(|pos| pos + desired_unique_count)
        .ok_or_else(|| anyhow!("No solution"))
}

fn task_2(input: &[u8]) -> Result<usize> {
    let desired_unique_count = 14;
    let pos = input
        .windows(desired_unique_count)
        .position(all_unique);

    pos.map(|pos| pos + desired_unique_count)
        .ok_or_else(|| anyhow!("No solution"))
}

fn all_unique(characters: &[u8]) -> bool {
    assert!(characters.len() < 64);

    let mut m = 0u64;
    for ch in characters {
        m |= 1 << (*ch - b'a');
    }

    m.count_ones() as usize == characters.len()
}
