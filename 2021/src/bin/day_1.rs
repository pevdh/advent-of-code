use aoc2021::prelude::*;

fn main() {
    let numbers: Vec<i64> = read_integers(1);

    let part1 = count_increased(&numbers);
    println!("{}", part1);

    let part2 = count_increased_sliding_window(&numbers);
    println!("{}", part2);
}

fn count_increased(numbers: &Vec<i64>) -> i32 {
    let mut previous = None;

    let mut increased = 0;
    for current in numbers {
        if let Some(previous) = previous {
            if previous < current {
                increased += 1;
            }
        }

        previous = Some(current);
    }

    increased
}

fn count_increased_sliding_window(numbers: &Vec<i64>) -> i32 {
    let mut increased = 0;

    for i in 3..numbers.len() {
        let previous_window_sum = numbers[i - 1] + numbers[i - 2] + numbers[i - 3];
        let current_window_sum = numbers[i] + numbers[i - 1] + numbers[i - 2];

        if previous_window_sum < current_window_sum {
            increased += 1;
        }
    }

    increased
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_increased_works() {
        assert_eq!(count_increased(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]), 7);
    }

    #[test]
    fn count_increased_sliding_window_works() {
        assert_eq!(count_increased_sliding_window(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]), 5);
    }
}

