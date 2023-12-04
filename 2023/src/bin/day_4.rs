use std::iter::repeat;

use aoc2023::*;

aoc_main!(
    day: 4,
    test_input: r#"
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#,
    task_1: task_1,
    expected_1: 13,
    task_2: task_2,
    expected_2: 30,
);

fn task_1(input: &str) -> Result<i64> {
    let mut score = 0;

    for line in input.lines() {
        let (_, cards) = line.split_once(": ").unwrap();
        let (winning_numbers, our_numbers) = cards.split_once(" | ").unwrap();

        let winning_numbers: HashSet<i64> = winning_numbers
            .split_whitespace()
            .map(|d| d.parse::<i64>().unwrap())
            .collect();
        let our_numbers: HashSet<i64> = our_numbers
            .split_whitespace()
            .map(|d| d.parse::<i64>().unwrap())
            .collect();

        let our_winning_numbers_count = winning_numbers.intersection(&our_numbers).count() as u32;
        if our_winning_numbers_count == 0 {
            continue;
        }

        score += 2i64.pow(our_winning_numbers_count - 1);
    }

    Ok(score)
}

fn task_2(input: &str) -> Result<i64> {
    let mut copies = Vec::from_iter(repeat(1).take(input.lines().count()));

    for (card_idx, line) in input.lines().enumerate() {
        let (_, cards) = line.split_once(": ").unwrap();
        let (winning_numbers, our_numbers) = cards.split_once(" | ").unwrap();

        let winning_numbers: HashSet<i64> = winning_numbers
            .split_whitespace()
            .map(|d| d.parse::<i64>().unwrap())
            .collect();

        let our_numbers: HashSet<i64> = our_numbers
            .split_whitespace()
            .map(|d| d.parse::<i64>().unwrap())
            .collect();

        let our_winning_numbers_count = winning_numbers.intersection(&our_numbers).count();

        for i in 1usize..=our_winning_numbers_count {
            if card_idx + i >= copies.len() {
                break;
            }

            copies[card_idx + i] += copies[card_idx];
        }
    }

    Ok(copies.iter().sum())
}
