use aoc2024::*;

aoc_main!(
    day: 11,
    test_input: r#"
    125 17
"#,
    task_1: task_1,
    expected_1: 55312,
    task_2: task_2,
    expected_2: 65601038650482,
);

fn task_1(input: &str) -> Result<i64> {
    let mut stones = parse_nums(input)?.iter().map(|&n| (n, 1)).collect();

    for _ in 0..25 {
        stones = simulate_blink(&mut stones);
    }

    Ok(stones.values().sum())
}

fn task_2(input: &str) -> Result<i64> {
    let mut stones = parse_nums(input)?.iter().map(|&n| (n, 1)).collect();

    for _ in 0..75 {
        stones = simulate_blink(&mut stones);
    }

    Ok(stones.values().sum())
}

fn simulate_blink(stones: &mut HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut next = HashMap::default();
    for (&stone, &amount) in stones.iter() {
        if stone == 0 {
            *next.entry(1).or_insert(0) += amount;
        } else if let Some((left, right)) = split_if_even_digits(stone) {
            *next.entry(left).or_insert(0) += amount;
            *next.entry(right).or_insert(0) += amount;
        } else {
            *next.entry(stone * 2024).or_insert(0) += amount;
        }
    }

    next
}

fn split_if_even_digits(n: i64) -> Option<(i64, i64)> {
    let n_digits = n.ilog10() + 1;
    if n_digits % 2 == 0 {
        let half = n_digits / 2;

        let left = n / (10i64.pow(half));
        let right = n % (10i64.pow(half));

        return Some((left, right));
    }

    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_split_if_even_digits() {
        assert_eq!(split_if_even_digits(1234), Some((12, 34)));
        assert_eq!(split_if_even_digits(12345), None);
        assert_eq!(split_if_even_digits(123456), Some((123, 456)));
    }
}
