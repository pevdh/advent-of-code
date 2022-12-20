use aoc2022::*;

aoc_main!(
    day: 20,
    test_input:
    r#"
    1
    2
    -3
    3
    -2
    0
    4
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 3,
    task_2: task_2,
    expected_2: 1623178306,
);

fn parse(raw_input: &str) -> Result<Vec<i64>> {
    Ok(raw_input.lines().map(|l| l.parse().unwrap()).collect())
}

fn task_1(numbers: &[i64]) -> Result<i64> {
    let len = numbers.len() as i64;

    let orig_zero = numbers
        .iter()
        .copied()
        .enumerate()
        .find(|&(_idx, n)| n == 0)
        .unwrap();

    let mut reordered = numbers.iter().copied().enumerate().collect::<Vec<_>>();

    for (i, n) in numbers.iter().copied().enumerate() {
        let current_idx = reordered.iter().position(|r| *r == (i, n)).unwrap() as i64;
        let insert_before = (current_idx + n).rem_euclid(len - 1);

        reordered.remove(current_idx as usize);
        if insert_before == 0 {
            reordered.insert(reordered.len(), (i, n));
        } else {
            reordered.insert(insert_before as usize, (i, n));
        }
    }

    let new_zero_idx = reordered.iter().position(|r| *r == orig_zero).unwrap();
    let a = (new_zero_idx + 1000) % (len as usize);
    let b = (new_zero_idx + 2000) % (len as usize);
    let c = (new_zero_idx + 3000) % (len as usize);

    Ok(reordered[a].1 + reordered[b].1 + reordered[c].1)
}

fn task_2(numbers: &[i64]) -> Result<i64> {
    let numbers = numbers.iter().map(|n| *n * 811_589_153).collect::<Vec<_>>();

    let len = numbers.len() as i64;

    let orig_zero = numbers
        .iter()
        .copied()
        .enumerate()
        .find(|&(_idx, n)| n == 0)
        .unwrap();

    let mut reordered = numbers.iter().copied().enumerate().collect::<Vec<_>>();

    for _ in 0..10 {
        for (i, n) in numbers.iter().copied().enumerate() {
            let current_idx = reordered.iter().position(|r| *r == (i, n)).unwrap() as i64;
            let insert_before = (current_idx + n).rem_euclid(len - 1);

            reordered.remove(current_idx as usize);
            if insert_before == 0 {
                reordered.insert(reordered.len(), (i, n));
            } else {
                reordered.insert(insert_before as usize, (i, n));
            }
        }
    }

    let new_zero_idx = reordered.iter().position(|r| *r == orig_zero).unwrap();
    let a = (new_zero_idx + 1000) % (len as usize);
    let b = (new_zero_idx + 2000) % (len as usize);
    let c = (new_zero_idx + 3000) % (len as usize);

    Ok(reordered[a].1 + reordered[b].1 + reordered[c].1)
}
