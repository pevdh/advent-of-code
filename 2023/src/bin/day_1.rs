use aoc2023::*;
use ascii::{AsAsciiStr, IntoAsciiString};
use eyre::Context;

aoc_main!(
    day: 1,
    test_input: r#"
    1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet"#,
    test_input_2: r#"
    two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen"#,
    task_1: task_1,
    expected_1: 142,
    task_2: task_2,
    expected_2: 281,
);

fn task_1(input: &str) -> Result<u32> {
    let input = input
        .as_ascii_str()
        .with_context(|| "Input contains non-ASCII characters.")?;

    Ok(input
        .lines()
        .filter_map(|line| {
            let digits = line
                .chars()
                .filter_map(|ch| ch.to_digit(10))
                .collect::<Vec<u32>>();

            digits.first().zip(digits.last()).map(|(f, l)| f * 10 + l)
        })
        .sum())
}

fn task_2(input: &str) -> Result<u32> {
    let input = input
        .as_ascii_str()
        .wrap_err_with(|| "Input contains non-ASCII characters.")?;

    let spelled_out_digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .map(|s| {
        s.into_ascii_string()
            .expect("parsing as ASCII string failed")
    });

    let answer = input
        .lines()
        .filter_map(|line| {
            let mut digits = vec![];

            for idx in 0..line.len() {
                if let Some(digit) = line[idx].to_digit(10) {
                    digits.push(digit);
                    continue;
                }

                for (digit_idx, spelled_out_digit) in spelled_out_digits.iter().enumerate() {
                    if line[idx..].starts_with(spelled_out_digit) {
                        digits.push(digit_idx as u32);
                    }
                }
            }

            digits.first().zip(digits.last()).map(|(f, l)| f * 10 + l)
        })
        .sum();

    Ok(answer)
}
