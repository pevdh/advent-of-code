use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

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
    let (pages_to_produce_in_each_update, rules) = parse(input)?;

    let mut num_valid = 0;

    for pages in pages_to_produce_in_each_update {
        if is_valid(&pages, &rules) {
            let middle = pages[pages.len() / 2];
            num_valid += middle;
        }
    }

    Ok(num_valid)
}

fn task_2(input: &str) -> Result<i64> {
    let (pages_to_produce_in_each_update, rules) = parse(input)?;

    let mut num_valid = 0;

    for pages in pages_to_produce_in_each_update {
        if !is_valid(&pages, &rules) {
            let fixed = fix(&pages, &rules);
            let middle = fixed[fixed.len() / 2];

            num_valid += middle;
        }
    }

    Ok(num_valid)
}

type RuleMap = HashMap<i64, HashSet<i64>>;
type ParsedInput = (Vec<Vec<i64>>, RuleMap);

fn parse(input: &str) -> Result<ParsedInput> {
    let (page_order_rules_part, page_updates_part) =
        input.split_once("\n\n").ok_or_parse_error()?;

    let page_order_rules: HashSet<(i64, i64)> = page_order_rules_part
        .split("\n")
        .map(|rule_part| {
            let (first, second) = rule_part.split_once("|").unwrap();

            (
                first.parse::<i64>().unwrap(),
                second.parse::<i64>().unwrap(),
            )
        })
        .collect();

    let mut rule_map: HashMap<i64, HashSet<i64>> = HashMap::default();

    for (first, second) in page_order_rules {
        let entry = rule_map.entry(first).or_default();
        entry.insert(second);
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

fn is_valid(pages: &[i64], rules: &RuleMap) -> bool {
    let mut seen = HashSet::default();
    for &page in pages {
        if let Some(after) = rules.get(&page) {
            if seen.intersection(after).count() > 0 {
                return false;
            }
        }

        seen.insert(page);
    }

    return true;
}

fn fix(pages: &[i64], rule_map: &RuleMap) -> Vec<i64> {
    let mut fixed = pages.to_vec();
    fixed.sort_unstable_by(|&a, &b| {
        // a|b means a needs to be sorted before b
        if rule_map.get(&a).unwrap_or(&HashSet::default()).contains(&b) {
            return Ordering::Greater;
        }

        // b|a means b needs to be sorted before a
        if rule_map.get(&b).unwrap_or(&HashSet::default()).contains(&a) {
            return Ordering::Less;
        }

        // otherwise we don't care
        return Ordering::Equal;
    });

    fixed
}
