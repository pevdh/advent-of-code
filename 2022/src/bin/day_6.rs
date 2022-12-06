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
    let pos = (4..input.len()).find(|i| count_unique(&input[i - 4..*i]) == 4);

    pos.ok_or_else(|| anyhow!("No solution"))
}

fn task_2(input: &[char]) -> Result<usize> {
    let pos = (14..input.len()).find(|i| count_unique(&input[i - 14..*i]) == 14);

    pos.ok_or_else(|| anyhow!("No solution"))
}

fn count_unique(characters: &[char]) -> u32 {
    let mut m = 0u64;
    for ch in characters {
        m |= 1 << (*ch as u8 - b'a');
    }

    return m.count_ones();
}
