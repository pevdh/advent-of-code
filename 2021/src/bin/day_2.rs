use aoc2021::*;

aoc_main!(
    day: 2,
    test_input: "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2",
    parser: parse,
    task_1: task_1,
    expected_1: 150,
    task_2: task_2,
    expected_2: 900,
);

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

struct ParsedInput {
    commands: Vec<Command>,
}

fn parse(raw_input: &str) -> Result<ParsedInput> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::char;
    use nom::character::complete::i32;
    use nom::character::complete::newline;
    use nom::combinator::all_consuming;
    use nom::combinator::map;
    use nom::multi::separated_list0;
    use nom::sequence::separated_pair;

    let forward = map(
        separated_pair(tag("forward"), char(' '), i32),
        |(_, value)| Command::Forward(value),
    );
    let up = map(separated_pair(tag("up"), char(' '), i32), |(_, value)| {
        Command::Up(value)
    });
    let down = map(separated_pair(tag("down"), char(' '), i32), |(_, value)| {
        Command::Down(value)
    });

    let line = alt((forward, up, down));
    let file = separated_list0(newline, line);

    let parser = all_consuming(map(file, |commands| ParsedInput { commands }));

    nom_parse(raw_input, parser)
}

fn task_1(input: &ParsedInput) -> Result<i32> {
    let (horizontal_position, depth) = input.commands.iter().fold(
        (0, 0),
        |(horizontal_position, depth), command| match command {
            Command::Forward(value) => (horizontal_position + value, depth),
            Command::Down(value) => (horizontal_position, depth + value),
            Command::Up(value) => (horizontal_position, depth - value),
        },
    );

    Ok(horizontal_position * depth)
}

fn task_2(input: &ParsedInput) -> Result<i32> {
    let (horizontal_position, depth, _) =
        input
            .commands
            .iter()
            .fold(
                (0, 0, 0),
                |(horizontal_position, depth, aim), command| match command {
                    Command::Forward(value) => {
                        (horizontal_position + value, depth + (aim * value), aim)
                    }
                    Command::Down(value) => (horizontal_position, depth, aim + value),
                    Command::Up(value) => (horizontal_position, depth, aim - value),
                },
            );

    Ok(horizontal_position * depth)
}
