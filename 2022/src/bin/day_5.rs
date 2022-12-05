use aoc2022::*;

aoc_main!(
    day: 5,
    test_input:
    r#"
        [D]
    [N] [C]
    [Z] [M] [P]
     1   2   3

    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 2,
    task_2: task_2,
    expected_2: 4,
);

struct ParsedInput {
    stacks: Vec<Vec<char>>,
    rearrangement_procedure: Vec<RearrangementStep>,
}

struct RearrangementStep {
    number: u64,
    from: u64,
    to: u64,
}

fn parse(raw_input: &str) -> Result<ParsedInput> {
    Ok(())
}

fn parse_stacks(raw_input: &str) -> Result<(Vec<Vec<char>>, &str)> {

}

fn task_1(input: &ParsedInput) -> Result<usize> {
    Ok(0)
}

fn task_2(input: &ParsedInput) -> Result<usize> {
    Ok(0)
}
