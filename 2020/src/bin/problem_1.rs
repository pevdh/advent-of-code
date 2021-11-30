use aoc2020::prelude::*;

fn main() {
    let s = read_to_string("inputs/problem_1.txt")
        .unwrap();

    let numbers: Vec<i64> = s.split("\n")
        .map(|line| line.parse().unwrap())
        .collect();

    let mut required: HashMap<i64, (i64, i64)> = HashMap::new();

    for i in 0..numbers.len() {
        let a = numbers[i];

        if let Some((b, c)) = required.get(&a) {
            println!("{}", a * b * c);
            return;
        }

        for j in i..numbers.len() {
            let b = numbers[j];

            let key = 2020 - a - b;
            required.insert(key, (a, b));
        }
    }
}