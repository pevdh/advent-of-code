use ndarray::{Array, Array2, Axis};
use aoc2021::*;

fn parse(raw_input: &str) -> Result<Array2<u8>> {
    let rows = raw_input.lines().count();
    let columns = raw_input.lines().next().unwrap().len();

    let mut array = Array::zeros((rows, columns));

    let mut i = 0;
    for row in raw_input.lines() {
        let mut j = 0;
        for ch in row.chars() {
            array[[i, j]] = if ch == '1' { 1u8 } else { 0u8 };

            j += 1;
        }

        i += 1;
    }

    Ok(array)
}

fn count_zeros_and_ones<'a, I: IntoIterator<Item = &'a u8>>(v: I) -> (i32, i32) {
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

fn binary_to_u32<'a, I: IntoIterator<Item = &'a u8>>(v: I) -> u32
    where <I as IntoIterator>::IntoIter: DoubleEndedIterator {
    let mut result: u32 = 0;
    let mut i = 0;
    for &el in v.into_iter().rev() {
        result |= (el as u32 & 1u32) << i;
        i += 1;
    }

    result
}

fn task_1(numbers: &Array2<u8>) -> Result<i32> {
    // Most common number (0 or 1) in each column
    let most_common_numbers: Vec<u8> = numbers.columns().into_iter()
        .map(|column| {
            let (zeros, ones) = count_zeros_and_ones(&column);
            if zeros > ones { 0u8 } else { 1u8 }
        })
        .collect();

    let gamma: u32 = binary_to_u32(&most_common_numbers);

    // Least common number (0 or 1) in each column
    let least_common_numbers: Vec<u8> = numbers.columns().into_iter()
        .map(|column| {
            let (zeros, ones) = count_zeros_and_ones(&column);
            if zeros < ones { 0u8 } else { 1u8 }
        })
        .collect();

    let epsilon: u32 = binary_to_u32(&least_common_numbers);

    Ok((gamma * epsilon) as i32)
}

fn task_2(numbers: &Array2<u8>) -> Result<i32> {
    // Find oxygen generator rating
    let mut candidates: Array2<u8> = numbers.clone();
    let mut position: usize = 0;
    let oxygen_generator_rating = loop {
        let column = candidates.column(position);
        let (zeros, ones) = count_zeros_and_ones(&column);

        let filter_number = if zeros > ones {
            0
        } else {
            1
        };

        let indices: Vec<usize> = (0..candidates.nrows())
            .filter(|&i| candidates[[i, position]] != filter_number)
            .collect();

        candidates = candidates.select(Axis(0), &indices);

        assert!(candidates.nrows() > 0);

        if candidates.nrows() == 1 {
            break binary_to_u32(&candidates.row(0))
        }

        position += 1;
    };

    // Find CO2 scrubber rating
    let mut candidates: Array2<u8> = numbers.clone();
    let mut position: usize = 0;
    let co2_scrubber_rating = loop {
        let column = candidates.column(position);
        let (zeros, ones) = count_zeros_and_ones(&column);

        let filter_number = if ones < zeros {
            1
        } else {
            0
        };

        let indices: Vec<usize> = (0..candidates.nrows())
            .filter(|&i| candidates[[i, position]] != filter_number)
            .collect();

        candidates = candidates.select(Axis(0), &indices);

        assert!(candidates.nrows() > 0);

        if candidates.nrows() == 1 {
            break binary_to_u32(&candidates.row(0))
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
