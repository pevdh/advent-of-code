use aoc2020::prelude::*;

fn is_valid(min_occurrences: usize, max_occurrences: usize, character: char, password: &str) -> bool
{
    let count = password.chars().filter(|&c| c == character).collect::<Vec<char>>().len();

    return count >= min_occurrences && count <= max_occurrences;
}

fn is_valid2(first_idx: usize, second_idx: usize, character: char, password: &str) -> bool
{
    let first_valid = password.chars().nth(first_idx - 1).unwrap() == character;
    let second_valid = password.chars().nth(second_idx - 1).unwrap() == character;
    return first_valid ^ second_valid;
}

fn main() {
    let lines = read_lines("inputs/problem_2.txt");

    let mut valid = 0;
    for line in lines.iter() {
        let parts: Vec<&str> = line.split(" ").collect();

        let occurrences_part: Vec<&str> = parts[0].split("-").collect();
        let min_occurrences = occurrences_part[0].parse::<usize>().unwrap();
        let max_occurrences = occurrences_part[1].parse::<usize>().unwrap();

        let character = parts[1].chars().next().unwrap();

        let password = &parts[2];

        if is_valid2(min_occurrences, max_occurrences, character, password) {
            valid += 1;
        }
    }

    println!("{}", valid);
}