use aoc2022::*;

use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Integer(u32),
    List(Vec<Value>),
}

fn parse(raw_input: &str) -> Result<Vec<(Value, Value)>> {
    Ok(raw_input
        .split("\n\n")
        .map(|pair_s| {
            let mut lines = pair_s.lines();

            let left = parse_value(lines.next().unwrap());
            let right = parse_value(lines.next().unwrap());

            (left, right)
        })
        .collect())
}

fn parse_value(line: &str) -> Value {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::u32;
    use nom::combinator::map;
    use nom::multi::separated_list0;
    use nom::sequence::delimited;
    use nom::IResult;

    fn value(input: &str) -> IResult<&str, Value> {
        alt((
            map(u32, Value::Integer),
            map(
                delimited(tag("["), separated_list0(tag(","), value), tag("]")),
                Value::List,
            ),
        ))(input)
    }

    value(line).unwrap().1
}

fn task_1(pairs: &[(Value, Value)]) -> Result<usize> {
    Ok(pairs
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| compare(left, right) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum())
}

fn task_2(pairs: &[(Value, Value)]) -> Result<usize> {
    let mut all_packets = pairs.iter().fold(Vec::new(), |mut acc, (left, right)| {
        acc.push(left.clone());
        acc.push(right.clone());

        acc
    });

    let divider1 = parse_value("[[2]]");
    let divider2 = parse_value("[[6]]");

    all_packets.push(divider1.clone());
    all_packets.push(divider2.clone());

    all_packets.sort_unstable_by(compare);

    let divider_1_idx = all_packets.iter().position(|v| v == &divider1).unwrap() + 1;
    let divider_2_idx = all_packets.iter().position(|v| v == &divider2).unwrap() + 1;

    Ok(divider_1_idx * divider_2_idx)
}

fn compare(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Value::Integer(left), Value::Integer(right)) => left.cmp(right),
        (Value::List(left), Value::List(right)) => {
            for i in 0..left.len() {
                if i >= right.len() {
                    return Ordering::Greater;
                }

                let cmp = compare(&left[i], &right[i]);

                if cmp != Ordering::Equal {
                    return cmp;
                }
            }

            if left.len() == right.len() {
                return Ordering::Equal;
            }

            Ordering::Less
        }
        (Value::List(..), Value::Integer(..)) => compare(left, &Value::List(vec![right.clone()])),
        (Value::Integer(..), Value::List(..)) => compare(&Value::List(vec![left.clone()]), right),
    }
}

aoc_main!(
    day: 13,
    test_input:
    r#"
    [1,1,3,1,1]
    [1,1,5,1,1]

    [[1],[2,3,4]]
    [[1],4]

    [9]
    [[8,7,6]]

    [[4,4],4,4]
    [[4,4],4,4,4]

    [7,7,7,7]
    [7,7,7]

    []
    [3]

    [[[]]]
    [[]]

    [1,[2,[3,[4,[5,6,7]]]],8,9]
    [1,[2,[3,[4,[5,6,0]]]],8,9]
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 13,
    task_2: task_2,
    expected_2: 140,
);
