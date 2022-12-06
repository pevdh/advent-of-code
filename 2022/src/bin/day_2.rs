use aoc2022::*;

aoc_main!(
    day: 2,
    test_input:
    r#"
    A Y
    B X
    C Z
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 15,
    task_2: task_2,
    expected_2: 12,
);

fn parse(raw_input: &str) -> Result<Vec<(char, char)>> {
    Ok(raw_input
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(a, b)| (a.chars().next().unwrap(), b.chars().next().unwrap()))
                .unwrap()
        })
        .collect())
}

fn task_1(input: &[(char, char)]) -> Result<u64> {
    Ok(input
        .iter()
        .map(|(opponent_c, us_c)| {
            (
                Shape::from_index(*opponent_c as u8 - b'A'),
                Shape::from_index(*us_c as u8 - b'X'),
            )
        })
        .map(|(opponent, us)| shape_score(us) + outcome_score(play(opponent, us)))
        .sum())
}

fn task_2(input: &[(char, char)]) -> Result<u64> {
    Ok(input
        .iter()
        .map(|(opponent_c, desired_outcome_c)| {
            (
                Shape::from_index(*opponent_c as u8 - b'A'),
                Outcome::from_desired_outcome_char(*desired_outcome_c),
            )
        })
        .map(|(opponent, desired_outcome)| {
            let us = shape_for_desired_outcome(opponent, desired_outcome);

            shape_score(us) + outcome_score(desired_outcome)
        })
        .sum())
}

#[derive(Copy, Clone, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_index(b: u8) -> Shape {
        match b {
            0 => Shape::Rock,
            1 => Shape::Paper,
            2 => Shape::Scissors,
            _ => panic!("Invalid index"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn from_desired_outcome_char(c: char) -> Outcome {
        match c {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("Invalid index"),
        }
    }
}

fn play(opponent: Shape, us: Shape) -> Outcome {
    match (opponent, us) {
        (Shape::Rock, Shape::Rock) => Outcome::Draw,
        (Shape::Rock, Shape::Paper) => Outcome::Win,
        (Shape::Rock, Shape::Scissors) => Outcome::Loss,
        (Shape::Paper, Shape::Rock) => Outcome::Loss,
        (Shape::Paper, Shape::Paper) => Outcome::Draw,
        (Shape::Paper, Shape::Scissors) => Outcome::Win,
        (Shape::Scissors, Shape::Rock) => Outcome::Win,
        (Shape::Scissors, Shape::Paper) => Outcome::Loss,
        (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
    }
}

fn shape_score(s: Shape) -> u64 {
    match s {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn outcome_score(outcome: Outcome) -> u64 {
    match outcome {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Loss => 0,
    }
}

fn shape_for_desired_outcome(opponent: Shape, desired_outcome: Outcome) -> Shape {
    match (opponent, desired_outcome) {
        (Shape::Rock, Outcome::Win) => Shape::Paper,
        (Shape::Rock, Outcome::Draw) => Shape::Rock,
        (Shape::Rock, Outcome::Loss) => Shape::Scissors,
        (Shape::Paper, Outcome::Win) => Shape::Scissors,
        (Shape::Paper, Outcome::Draw) => Shape::Paper,
        (Shape::Paper, Outcome::Loss) => Shape::Rock,
        (Shape::Scissors, Outcome::Win) => Shape::Rock,
        (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
        (Shape::Scissors, Outcome::Loss) => Shape::Paper,
    }
}
