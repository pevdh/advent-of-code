use aoc2022::*;

aoc_main!(
    day: 1,
    test_input:
    r#"
    test1234
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 0,
    task_2: task_2,
    expected_2: 0,
);

struct ParsedInput {}

fn parse(raw_input: &str) -> Result<ParsedInput> {
    println!("\"{}\"", raw_input);
    todo!();
}

fn task_1(input: &ParsedInput) -> Result<i32> {
    todo!();
}

fn task_2(input: &ParsedInput) -> Result<i32> {
    todo!();
}
