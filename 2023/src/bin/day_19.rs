use std::collections::HashMap;

use aoc2023::*;

aoc_main!(
    day: 19,
    test_input: r#"
    px{a<2006:qkq,m>2090:A,rfg}
    pv{a>1716:R,A}
    lnx{m>1548:A,A}
    rfg{s<537:gd,x>2440:R,A}
    qs{s>3448:A,lnx}
    qkq{x<1416:A,crn}
    crn{x>2662:A,R}
    in{s<1351:px,qqz}
    qqz{s>2770:qs,m<1801:hdj,R}
    gd{a>3333:R,R}
    hdj{m>838:A,pv}

    {x=787,m=2655,a=1222,s=2876}
    {x=1679,m=44,a=2067,s=496}
    {x=2036,m=264,a=79,s=2244}
    {x=2461,m=1339,a=466,s=291}
    {x=2127,m=1623,a=2188,s=1013}"#,
    task_1: task_1,
    expected_1: 19114,
    task_2: task_2,
    expected_2: 167409079868000,
);

fn task_1(input: &str) -> Result<i64> {
    let (workflows_part, parts_part) = input.split_once("\n\n").ok_or_parse_error()?;

    let workflows = parse_workflows(workflows_part)?;
    let parts = parse_parts(parts_part)?;

    let answer = parts
        .iter()
        .map(|part| (part, run_workflow(part, &workflows).unwrap()))
        .filter(|&(_part, workflow_result)| workflow_result)
        .map(|(part, _)| part.0 + part.1 + part.2 + part.3)
        .sum();

    Ok(answer)
}

fn task_2(input: &str) -> Result<u64> {
    let (workflows_part, _parts_part) = input.split_once("\n\n").ok_or_parse_error()?;

    let workflows = parse_workflows(workflows_part)?;

    compute_num_accepted(&workflows)
}

type Range = (u64, u64);

#[derive(Debug, Clone, Copy)]
struct CategoryRanges {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

#[derive(Debug)]
struct Rule<'a> {
    condition: Option<Condition>,
    label: &'a str,
}

#[derive(Debug, Clone, Copy)]
struct Condition {
    lhs: char,
    op: char,
    rhs: i64,
}

type Part = (i64, i64, i64, i64);

impl CategoryRanges {
    fn split_on_condition(&self, condition: &Condition) -> (CategoryRanges, CategoryRanges) {
        let mut accepted = *self;
        let mut rejected = *self;

        match (condition.lhs, condition.op, condition.rhs) {
            ('x', op, rhs) => {
                let (accepted_x, rejected_x) = split_on_condition(self.x, op, rhs as u64);
                accepted.x = accepted_x;
                rejected.x = rejected_x;
            }
            ('m', op, rhs) => {
                let (accepted_m, rejected_m) = split_on_condition(self.m, op, rhs as u64);
                accepted.m = accepted_m;
                rejected.m = rejected_m;
            }
            ('a', op, rhs) => {
                let (accepted_a, rejected_a) = split_on_condition(self.a, op, rhs as u64);
                accepted.a = accepted_a;
                rejected.a = rejected_a;
            }
            ('s', op, rhs) => {
                let (accepted_s, rejected_s) = split_on_condition(self.s, op, rhs as u64);
                accepted.s = accepted_s;
                rejected.s = rejected_s;
            }
            _ => panic!("unhandled case: {}", condition.lhs),
        };

        (accepted, rejected)
    }

    fn num_combinations(&self) -> u64 {
        let num_x = self.x.1 - self.x.0 + 1;
        let num_m = self.m.1 - self.m.0 + 1;
        let num_a = self.a.1 - self.a.0 + 1;
        let num_s = self.s.1 - self.s.0 + 1;

        num_x * num_m * num_a * num_s
    }
}

fn split_on_condition(range: Range, op: char, rhs: u64) -> (Range, Range) {
    let (min, max) = range;

    match op {
        '<' => {
            let accepted = (min, (rhs - 1));
            let rejected = (rhs, max);

            (accepted, rejected)
        }
        '>' => {
            let accepted = (rhs + 1, max);
            let rejected = (min, rhs);

            (accepted, rejected)
        }
        _ => panic!("unknown op: {}", op),
    }
}

const ENTIRE_RANGE: Range = (1, 4000);

fn compute_num_accepted(workflows: &HashMap<&str, Vec<Rule>>) -> Result<u64> {
    let mut to_visit: VecDeque<(&str, CategoryRanges)> = VecDeque::new();
    to_visit.push_back((
        "in",
        CategoryRanges {
            x: ENTIRE_RANGE,
            m: ENTIRE_RANGE,
            a: ENTIRE_RANGE,
            s: ENTIRE_RANGE,
        },
    ));

    let mut accepted_ranges = vec![];

    while let Some((label, mut category_ranges)) = to_visit.pop_front() {
        if label == "A" {
            accepted_ranges.push(category_ranges);
            continue;
        }

        if label == "R" {
            continue;
        }

        let workflow = workflows.get(label).unwrap();

        for rule in workflow {
            match rule.condition {
                Some(condition) => {
                    let (accepted, rejected) = category_ranges.split_on_condition(&condition);

                    to_visit.push_back((rule.label, accepted));

                    category_ranges = rejected;
                }
                None => {
                    to_visit.push_back((rule.label, category_ranges));
                }
            }
        }
    }

    Ok(accepted_ranges
        .iter()
        .map(|r| r.num_combinations())
        .sum::<u64>())
}

fn run_workflow(part: &Part, workflows: &HashMap<&str, Vec<Rule>>) -> Result<bool> {
    let mut current = workflows.get("in");

    while let Some(current_rules) = current {
        for rule in current_rules {
            let next_label = match &rule.condition {
                Some(condition) => {
                    if !evaluate_condition(part, &condition) {
                        continue;
                    }

                    rule.label
                }
                None => rule.label,
            };

            if next_label == "A" {
                return Ok(true);
            }

            if next_label == "R" {
                return Ok(false);
            }

            current = workflows.get(next_label);
            break;
        }
    }

    Err(eyre!("no solution"))
}

fn evaluate_condition(part: &Part, condition: &Condition) -> bool {
    let lhs = match condition.lhs {
        'x' => part.0,
        'm' => part.1,
        'a' => part.2,
        's' => part.3,
        _ => panic!("unknown lhs: {}", condition.lhs),
    };

    match condition.op {
        '>' => lhs > condition.rhs,
        '<' => lhs < condition.rhs,
        _ => panic!("unknown operation: {}", condition.op),
    }
}

fn parse_workflows(workflows_part: &str) -> Result<HashMap<&str, Vec<Rule>>> {
    let mut workflows = HashMap::default();

    for line in workflows_part.lines() {
        let (label, workflow) = parse_workflow(line)?;

        workflows.insert(label, workflow);
    }

    Ok(workflows)
}

fn parse_parts(parts_part: &str) -> Result<Vec<Part>> {
    let mut parts = vec![];

    for line in parts_part.lines() {
        parts.push(parse_part(line)?);
    }

    Ok(parts)
}

fn parse_workflow(workflow_line: &str) -> Result<(&str, Vec<Rule>)> {
    use nom::character::complete::char;
    use nom::character::complete::i64;
    use nom::{
        branch::alt,
        character::complete::{alpha1, anychar},
        combinator::map,
        multi::separated_list0,
        sequence::{delimited, tuple},
    };

    let op = |i| alt((char('<'), char('>')))(i);
    let condition = |i| {
        map(tuple((anychar, op, i64)), |(lhs, op, rhs)| Condition {
            lhs,
            op,
            rhs,
        })(i)
    };

    let rule_with_condition = |i| {
        map(
            tuple((condition, char(':'), alpha1)),
            |(condition, _, label)| Rule {
                condition: Some(condition),
                label,
            },
        )(i)
    };

    let rule_without_condition = |i| {
        map(alpha1, |label| Rule {
            condition: None,
            label,
        })(i)
    };

    let rules = |i| {
        separated_list0(
            char(','),
            alt((rule_with_condition, rule_without_condition)),
        )(i)
    };

    let workflow = |i| {
        map(
            tuple((alpha1, delimited(char('{'), rules, char('}')))),
            |(label, rules)| (label, rules),
        )(i)
    };

    nom_parse(workflow_line, workflow)
}

fn parse_part(part_line: &str) -> Result<(i64, i64, i64, i64)> {
    use nom::character::complete::char;
    use nom::character::complete::i64;
    use nom::{
        character::complete::anychar,
        combinator::map,
        sequence::{delimited, tuple},
    };

    let rating = |i| map(tuple((anychar, char('='), i64)), |(_, _, n)| n)(i);

    let part = |i| {
        map(
            delimited(
                char('{'),
                tuple((
                    rating,
                    char(','),
                    rating,
                    char(','),
                    rating,
                    char(','),
                    rating,
                )),
                char('}'),
            ),
            |(x, _, m, _, a, _, s)| (x, m, a, s),
        )(i)
    };

    nom_parse(part_line, part)
}

