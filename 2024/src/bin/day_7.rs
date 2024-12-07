use aoc2024::*;

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
    let equations = input
        .lines()
        .map(|line| parse_nums(line).unwrap())
        .collect_vec();

    let res = equations
        .into_iter()
        .filter(|equation| {
            let target = equation[0];
            let nums = &equation[1..];

            is_valid(target, nums, false)
        })
        .map(|equation| equation[0])
        .sum();

    Ok(res)
}

fn task_2(input: &str) -> Result<i64> {
    let equations = input
        .lines()
        .map(|line| parse_nums(line).unwrap())
        .collect_vec();

    let res = equations
        .iter()
        .filter(|&equation| {
            let target = equation[0];
            let nums = &equation[1..];

            is_valid(target, nums, true)
        })
        .map(|equation| equation[0])
        .sum();

    Ok(res)
}

fn is_valid(target: i64, numbers: &[i64], concat_op: bool) -> bool {
    let numbers_rev = numbers.iter().copied().rev().collect_vec();

    fn is_possible(target: i64, number: i64, rem: &[i64], concat_op: bool) -> bool {
        // is it possible to calculate `target` given a `number` and
        // three possible operations: +, * and ||

        // if we've reached the end of the reversed list of numbers then we
        // should have arrived at the first number in the list
        if rem.is_empty() {
            return target == number;
        }

        // case 1: multiplication
        // ? * number = target
        // ? = target / number
        // this operation is possible if the target is divisible by the number
        let mut possible = false;
        if (target % number) == 0 {
            let new_target = target / number;
            possible = possible || is_possible(new_target, rem[0], &rem[1..], concat_op);
        }

        // case 2: concatenation
        // ? || number = target
        // this operation is possible if target ends with number
        if concat_op && ends_with(target, number) {
            let new_target = rstrip(target, number);
            possible = possible || is_possible(new_target, rem[0], &rem[1..], concat_op);
        }

        // case 3: addition
        // ? + number = target
        // this operation is possible if number is smaller than target
        if number < target {
            let new_target = target - number;
            possible = possible || is_possible(new_target, rem[0], &rem[1..], concat_op);
        }

        possible
    }

    is_possible(target, numbers_rev[0], &numbers_rev[1..], concat_op)
}

fn ends_with(subject: i64, num: i64) -> bool {
    // check if subject ends with num. For example:
    // ends_with(12345, 345) -> true
    let n_digits = num.ilog10() + 1;
    let rem = subject % (10i64.pow(n_digits));

    rem == num
}

fn rstrip(subject: i64, num: i64) -> i64 {
    // strip num from subject:
    // rstrip(12345, 45) -> 123
    let n_digits = num.ilog10() + 1;

    (subject - num) / (10i64.pow(n_digits))
}
