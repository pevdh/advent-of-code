use aoc2021::*;
use std::cmp;

#[derive(Default, Debug, PartialEq)]
struct ParsedInput {
    positions: Vec<i32>,
}

fn parse(raw_input: &str) -> ParseResult<ParsedInput> {
    Ok(("", ParsedInput {
        positions: raw_input
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect()
    }))
}

fn task_1(input: &ParsedInput) -> Result<i64> {
    fn calculate_fuel_consumption(target_position: i32, positions: &[i32]) -> i32 {
        positions.iter()
            .map(|pos| (target_position - pos).abs())
            .sum()
    }

    let mut positions = input.positions.clone();
    positions.sort();

    let target_position = stats::median(positions.clone().into_iter())
        .expect("Unable to find median");

    let fuel_consumption = cmp::min(
        calculate_fuel_consumption(target_position.floor() as i32, &positions),
        calculate_fuel_consumption(target_position.ceil() as i32, &positions)
    );

    Ok(fuel_consumption as i64)
}

fn task_2(input: &ParsedInput) -> Result<i64> {
    fn calculate_fuel_consumption(target_position: i32, positions: &[i32]) -> i32 {
        positions.iter()
            .map(|pos| {
                let difference = (target_position - pos).abs();
                (1..=difference).sum::<i32>()
            })
            .sum()
    }

    let target_position = stats::mean(input.positions.clone().into_iter());

    let fuel_consumption = cmp::min(
        calculate_fuel_consumption(target_position.floor() as i32, &input.positions),
        calculate_fuel_consumption(target_position.ceil() as i32, &input.positions)
    );

    Ok(fuel_consumption as i64)
}

aoc_main!(
    day: 7,
    test_input: "16,1,2,0,4,2,7,1,2,14",
    parser: parse,
    task_1: task_1,
    expected_1: 37,
    task_2: task_2,
    expected_2: 168,
);
