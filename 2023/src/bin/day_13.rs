use aoc2023::*;

aoc_main!(
    day: 13,
    test_input: r#"
    #.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.

    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#
    "#,
    task_1: task_1,
    expected_1: 405,
    task_2: task_2,
    expected_2: 400,
);

fn task_1(input: &str) -> Result<u64> {
    let answer = input
        .split("\n\n")
        .map(|pattern| {
            let rows: Vec<String> = pattern.lines().map(|l| l.to_string()).collect();
            let columns: Vec<String> = pattern.columns().collect();

            find_reflection(&rows).unwrap_or(0) * 100 + find_reflection(&columns).unwrap_or(0)
        })
        .sum();

    Ok(answer)
}

fn find_reflection(pattern: &[String]) -> Option<u64> {
    for i in 0..pattern.len() {
        if i > 0 && pattern[i - 1] == pattern[i] {
            let num_rows_to_check = std::cmp::min(i, pattern.len() - i);

            let m = (1..num_rows_to_check)
                .map(|j| (&pattern[i + j], &pattern[i - j - 1]))
                .all(|(left, right)| left == right);

            if m {
                return Some(i as u64);
            }
        }
    }

    None
}

fn task_2(input: &str) -> Result<u64> {
    let answer = input
        .split("\n\n")
        .map(|pattern| {
            let rows: Vec<String> = pattern.lines().map(|s| s.to_string()).collect();
            let columns: Vec<String> = pattern.columns().collect();

            let score = find_reflection2(&rows);
            if let Some(score) = score {
                return score * 100;
            }

            let score = find_reflection2(&columns);

            score.unwrap()
        })
        .sum();

    Ok(answer)
}

fn find_reflection2(pattern: &[String]) -> Option<u64> {
    for i in 1..pattern.len() {
        let (eq, smudge_needed_for_equality) =
            almost_equal_if_not_for_that_one_smudge(&pattern[i - 1], &pattern[i]);

        if !eq {
            continue;
        }

        let num_rows_to_check = std::cmp::min(i, pattern.len() - i);
        let mut m = true;

        let mut smudge_used = smudge_needed_for_equality;

        for j in 1..num_rows_to_check {
            let (eq, smudge_needed_for_equality) =
                almost_equal_if_not_for_that_one_smudge(&pattern[i + j], &pattern[i - j - 1]);

            if !eq || (smudge_needed_for_equality && smudge_used) {
                m = false;
                break;
            } else if smudge_needed_for_equality && !smudge_used {
                smudge_used = smudge_needed_for_equality;
            }
        }

        if m && smudge_used {
            return Some(i as u64);
        }
    }

    None
}

fn almost_equal_if_not_for_that_one_smudge(a: &str, b: &str) -> (bool, bool) {
    let mut smudge_used = false;

    for (lch, rch) in a.chars().zip(b.chars()) {
        if lch != rch {
            if smudge_used {
                return (false, false);
            } else {
                smudge_used = true;
                continue;
            }
        }
    }

    (true, smudge_used)
}
