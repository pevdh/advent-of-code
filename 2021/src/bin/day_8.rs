use aoc2021::*;
use anyhow::{anyhow, Context};
use itertools::Itertools;
use nom::combinator::map;

aoc_main!(
    day: 8,
    test_input: r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    parser: parse,
    task_1: task_1,
    expected_1: 26,
    task_2: task_2,
    expected_2: 61229,
);

#[derive(Debug)]
struct NoteEntry {
    unique_signal_patterns: Vec<Vec<char>>,
    output_value: Vec<Vec<char>>,
}

fn parse(raw_input: &str) -> Result<Vec<NoteEntry>> {
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, char, line_ending};
    use nom::combinator::{all_consuming, map};
    use nom::multi::separated_list1;
    use nom::sequence::separated_pair;

    let signal_pattern = |i| map(alpha1, |s: &str| s.chars().collect())(i);
    let unique_signal_patterns = separated_list1(char(' '), signal_pattern);
    let output_value = separated_list1(char(' '), signal_pattern);

    let entry = map(separated_pair(unique_signal_patterns, tag(" | "), output_value), |(p, v)| NoteEntry {
        unique_signal_patterns: p,
        output_value: v,
    });

    let entries = all_consuming(separated_list1(line_ending, entry));

    nom_parse(raw_input, entries)
}

fn task_1(entries: &[NoteEntry]) -> Result<i32> {
    let count = entries.iter()
        .flat_map(|e| e.output_value.iter())
        .filter(|output_value| {
            output_value.len() == DIGITS[1].len()
            || output_value.len() == DIGITS[4].len()
            || output_value.len() == DIGITS[7].len()
            || output_value.len() == DIGITS[8].len()
        })
        .count();

    Ok(count as i32)
}

fn task_2(entries: &[NoteEntry]) -> Result<i64> {
    let mut added_output_values = 0;

    for entry in entries {
        let signal_patterns = &entry.unique_signal_patterns;

        let mut mapping: HashMap<char, char> = HashMap::new();

        let one = signal_patterns.iter().find(|s| s.len() == DIGITS[1].len())
            .ok_or(anyhow!("Unable to find one"))?;

        let four = signal_patterns.iter().find(|s| s.len() == DIGITS[4].len())
            .ok_or(anyhow!("Unable to find four"))?;

        let seven = signal_patterns.iter().find(|s| s.len() == DIGITS[7].len())
            .ok_or(anyhow!("Unable to find seven"))?;

        let eight = signal_patterns.iter().find(|s| s.len() == DIGITS[8].len())
            .ok_or(anyhow!("Unable to find eight"))?;

        let a = seven.iter().copied()
            .filter(|&c| !one.iter().any(|&a| a == c))
            .next()
            .ok_or(anyhow!("Unable to find \"a\" position"))?;

        mapping.insert(a, 'a');

        let is_six = |s: &[char]| {
            // Six is the pattern of length 6 that only overlaps one segment with a one
            s.len() == 6 && overlap(s, &one) == 1
        };

        let six = signal_patterns.iter()
            .find(|s| is_six(&s))
            .ok_or(anyhow!("Unable to find six"))?;

        // The segment that overlaps in the six and one is f, the segment that does not overlap is c
        let f = intersection(one, six)
            .first()
            .copied()
            .ok_or(anyhow!("Unable to find f"))?;
        mapping.insert(f, 'f');

        let c = difference(one, six)
            .first()
            .copied()
            .ok_or(anyhow!("Unable to find c"))?;
        mapping.insert(c, 'c');

        let is_zero = |s: &[char]| {
            // Zero is the pattern of length 6 that overlaps with digit 8 for 6 segments
            // and with digit 4 for 3 segments and it's not the six
            s.len() == 6 && overlap(&eight, s) == 6 && overlap(s, &four) == 3 && s != &six[..]
        };

        let zero = signal_patterns.iter()
            .find(|s| is_zero(&s))
            .ok_or(anyhow!("Unable to find zero"))?;

        let d = difference(&eight, &zero)
            .first()
            .copied()
            .ok_or(anyhow!("Unable to find d"))?;

        mapping.insert(d, 'd');

        let is_nine = |s: &[char]| {
            // Nine is the pattern of length 6 that overlaps with digit 8 for 6 segments
            // and with digit 4 for 4 segments and it's not the six
            s.len() == 6 && overlap(&eight, s) == 6 && overlap(s, &four) == 4 && s != &six[..]
        };

        let nine = signal_patterns.iter()
            .find(|s| is_nine(&s))
            .ok_or(anyhow!("Unable to find nine"))?;

        let e = difference(&eight, &nine)
            .first()
            .copied()
            .ok_or(anyhow!("Unable to find e"))?;
        mapping.insert(e, 'e');

        // b is the one segment where zero and four overlap, that we haven't mapped yet
        let b = intersection(&zero, &four).iter()
            .filter(|c| !mapping.keys().contains(c))
            .next()
            .copied()
            .ok_or(anyhow!("Unable to find b"))?;
        mapping.insert(b, 'b');

        // The left over segment is g
        let g = ['a', 'b', 'c', 'd', 'e', 'f', 'g'].iter()
            .filter(|c| !mapping.keys().contains(c))
            .next()
            .copied()
            .ok_or(anyhow!("Unable to find g"))?;
        mapping.insert(g, 'g');

        for (i, output_signal) in entry.output_value.iter().enumerate() {
            let mut mapped_output_signal: Vec<char> = output_signal.iter()
                .map(|c| mapping.get(c).copied().unwrap())
                .collect();
            mapped_output_signal.sort_unstable();

            let digit = segments_to_digit(&mapped_output_signal)? as i64;

            added_output_values += digit * 10i64.pow(((entry.output_value.len() - i - 1) as u32));
        }
    }

    Ok(added_output_values)
}

fn overlap(a: &[char], b: &[char]) -> usize {
    a.iter().copied().filter(|c| b.contains(c)).count()
}

fn intersection(a: &[char], b: &[char]) -> Vec<char> {
    a.iter().copied().filter(|c| b.contains(c)).collect()
}

fn difference(a: &[char], b: &[char]) -> Vec<char> {
    a.iter().copied().filter(|c| !b.contains(c)).collect()
}

const DIGITS: [&str; 10] = [
    "abcefg",  // 0
    "cf",      // 1
    "acdeg",   // 2
    "acdfg",   // 3
    "bcdf",    // 4
    "abdfg",   // 5
    "abdefg",  // 6
    "acf",     // 7
    "abcdefg", // 8
    "abcdfg",  // 9
];

fn segments_to_digit(signals: &[char]) -> Result<usize> {
    let signals: String = signals.iter().collect();
    DIGITS.iter().position(|s| s == &signals)
        .ok_or(anyhow!("Cannot map \"{}\" to a digit because it is invalid", signals))
}
