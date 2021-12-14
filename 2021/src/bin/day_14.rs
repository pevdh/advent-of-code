use std::iter::FromIterator;

use aoc2021::*;

aoc_main!(
    day: 14,
    test_input: r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#,
    parser: parse,
    task_1: task_1,
    expected_1: 1588,
    task_2: task_2,
    expected_2: 2188189693529,
);

#[derive(Debug)]
struct ParsedInput {
    polymer_template: Vec<char>,
    pair_insertion_rules: HashMap<(char, char), char>,
}

fn parse(raw_input: &str) -> Result<ParsedInput> {
    use nom::{
        bytes::complete::{tag, take},
        character::complete::line_ending,
        combinator::{map, map_opt},
        multi::{count, many0, separated_list0},
        sequence::{separated_pair, tuple},
    };

    let uppercase_alphabetic = |i| {
        map_opt(take(1u32), |c: &str| {
            let c = c.chars().next().unwrap();

            if c.is_uppercase() {
                Some(c)
            } else {
                None
            }
        })(i)
    };

    let polymer_template = many0(uppercase_alphabetic);
    let pair_insertion_rule = separated_pair(
        tuple((uppercase_alphabetic, uppercase_alphabetic)),
        tag(" -> "),
        uppercase_alphabetic,
    );

    let pair_insertion_rules = map(
        separated_list0(line_ending, pair_insertion_rule),
        HashMap::from_iter,
    );

    let input = map(
        separated_pair(
            polymer_template,
            count(line_ending, 2),
            pair_insertion_rules,
        ),
        |(polymer_template, pair_insertion_rules)| ParsedInput {
            polymer_template,
            pair_insertion_rules,
        },
    );

    nom_parse(raw_input, input)
}

fn task_1(input: &ParsedInput) -> Result<u64> {
    let mut polymer = Polymer::from_template(&input.polymer_template);

    for _ in 0..10 {
        polymer.grow(&input.pair_insertion_rules);
    }

    Ok(polymer.most_common_element_count() - polymer.least_common_element_count())
}

fn task_2(input: &ParsedInput) -> Result<u64> {
    let mut polymer = Polymer::from_template(&input.polymer_template);

    for _ in 0..40 {
        polymer.grow(&input.pair_insertion_rules);
    }

    Ok(polymer.most_common_element_count() - polymer.least_common_element_count())
}

struct Polymer {
    element_counts: HashMap<char, u64>,
    pair_counts: HashMap<(char, char), u64>,
}

impl Polymer {
    fn from_template(polymer_template: &[char]) -> Self {
        let mut pair_counts = HashMap::new();
        let mut element_counts = HashMap::new();

        for &c in polymer_template {
            *element_counts.entry(c).or_insert(0) += 1;
        }

        for (&a, &b) in polymer_template.iter().zip(polymer_template.iter().skip(1)) {
            *pair_counts.entry((a, b)).or_insert(0) += 1;
        }

        Polymer {
            element_counts,
            pair_counts,
        }
    }

    fn grow(&mut self, pair_insertion_rules: &HashMap<(char, char), char>) {
        let mut new_pair_counts = self.pair_counts.clone();
        let mut new_element_counts = self.element_counts.clone();

        for (&pair, &result) in pair_insertion_rules.iter() {
            if let Some(&pair_count) = self.pair_counts.get(&pair) {
                if pair_count > 0 {
                    *new_pair_counts.get_mut(&pair).unwrap() -= pair_count;
                    *new_pair_counts.entry((pair.0, result)).or_insert(0) += pair_count;
                    *new_pair_counts.entry((result, pair.1)).or_insert(0) += pair_count;

                    *new_element_counts.entry(result).or_insert(0) += pair_count;
                }
            }
        }

        self.element_counts = new_element_counts;
        self.pair_counts = new_pair_counts;
    }

    fn most_common_element_count(&self) -> u64 {
        self.element_counts.iter().map(|(_, q)| *q).max().unwrap()
    }

    fn least_common_element_count(&self) -> u64 {
        self.element_counts.iter().map(|(_, q)| *q).min().unwrap()
    }
}
