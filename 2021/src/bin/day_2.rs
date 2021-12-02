use aoc2021::prelude::*;

fn main() {
    let course: Vec<String> = read_lines(2);

    let position = calculate_position(&course);
    println!("{}", position);

    let position = calculate_position_2(&course);
    println!("{}", position);
}

fn calculate_position(course: &Vec<String>) -> i64 {
    let mut horizontal_position = 0;
    let mut depth = 0;

    for line in course {
        let mut parts: Vec<&str> = line.split(" ").collect();

        let command = parts[0];
        let value: i64 = parts[1].parse().unwrap();

        match command {
            "forward" => {
                horizontal_position += value;
            },
            "down" => {
                depth += value;
            },
            "up" => {
                depth -= value;
            },
            _ => panic!("Unexpected command: {}", command),
        }
    }

    horizontal_position * depth
}

fn calculate_position_2(course: &Vec<String>) -> i64 {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in course {
        let mut parts: Vec<&str> = line.split(" ").collect();

        let command = parts[0];
        let value: i64 = parts[1].parse().unwrap();

        match command {
            "forward" => {
                horizontal_position += value;
                depth += aim * value;
            },
            "down" => {
                aim += value;
            },
            "up" => {
                aim -= value;
            },
            _ => panic!("Unexpected command: {}", command),
        }
    }

    horizontal_position * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_position_works() {
        let input = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ].into_iter().map(|s| s.to_string()).collect();

        assert_eq!(calculate_position(&input), 150)
    }

    #[test]
    fn calculate_position_2_works() {
        let input = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ].into_iter().map(|s| s.to_string()).collect();

        assert_eq!(calculate_position_2(&input), 900)
    }
}

