use aoc2021::*;
use std::cmp;
use anyhow::Context;

fn parse(raw_input: &str) -> Result<Vec<i32>> {
    raw_input
        .split(',')
        .map(|s| s.parse().with_context(|| format!("Failed to parse number in input: {}", s)))
        .collect()
}

fn task_1(input: &Vec<i32>) -> Result<i64> {
    fn calculate_fuel_consumption(target_position: i32, positions: &[i32]) -> i32 {
        positions.iter()
            .map(|pos| (target_position - pos).abs())
            .sum()
    }

    let mut positions = input.clone();
    positions.sort();

    let target_position = stats::median(positions.clone().into_iter())
        .expect("Unable to find median");

    let fuel_consumption = cmp::min(
        calculate_fuel_consumption(target_position.floor() as i32, &positions),
        calculate_fuel_consumption(target_position.ceil() as i32, &positions)
    );

    Ok(fuel_consumption as i64)
}

fn task_2(input: &Vec<i32>) -> Result<i64> {
    fn calculate_fuel_consumption(target_position: i32, positions: &[i32]) -> i32 {
        positions.iter()
            .map(|pos| {
                let difference = (target_position - pos).abs();
                (1..=difference).sum::<i32>()
            })
            .sum()
    }

    let target_position = stats::mean(input.clone().into_iter());

    let fuel_consumption = cmp::min(
        calculate_fuel_consumption(target_position.floor() as i32, &input),
        calculate_fuel_consumption(target_position.ceil() as i32, &input)
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
