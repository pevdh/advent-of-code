use aoc2020::*;

aoc_main!(
    day: 1,
    test_input:
    r#"
    1721
    979
    366
    299
    675
    1456
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 514579,
    task_2: task_2,
    expected_2: 241861950,
);

fn parse(raw_input: &str) -> Result<Vec<i64>> {
    let mut numbers = Vec::new();

    for line in raw_input.lines() {
        numbers.push(line.parse().unwrap());
    }

    return Ok(numbers);
}

fn task_1(numbers: &Vec<i64>) -> Result<i64> {
    let mut required: HashMap<i64, i64> = HashMap::new();

    for &i in numbers {
        if let Some(b) = required.get(&i) {
            return Ok(b * i);
        }

        required.insert(2020 - i, i);
    }

    return Err(anyhow!("No solution"));
}

fn task_2(numbers: &Vec<i64>) -> Result<i64> {
    let mut required: HashMap<i64, (i64, i64)> = HashMap::new();

    for &i in numbers {
        if let Some((a, b)) = required.get(&i) {
            return Ok(a * b * i);
        }

        for &j in numbers {
            required.insert(2020 - i - j, (i, j));
        }
    }

    return Err(anyhow!("No solution"));
}
