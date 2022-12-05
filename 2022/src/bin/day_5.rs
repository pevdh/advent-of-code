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
    expected_1: "CMZ".to_owned(),
    task_2: task_2,
    expected_2: "MCD".to_owned(),
);

#[derive(Debug)]
struct ParsedInput {
    stacks: Vec<Vec<char>>,
    rearrangement_procedure: Vec<RearrangementStep>,
}

#[derive(Debug)]
struct RearrangementStep {
    number: usize,
    from: usize,
    to: usize,
}

fn parse(raw_input: &str) -> Result<ParsedInput> {
    let mut s = raw_input.split("\n\n");

    let stacks_s = s.next().unwrap();
    let steps_s = s.next().unwrap();

    Ok(ParsedInput {
        stacks: parse_stacks(stacks_s),
        rearrangement_procedure: parse_steps(steps_s),
    })
}

fn parse_stacks(raw_input: &str) -> Vec<Vec<char>> {
    return raw_input
        .columns()
        .skip(1)
        .step_by(4)
        .map(|column| {
            column
                .chars()
                .filter(|c| c.is_ascii_uppercase())
                .rev()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
}

fn parse_steps(raw_input: &str) -> Vec<RearrangementStep> {
    return raw_input
        .lines()
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<_>>();

            RearrangementStep {
                number: parts[1].parse().unwrap(),
                from: parts[3].parse().unwrap(),
                to: parts[5].parse().unwrap(),
            }
        })
        .collect();
}

fn task_1(input: &ParsedInput) -> Result<String> {
    let mut stacks = input.stacks.clone();

    apply_procedure_with_cratemover_9000(&mut stacks, &input.rearrangement_procedure);

    Ok(stacks
        .iter()
        .filter_map(|s| s.last())
        .collect::<String>())
}

fn apply_procedure_with_cratemover_9000(
    stacks: &mut [Vec<char>],
    procedure: &[RearrangementStep],
) {
    for step in procedure {
        let from_stack = &mut stacks[step.from - 1];
        let crates = from_stack.split_off(from_stack.len() - step.number);

        let to_stack = &mut stacks[step.to - 1];
        to_stack.extend(crates.iter().rev());
    }
}

fn task_2(input: &ParsedInput) -> Result<String> {
    let mut stacks = input.stacks.clone();

    apply_procedure_with_cratemover_9001(&mut stacks, &input.rearrangement_procedure);

    Ok(stacks
        .iter()
        .filter_map(|s| s.last())
        .collect::<String>())
}

fn apply_procedure_with_cratemover_9001(
    stacks: &mut [Vec<char>],
    procedure: &[RearrangementStep],
) {
    for step in procedure {
        let from_stack = &mut stacks[step.from - 1];
        let crates = from_stack.split_off(from_stack.len() - step.number);

        let to_stack = &mut stacks[step.to - 1];
        to_stack.extend(crates.iter());
    }
}
