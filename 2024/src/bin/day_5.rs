use std::collections::{HashMap, HashSet};

use aoc2024::*;

aoc_main!(
    day: 5,
    test_input: r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#,
    task_1: task_1,
    expected_1: 143,
    task_2: task_2,
    expected_2: 123,
);

fn task_1(input: &str) -> Result<i64> {
    let (pages_to_produce_in_each_update, rule_map) = parse(input)?;

    let mut num_valid = 0;

    for pages in pages_to_produce_in_each_update {
        if is_valid(&pages, &rule_map) {
            let middle = pages[pages.len() / 2];
            num_valid += middle;
        }
    }

    Ok(num_valid)
}

fn task_2(input: &str) -> Result<i64> {
    let (pages_to_produce_in_each_update, rule_map) = parse(input)?;

    let mut num_valid = 0;

    for pages in pages_to_produce_in_each_update {
        if !is_valid(&pages, &rule_map) {
            let fixed = fix(&pages, &rule_map);
            let middle = fixed[fixed.len() / 2];

            num_valid += middle;
        }
    }

    Ok(num_valid)
}

type ParsedInput = (Vec<Vec<i64>>, HashMap<i64, Vec<i64>>);

fn parse(input: &str) -> Result<ParsedInput> {
    let (page_order_rules_part, page_updates_part) =
        input.split_once("\n\n").ok_or_parse_error()?;

    let page_order_rules: Vec<(i64, i64)> = page_order_rules_part
        .split("\n")
        .map(|rule_part| {
            let (first, second) = rule_part.split_once("|").unwrap();

            (
                first.parse::<i64>().unwrap(),
                second.parse::<i64>().unwrap(),
            )
        })
        .collect();

    let mut rule_map: HashMap<i64, Vec<i64>> = HashMap::default();
    for (first, second) in page_order_rules {
        let entry = rule_map.entry(first).or_default();
        entry.push(second);
    }

    let pages_to_produce_in_each_update: Vec<Vec<i64>> = page_updates_part
        .lines()
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    Ok((pages_to_produce_in_each_update, rule_map))
}

fn is_valid(pages: &[i64], rules: &HashMap<i64, Vec<i64>>) -> bool {
    let mut seen: HashSet<i64> = HashSet::default();
    for &page in pages {
        for &second in rules.get(&page).unwrap_or(&vec![]) {
            let rule_valid = !seen.contains(&second);

            if !rule_valid {
                return false;
            }
        }

        seen.insert(page);
    }

    true
}

fn fix(pages: &[i64], rules: &HashMap<i64, Vec<i64>>) -> Vec<i64> {
    let mut seen: HashSet<i64> = HashSet::default();

    let mut fixed = pages.to_vec();
    let mut idx = 0;
    while idx < fixed.len() {
        let page = fixed[idx];

        for &second in rules.get(&page).unwrap_or(&vec![]) {
            let rule_valid = !seen.contains(&second);

            if !rule_valid {
                // if we have seen the second value then we need to move it to after the current page
                let second_idx = fixed.iter().position(|&p| p == second).unwrap();

                fixed.insert(idx + 1, second);
                fixed.remove(second_idx);

                seen.remove(&second);
                idx -= 1;
            }
        }

        seen.insert(page);
        idx += 1;
    }

    fixed
}
