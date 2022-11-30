use aoc2020::*;

aoc_main!(
    day: 2,
    test_input:
    r#"
    1-3 a: abcde
    1-3 b: cdefg
    2-9 c: ccccccccc
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 2,
    task_2: task_2,
    expected_2: 1,
);

#[derive(Debug)]
struct Policy {
    a: i64,
    b: i64,
    letter: char,
}

type ParsedInput = Vec<(Policy, String)>;

fn parse(raw_input: &str) -> Result<ParsedInput> {
    let mut input = Vec::new();

    for line in raw_input.lines() {
        // Example: 1-3 a: abcde
        let mut split = line.split(&['-', ' ', ':']);

        let min_occurences: i64 = split.next().unwrap().parse()?;
        let max_occurences: i64 = split.next().unwrap().parse()?;
        let letter: char = split.next().unwrap().chars().next().unwrap();
        let password = split.last().unwrap();

        input.push((
            Policy {
                a: min_occurences,
                b: max_occurences,
                letter,
            },
            password.into(),
        ))
    }

    return Ok(input);
}

fn task_1(input: &ParsedInput) -> Result<i64> {
    let mut num_valid = 0;

    for (policy, password) in input {
        let occurrences = password.chars().filter(|&c| c == policy.letter).count() as i64;

        if occurrences >= policy.a && occurrences <= policy.b {
            num_valid += 1;
        }
    }

    return Ok(num_valid);
}

fn task_2(input: &ParsedInput) -> Result<i64> {
    let mut num_valid = 0;

    for (policy, password) in input {
        let first_valid = password.chars().nth(policy.a as usize - 1) == Some(policy.letter);
        let second_valid = password.chars().nth(policy.b as usize - 1) == Some(policy.letter);

        if first_valid ^ second_valid {
            num_valid += 1;
        }
    }

    return Ok(num_valid);
}
