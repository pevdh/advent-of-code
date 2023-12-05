use aoc2023::*;

aoc_main!(
    day: 3,
    test_input: r#"
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598.."#,
    task_1: task_1,
    expected_1: 4361,
    task_2: task_2,
    expected_2: 467835,
);

fn task_1(input: &str) -> Result<i64> {
    let schematic: Array2<char> = Array2::from_2d_text(input)?;

    let mut answer = 0i64;

    for row in 0..schematic.nrows() {
        let mut in_number = false;
        let mut number = 0;
        let mut is_part_number = false;

        for col in 0..schematic.ncols() {
            let ch = schematic[(row, col)];
            match ch {
                '0'..='9' => {
                    if in_number {
                        number =
                            number * 10 + ch.to_digit(10).ok_or(eyre!("Char is not a digit"))?;
                    } else {
                        number = ch.to_digit(10).ok_or(eyre!("Char is not a digit"))?;
                        in_number = true;
                    }
                }
                _ => {
                    if in_number && is_part_number {
                        answer += number as i64;
                    }

                    in_number = false;
                    is_part_number = false;
                }
            }

            if in_number && !is_part_number {
                for surrounding_pos in schematic.moore_neighborhood(&(row, col)) {
                    let ch = schematic[surrounding_pos];
                    if ch != '.' && !ch.is_ascii_digit() {
                        is_part_number = true;
                        break;
                    }
                }
            }
        }

        if in_number && is_part_number {
            answer += number as i64;
        }
    }

    Ok(answer)
}

fn task_2(input: &str) -> Result<u64> {
    let schematic: Array2<char> = Array2::from_2d_text(input)?;

    let mut gear_numbers: HashMap<(usize, usize), Vec<u64>> = HashMap::default();

    for row in 0..schematic.nrows() {
        let mut in_number = false;
        let mut number = 0;
        let mut gears_to_update: HashSet<(usize, usize)> = HashSet::default();

        for col in 0..schematic.ncols() {
            let ch = schematic[(row, col)];
            match ch {
                '0'..='9' => {
                    if in_number {
                        number =
                            number * 10 + ch.to_digit(10).ok_or(eyre!("Char is not a digit"))?;
                    } else {
                        number = ch.to_digit(10).ok_or(eyre!("Char is not a digit"))?;
                        in_number = true;
                    }
                }
                _ => {
                    if in_number {
                        for gear in gears_to_update.iter() {
                            gear_numbers.entry(*gear).or_default().push(number as u64);
                        }
                    }

                    in_number = false;
                    gears_to_update.clear();
                }
            }

            if in_number {
                for surrounding_pos in schematic.moore_neighborhood(&(row, col)) {
                    let ch = schematic[surrounding_pos];
                    if ch == '*' {
                        gears_to_update.insert(surrounding_pos);
                    }
                }
            }
        }
        if in_number {
            for gear in gears_to_update.iter() {
                gear_numbers.entry(*gear).or_default().push(number as u64);
            }
        }
    }

    let answer = gear_numbers
        .iter()
        .filter(|&(_k, v)| v.len() == 2)
        .map(|(_, numbers)| numbers.iter().product::<u64>())
        .sum();

    Ok(answer)
}
