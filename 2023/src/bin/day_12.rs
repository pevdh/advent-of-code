use aoc2023::*;

aoc_main!(
    day: 12,
    test_input: r#"
    ???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1"#,
    task_1: task_1,
    expected_1: 21,
    task_2: task_2,
    expected_2: 525152,
);

fn task_1(input: &str) -> Result<u64> {
    let answer = input
        .lines()
        .map(|line| {
            let (broken_condition_record, groups) = line.split_once(' ').unwrap();
            let broken_condition_record: Vec<char> = broken_condition_record.chars().collect();

            let groups: Vec<usize> = groups
                .split(',')
                .map(|d| d.parse::<usize>().unwrap())
                .collect();

            num_possible(&broken_condition_record, &groups)
        })
        .sum();

    Ok(answer)
}

fn task_2(input: &str) -> Result<u64> {
    let answer = input
        .lines()
        .map(|line| {
            let (broken_condition_record, groups) = line.split_once(' ').unwrap();

            let groups: Vec<usize> = groups
                .split(',')
                .map(|d| d.parse::<usize>().unwrap())
                .collect();

            let unfolded_condition_record: Vec<char> =
                [broken_condition_record; 5].join("?").chars().collect();

            let unfolded_groups: Vec<usize> = groups
                .iter()
                .copied()
                .cycle()
                .take(5 * groups.len())
                .collect();

            num_possible(&unfolded_condition_record, &unfolded_groups)
        })
        .sum();

    Ok(answer)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MemoizationKey {
    cr_len: usize,
    groups_len: usize,
    num_operational: usize,
    num_damaged: usize,
    num_unknown: usize,
    num_left_in_group: usize,
    group_total: usize,
}

fn num_possible(condition_record: &[char], groups: &[usize]) -> u64 {
    #[allow(clippy::too_many_arguments)]
    fn nom_possible_rec(
        hm: &mut HashMap<MemoizationKey, u64>,
        condition_record: &[char],
        groups: &[usize],
        num_operational: usize,
        num_damaged: usize,
        num_unknown: usize,
        num_left_in_group: usize,
        group_total: usize,
    ) -> u64 {
        if condition_record.is_empty() && groups.is_empty() {
            return 1;
        }

        if groups.is_empty() {
            // remaining chars can only be . or ?
            return if condition_record.iter().all(|&c| c == '.' || c == '?') {
                1
            } else {
                0
            };
        }

        if !groups.is_empty() && condition_record.is_empty() {
            return 0;
        }

        let s = MemoizationKey {
            cr_len: condition_record.len(),
            groups_len: groups.len(),
            num_operational,
            num_damaged,
            num_unknown,
            num_left_in_group,
            group_total,
        };

        if let Some(&v) = hm.get(&s) {
            return v;
        }

        let ch = condition_record[0];

        let possibilities = match ch {
            '.' => nom_possible_rec(
                hm,
                &condition_record[1..],
                groups,
                num_operational - 1,
                num_damaged,
                num_unknown,
                num_left_in_group,
                group_total,
            ),
            '#' => {
                // consume the group and one "."
                let group_size = groups[0];
                if group_size > condition_record.len() {
                    return 0;
                }

                let mut num_damaged_in_group = 0;
                let mut num_unknown_in_group = 0;

                for &ch in condition_record.iter().take(group_size) {
                    if ch == '?' {
                        num_unknown_in_group += 1;
                    } else if ch == '#' {
                        num_damaged_in_group += 1;
                    } else {
                        // group cannot contain "."
                        return 0;
                    }
                }

                // check if group ends with "." or "?".
                let char_after_group = condition_record.get(group_size).copied();

                let new_condition_record = if char_after_group == Some('.') {
                    &condition_record[group_size + 1..]
                } else if char_after_group == Some('?') {
                    num_unknown_in_group += 1;

                    &condition_record[group_size + 1..]
                } else if char_after_group == Some('#') {
                    return 0;
                } else {
                    // None
                    &condition_record[group_size..]
                };

                nom_possible_rec(
                    hm,
                    new_condition_record,
                    &groups[1..],
                    num_operational,
                    num_damaged - num_damaged_in_group,
                    num_unknown - num_unknown_in_group,
                    num_left_in_group - group_size,
                    group_total,
                )
            }
            '?' => {
                let mut condition_record = condition_record.to_vec();
                condition_record[0] = '.';

                let left = nom_possible_rec(
                    hm,
                    &condition_record,
                    groups,
                    num_operational + 1,
                    num_damaged,
                    num_unknown - 1,
                    num_left_in_group,
                    group_total,
                );

                condition_record[0] = '#';
                let right = nom_possible_rec(
                    hm,
                    &condition_record,
                    groups,
                    num_operational,
                    num_damaged + 1,
                    num_unknown - 1,
                    num_left_in_group,
                    group_total,
                );

                left + right
            }
            _ => panic!("unknown character: {}", ch),
        };
        // memoize return value
        hm.insert(s, possibilities);

        possibilities
    }

    return nom_possible_rec(
        &mut HashMap::default(),
        condition_record,
        groups,
        condition_record.iter().filter(|&c| *c == '.').count(),
        condition_record.iter().filter(|&c| *c == '#').count(),
        condition_record.iter().filter(|&c| *c == '?').count(),
        groups.iter().sum(),
        groups.iter().sum(),
    );
}
