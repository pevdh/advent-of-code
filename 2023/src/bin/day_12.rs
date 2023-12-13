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
    condition_record: Vec<char>,
    groups: Vec<usize>,
}

fn num_possible(condition_record: &[char], groups: &[usize]) -> u64 {
    fn nom_possible_rec(
        memoization_map: &mut HashMap<MemoizationKey, u64>,
        condition_record: &[char],
        groups: &[usize],
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

        let key = MemoizationKey {
            condition_record: condition_record.to_vec(),
            groups: groups.to_vec(),
        };

        if let Some(&num) = memoization_map.get(&key) {
            return num;
        }

        let ch = condition_record[0];

        let possibilities = match ch {
            '.' => nom_possible_rec(memoization_map, &condition_record[1..], groups),
            '#' => {
                // consume the group and one "."
                let group_size = groups[0];
                if group_size > condition_record.len() {
                    return 0;
                }

                if !condition_record[..group_size]
                    .iter()
                    .all(|&c| c == '?' || c == '#')
                {
                    return 0;
                }

                // check if group ends with "." or "?".
                let char_after_group = condition_record.get(group_size).copied();

                let new_condition_record =
                    if char_after_group == Some('.') || char_after_group == Some('?') {
                        &condition_record[group_size + 1..]
                    } else if char_after_group == Some('#') {
                        return 0;
                    } else {
                        // None
                        &condition_record[group_size..]
                    };

                nom_possible_rec(memoization_map, new_condition_record, &groups[1..])
            }
            '?' => {
                let mut condition_record = condition_record.to_vec();
                condition_record[0] = '.';

                let left = nom_possible_rec(memoization_map, &condition_record, groups);

                condition_record[0] = '#';
                let right = nom_possible_rec(memoization_map, &condition_record, groups);

                left + right
            }
            _ => panic!("unknown character: {}", ch),
        };
        // memoize return value
        memoization_map.insert(key, possibilities);

        possibilities
    }

    nom_possible_rec(&mut HashMap::default(), condition_record, groups)
}
