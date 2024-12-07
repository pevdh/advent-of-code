use aoc2024::*;
use std::cmp::max;

aoc_main!(
    day: 7,
    test_input: r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#,
    task_1: task_1,
    expected_1: 3749,
    task_2: task_2,
    expected_2: 11387,
);

fn task_1(input: &str) -> Result<i64> {
    let mut res = 0;

    for line in input.lines() {
        let nums = parse_nums(line)?;
        let target = nums[0];
        let numbers = &nums[1..];

        if is_valid(target, numbers, false) {
            res += target;
        }
    }

    Ok(res)
}

fn task_2(input: &str) -> Result<i64> {
    let mut res = 0;

    for line in input.lines() {
        let nums = parse_nums(line)?;
        let target = nums[0];
        let numbers = &nums[1..];

        if is_valid(target, numbers, true) {
            res += target;
        }
    }

    Ok(res)
}

fn is_valid(target: i64, numbers: &[i64], concat_op: bool) -> bool {
    fn is_valid_rec(target: i64, numbers: &[i64], curr: i64, concat_op: bool) -> bool {
        if numbers.is_empty() {
            return target == curr;
        }

        is_valid_rec(target, &numbers[1..], curr + numbers[0], concat_op)
            || is_valid_rec(target, &numbers[1..], curr * numbers[0], concat_op)
            || (concat_op
                && is_valid_rec(target, &numbers[1..], combine(curr, numbers[0]), concat_op))
    }

    is_valid_rec(target, &numbers[1..], numbers[0], concat_op)
}

fn combine(lhs: i64, rhs: i64) -> i64 {
    let shift = max(rhs, 1).ilog10() + 1;

    lhs * (10i64.pow(shift)) + rhs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine() {
        assert_eq!(combine(123, 45), 12345);
        assert_eq!(combine(1, 45), 145);
        assert_eq!(combine(1, 1), 11);
        assert_eq!(combine(1, 0), 10);
    }
}
