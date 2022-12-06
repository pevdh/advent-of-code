use aoc2022::*;

aoc_main!(
    day: 3,
    test_input:
    r#"
    vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 157,
    task_2: task_2,
    expected_2: 70,
);

fn parse(raw_input: &str) -> Result<Vec<String>> {
    Ok(raw_input.lines().map(|line| line.to_owned()).collect())
}

fn task_1(input: &[String]) -> Result<u64> {
    Ok(input
        .iter()
        .map(|rucksack| {
            let num_items = rucksack.len();
            let first_compartment = rucksack.chars().take(num_items / 2);
            let first_compartment_items: HashSet<char> = HashSet::from_iter(first_compartment);

            let second_compartment = rucksack.chars().skip(num_items / 2);
            let second_compartment_items: HashSet<char> = HashSet::from_iter(second_compartment);

            let mut common_items = first_compartment_items.intersection(&second_compartment_items);

            *common_items.next().unwrap() as u8
        })
        .map(priority_of)
        .sum())
}

fn priority_of(item: u8) -> u64 {
    match item {
        b'a'..=b'z' => ((item - b'a') + 1) as u64,
        b'A'..=b'Z' => ((item - b'A') + 27) as u64,
        _ => panic!("Unknown item"),
    }
}

fn task_2(input: &[String]) -> Result<u64> {
    Ok(input
        .iter()
        .tuples()
        .map(|(a, b, c)| {
            let b: HashSet<char> = HashSet::from_iter(b.chars());
            let c: HashSet<char> = HashSet::from_iter(c.chars());

            let mut common_items = a
                .chars()
                .filter(|item| b.contains(item) && c.contains(item));

            common_items.next().unwrap() as u8
        })
        .map(priority_of)
        .sum())
}
