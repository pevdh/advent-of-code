use aoc2023::*;

aoc_main!(
    day: 9,
    test_input: r#"
    0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45"#,
    task_1: task_1,
    expected_1: 114,
    task_2: task_2,
    expected_2: 2,
);

fn task_1(input: &str) -> Result<i64> {
    let answer = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|history| extrapolate(&history))
        .sum();

    Ok(answer)
}

fn task_2(input: &str) -> Result<i64> {
    let answer = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|history| extrapolate_backwards(&history))
        .sum();

    Ok(answer)
}

fn extrapolate(history: &[i64]) -> i64 {
    let mut histories: Vec<Vec<i64>> = vec![history.to_vec()];

    while !histories.last().unwrap().iter().all(|v| *v == 0) {
        histories.push(differentiate(histories.last().unwrap()));
    }

    histories
        .iter()
        .rev()
        .filter_map(|history| history.last())
        .fold(0, |delta, first| *first + delta)
}

fn differentiate(history: &[i64]) -> Vec<i64> {
    history
        .iter()
        .tuple_windows()
        .map(|(a, b)| (b - a))
        .collect()
}

fn extrapolate_backwards(history: &[i64]) -> i64 {
    let mut histories: Vec<Vec<i64>> = vec![history.to_vec()];

    while !histories.last().unwrap().iter().all(|v| *v == 0) {
        histories.push(differentiate(histories.last().unwrap()));
    }

    histories
        .iter()
        .rev()
        .filter_map(|history| history.first())
        .fold(0, |diff, first| *first - diff)
}
