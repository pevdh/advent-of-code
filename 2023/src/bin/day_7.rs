use std::cmp::Ordering;

use aoc2023::*;

aoc_main!(
    day: 7,
    test_input: r#"
    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483"#,
    task_1: task_1,
    expected_1: 6440,
    task_2: task_2,
    expected_2: 5905,
);

fn task_1(input: &str) -> Result<u64> {
    let hands_to_bids: HashMap<Hand, u64> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();

            (analyze_hand(hand), bid.parse().unwrap())
        })
        .collect();

    Ok(calculate_total_winnings(&hands_to_bids))
}

fn task_2(input: &str) -> Result<u64> {
    // We use "X" for the joker card
    let hands_to_bids: HashMap<Hand, u64> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();

            (analyze_hand(&hand.replace('J', "X")), bid.parse().unwrap())
        })
        .collect();

    Ok(calculate_total_winnings(&hands_to_bids))
}

fn calculate_total_winnings(hands_to_bids: &HashMap<Hand, u64>) -> u64 {
    let sorted_hands = hands_to_bids.keys().sorted_by(|&a, &b| compare_hand(a, b));

    let num_hands = hands_to_bids.len();

    let total_winnings = sorted_hands
        .zip(1..=num_hands)
        .map(|(hand, rank)| hands_to_bids.get(hand).unwrap() * rank as u64)
        .sum();

    total_winnings
}

const CARD_ORDER: [char; 14] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'X',
];

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Hand {
    cards: String,
    type_value: u64,
}

fn analyze_hand(cards: &str) -> Hand {
    Hand {
        cards: cards.to_string(),
        type_value: determine_type_value_with_joker(cards),
    }
}

fn determine_type_value_with_joker(cards: &str) -> u64 {
    if !cards.contains('X') {
        return determine_type_value(cards);
    }

    CARD_ORDER
        .iter()
        .map(|replacement| {
            let new_cards = cards.replace('X', &replacement.to_string());
            let new_value = determine_type_value(&new_cards);

            new_value
        })
        .max()
        .unwrap() as u64
}

fn determine_type_value(cards: &str) -> u64 {
    // count number of pairs
    cards
        .chars()
        .cartesian_product(cards.chars())
        .filter(|(a, b)| a == b)
        .count() as u64
}

fn compare_hand(a: &Hand, b: &Hand) -> Ordering {
    if a.type_value != b.type_value {
        return a.type_value.cmp(&b.type_value);
    }

    for (a, b) in a.cards.chars().zip(b.cards.chars()) {
        match compare_card(a, b) {
            Ordering::Equal => continue,
            ordering => return ordering,
        }
    }

    Ordering::Equal
}

fn compare_card(a: char, b: char) -> Ordering {
    if a == b {
        return Ordering::Equal;
    }

    let a_order = CARD_ORDER.iter().position(|&ch| a == ch).unwrap();
    let b_order = CARD_ORDER.iter().position(|&ch| b == ch).unwrap();

    // We're supposed to return ascending order: higher value cards
    // are sorted after lower value cards.
    b_order.cmp(&a_order)
}
