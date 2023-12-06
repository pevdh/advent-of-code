use aoc2023::*;

aoc_main!(
    day: 6,
    test_input: r#"
    Time:      7  15   30
    Distance:  9  40  200"#,
    task_1: task_1,
    expected_1: 288,
    task_2: task_2,
    expected_2: 71503,
);

fn task_1(input: &str) -> Result<u64> {
    let mut lines = input.lines();

    let times = lines
        .next()
        .ok_or_parse_error()?
        .split_whitespace()
        .skip(1)
        .map(|d| d.parse::<i64>().unwrap());

    let distances = lines
        .next()
        .ok_or_parse_error()?
        .split_whitespace()
        .skip(1)
        .map(|d| d.parse::<i64>().unwrap());

    let races = times.zip(distances);

    let solution = races
        .map(|(time, distance)| {
            let wins = (1..time)
                .map(|hold| ((time - hold) * hold) > distance)
                .filter(|&win| win)
                .count();

            wins as u64
        })
        .product();

    Ok(solution)
}

fn task_2(input: &str) -> Result<u64> {
    let mut lines = input.lines();
    let times_line = lines.next().ok_or_parse_error()?;
    let distances_line = lines.next().ok_or_parse_error()?;

    let time = times_line["Time:".len()..]
        .replace(' ', "")
        .parse::<u64>()?;

    let distance = distances_line["Distance:".len()..]
        .replace(' ', "")
        .parse::<u64>()?;

    let min_hold = min_hold(time, distance);
    let wins = time - 2 * min_hold + 1;

    Ok(wins)
}

fn min_hold(time: u64, distance: u64) -> u64 {
    let (mut min, mut max) = (1u64, time / 2);

    loop {
        let midpoint = (max - min) / 2 + min;

        if ((time - midpoint) * midpoint) > distance {
            // search left half
            max = midpoint;
        } else {
            // search right half
            min = midpoint;
        }

        if (max - min) == 1 {
            return max;
        }
    }
}
