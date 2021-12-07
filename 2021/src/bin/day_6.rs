use aoc2021::*;

fn parse(raw_input: &str) -> Result<Vec<i32>> {
    Ok(raw_input.split(',').map(|s| s.parse().unwrap()).collect())
}

fn simulate_lanternfish_growth_rate(starting_ages: &[i32], days: i32) -> i64 {
    let mut ages: [i64; 9] = [0; 9];

    for &input_age in starting_ages {
        ages[input_age as usize] += 1;
    }

    for _ in 0..days {
        let new_fish_to_add = ages[0];
        for i in 0..(ages.len() - 1) {
            ages[i] = ages[i + 1];
        }

        ages[6] += new_fish_to_add;
        ages[8] = new_fish_to_add;
    }

    ages.iter().sum()
}

fn task_1(ages: &Vec<i32>) -> Result<i64> {
    let num_fish = simulate_lanternfish_growth_rate(ages, 80);

    Ok(num_fish)
}

fn task_2(ages: &Vec<i32>) -> Result<i64> {
    let num_fish = simulate_lanternfish_growth_rate(ages, 256);

    Ok(num_fish)
}

aoc_main!(
    day: 6,
    test_input: "3,4,3,1,2",
    parser: parse,
    task_1: task_1,
    expected_1: 5934,
    task_2: task_2,
    expected_2: 26984457539,
);
