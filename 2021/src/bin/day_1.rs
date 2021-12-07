use aoc2021::*;

struct ParsedInput {
    depths: Vec<i32>,
}

fn parse(raw_input: &str) -> Result<ParsedInput> {
    use nom::character::complete::i32;
    use nom::character::complete::newline;
    use nom::multi::separated_list0;
    use nom::combinator::map;
    use nom::combinator::all_consuming;

    let line = i32;
    let file = separated_list0(newline, line);
    let parser = all_consuming(map(file, |depths| ParsedInput { depths }));

    nom_parse(raw_input, parser)
}

fn count_increased(input: &ParsedInput) -> Result<i32> {
    let first = input.depths.iter();
    let second = input.depths.iter().skip(1);

    let increased = first.zip(second)
        .map(|(a, b)| {
            a < b
        })
        .filter(|&b| b)
        .count();

    Ok(increased as i32)
}

fn count_increased_sliding_window(input: &ParsedInput) -> Result<i32> {
    let window_size = 3;

    let first = input.depths.windows(window_size);
    let second = input.depths.windows(window_size).skip(1);
    let increased = first.zip(second)
        .map(|(a, b)| {
            a[0] + a[1] + a[2] < b[0] + b[1] + b[2]
        })
        .filter(|&b| b)
        .count();

    Ok(increased as i32)
}

aoc_main!(
    day: 1,
    test_input: "199\n200\n208\n210\n200\n207\n240\n269\n260\n263",
    parser: parse,
    task_1: count_increased,
    expected_1: 7,
    task_2: count_increased_sliding_window,
    expected_2: 5,
);
