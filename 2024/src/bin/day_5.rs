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
    let (pages_to_produce_in_each_update, rules) = parse(input)?;

    let mut result = 0;
    for pages in pages_to_produce_in_each_update {
        if is_sorted_topologically(&pages, &rules) {
            let middle = pages[pages.len() / 2];
            result += middle;
        }
    }

    Ok(result)
}

fn task_2(input: &str) -> Result<i64> {
    let (pages_to_produce_in_each_update, rules) = parse(input)?;

    let mut result = 0;
    for pages in pages_to_produce_in_each_update {
        if !is_sorted_topologically(&pages, &rules) {
            let fixed = topological_sort(&pages, &rules);
            let middle = fixed[fixed.len() / 2];
            result += middle;
        }
    }

    Ok(result)
}

type RuleMap = HashMap<i64, Vec<i64>>;
type ParsedInput = (Vec<Vec<i64>>, RuleMap);

fn parse(input: &str) -> Result<ParsedInput> {
    let (page_order_rules_part, page_updates_part) =
        input.split_once("\n\n").ok_or_parse_error()?;

    let mut rules: HashMap<i64, Vec<i64>> = HashMap::default();

    for rule in page_order_rules_part.lines() {
        let (first, second) = parse_num_pair(rule)?;

        rules.entry(first).or_default().push(second);
    }

    let pages_to_produce_in_each_update: Vec<Vec<i64>> = page_updates_part
        .lines()
        .map(parse_nums)
        .collect::<Result<_>>()?;

    Ok((pages_to_produce_in_each_update, rules))
}

fn is_sorted_topologically(nodes: &[i64], graph: &HashMap<i64, Vec<i64>>) -> bool {
    let mut seen: HashSet<i64> = HashSet::default();
    for &node in nodes {
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if seen.contains(&neighbor) {
                    return false;
                }
            }
        }

        seen.insert(node);
    }

    true
}

fn topological_sort(nodes: &[i64], graph: &HashMap<i64, Vec<i64>>) -> Vec<i64> {
    let nodes_set: HashSet<i64> = HashSet::from_iter(nodes.iter().copied());
    let mut sorted = vec![];

    let mut in_degrees: HashMap<i64, i64> = HashMap::default();

    for &node in nodes {
        for &neighbor in graph.get(&node).unwrap_or(&vec![]) {
            if nodes_set.contains(&neighbor) {
                *in_degrees.entry(neighbor).or_insert(0) += 1;
            }
        }
    }

    let mut queue: Vec<i64> = nodes
        .iter()
        .copied()
        .filter(|&p| *in_degrees.get(&p).unwrap_or(&0) == 0)
        .collect::<Vec<i64>>();

    while let Some(node) = queue.pop() {
        sorted.push(node);

        for &neighbor in graph.get(&node).unwrap_or(&vec![]) {
            match in_degrees.get_mut(&neighbor) {
                Some(entry) => {
                    *entry -= 1;

                    if *entry == 0 {
                        queue.push(neighbor);
                    }
                }
                None => continue,
            }
        }
    }

    sorted
}
