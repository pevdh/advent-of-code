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
    let total_points =
        input
            .lines()
            .map(parse_numbers)
            .fold(0, |acc, (winning_numbers, our_numbers)| {
                let win_count = winning_numbers.intersection(&our_numbers).count() as i64;

                acc + win_count.signum() * 2i64.pow((win_count - 1) as u32)
            });

    Ok(total_points)
}

fn parse_numbers(line: &str) -> (HashSet<i64>, HashSet<i64>) {
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

    (winning_numbers, our_numbers)
}

fn task_2(input: &str) -> Result<i64> {
    let win_counts = input
        .lines()
        .map(parse_numbers)
        .map(|(winning_numbers, our_numbers)| winning_numbers.intersection(&our_numbers).count());

    let card_counts = vec![1; input.lines().count()];

    let total_scratchcards = win_counts
        .enumerate()
        .scan(card_counts, |card_counts, (card_idx, win_count)| {
            let current_card_count = card_counts[card_idx];

            let range_to_update = (card_idx + 1)..=(card_idx + win_count);

            card_counts[range_to_update]
                .iter_mut()
                .for_each(|c| *c += current_card_count);

            Some(current_card_count)
        })
        .sum();

    Ok(total_scratchcards)
}
