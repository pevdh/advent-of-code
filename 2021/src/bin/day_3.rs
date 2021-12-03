use aoc2021::*;



struct ParsedInput {
    numbers: Vec<Vec<u32>>,
}

fn parse(raw_input: &str) -> ParseResult<ParsedInput> {
    let map_to_u8 = |s: char| match s {
        '1' => 1u32,
        '0' => 0u32,
        _ => panic!("Unexpected char: {}", s),
    };

    let numbers = raw_input
        .split("\n")
        .into_iter()
        .map(|s| s.chars().map(map_to_u8).collect::<Vec<u32>>())
        .collect();

    Ok(("", ParsedInput { numbers }))
}

fn transposed<E: Default + Clone>(m: &Vec<Vec<E>>) -> Vec<Vec<E>> {
    assert!(m.len() > 0);

    let mut transposed = Vec::with_capacity(m[0].len());
    for _ in 0..m[0].len() {
        transposed.push(vec![E::default(); m.len()]);
    }

    for i in 0..m.len() {
        for j in 0..m[0].len() {
            transposed[j][i] = m[i][j].clone();
        }
    }

    transposed
}

fn count_zeros_and_ones(v: &Vec<u32>) -> (i32, i32) {
    let mut zeros = 0;
    let mut ones = 0;

    for &n in v {
        if n == 0 {
            zeros += 1;
        } else {
            ones += 1;
        }
    }

    (zeros, ones)
}

fn binary_vec_to_u32(v: &Vec<u32>) -> u32 {
    let mut result: u32 = 0;
    for i in 0..v.len() {
        result |= v[i] << (v.len() - i - 1);
    }

    result
}

fn task_1(input: &ParsedInput) -> Result<i32> {
    let transposed = transposed(&input.numbers);

    let most_common = |row| {
        let (zeros, ones) = count_zeros_and_ones(row);
        if zeros > ones { 0 } else { 1 }
    };

    let least_common = |row| {
        let (zeros, ones) = count_zeros_and_ones(row);
        if zeros < ones { 0 } else { 1 }
    };

    // Most common number (0 or 1) in each column
    let most_common_numbers: Vec<u32> = transposed.iter()
        .map(most_common)
        .collect();

    let gamma: u32 = binary_vec_to_u32(&most_common_numbers);

    // Least common number (0 or 1) in each column
    let least_common_numbers: Vec<u32> = transposed.iter()
        .map(least_common)
        .collect();

    let epsilon: u32 = binary_vec_to_u32(&least_common_numbers);

    Ok((gamma * epsilon) as i32)
}

fn task_2(input: &ParsedInput) -> Result<i32> {
    // Find oxygen generator rating
    let mut candidates: Vec<Vec<u32>> = input.numbers.iter().map(|r| r.clone()).collect();
    let mut position: usize = 0;
    let oxygen_generator_rating = loop {
        let columns = transposed(&candidates);
        let column = &columns[position];
        let (zeros, ones) = count_zeros_and_ones(column);

        let filter_number = if zeros > ones {
            0
        } else {
            1
        };

        candidates.retain(|row| {
            row[position] == filter_number
        });

        assert!(candidates.len() > 0);

        if candidates.len() == 1 {
            break binary_vec_to_u32(&candidates[0])
        }

        position += 1;
    };

    // Find CO2 scrubber rating
    let mut candidates: Vec<Vec<u32>> = input.numbers.iter().map(|r| r.clone()).collect();
    let mut position: usize = 0;
    let co2_scrubber_rating = loop {
        let columns = transposed(&candidates);
        let column = &columns[position];
        let (zeros, ones) = count_zeros_and_ones(column);

        let filter_number = if ones < zeros {
            1
        } else {
            0
        };

        candidates.retain(|row| {
            row[position] == filter_number
        });

        assert!(candidates.len() > 0);

        if candidates.len() == 1 {
            break binary_vec_to_u32(&candidates[0])
        }

        position += 1;
    };

    Ok((oxygen_generator_rating * co2_scrubber_rating) as i32)
}

aoc_main!(
    day: 3,
    test_input: "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
    parser: parse,
    task_1: task_1,
    expected_1: 198,
    task_2: task_2,
    expected_2: 230,
);
