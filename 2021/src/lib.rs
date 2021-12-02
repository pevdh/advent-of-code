pub use std::collections::{
    HashMap,
    HashSet
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub type ParseResult<'a, S> = nom::IResult<&'a str, S>;

pub struct AdventOfCodeSolution<
    I,
    P: Fn(&str) -> ParseResult<I>,
    T1: Fn(&I) -> Result<i32>,
    T2: Fn(&I) -> Result<i32>,
> {
    pub day: u32,
    pub test_input: &'static str,
    pub parser: P,
    pub task_1: T1,
    pub expected_1: i32,
    pub task_2: T2,
    pub expected_2: i32,
}

pub fn run<I, P, T1, T2>(solution: AdventOfCodeSolution<I, P, T1, T2>)
    where
        P: Fn(&str) -> ParseResult<I>,
        T1: Fn(&I) -> Result<i32>,
        T2: Fn(&I) -> Result<i32>,
{
    let input_string = read_to_string(solution.day);

    let (_, parsed_test_input) = (solution.parser)(solution.test_input)
        .expect("Failed to parse test input string");

    let (_, parsed_input) =  (solution.parser)(input_string.as_str())
        .expect("Failed to parse input");

    let task1_test_output = (solution.task_1)(&parsed_test_input)
        .expect("Error while running task 1 on test input");

    assert_eq!(task1_test_output, solution.expected_1);

    let task1_output = (solution.task_1)(&parsed_input)
        .expect("Error while running task 1 on input");

    println!("Task 1: {}", task1_output);

    let task2_test_output = (solution.task_2)(&parsed_test_input)
        .expect("Error while running task 2 on test input");

    assert_eq!(task2_test_output, solution.expected_2);

    let task2_output = (solution.task_2)(&parsed_input)
        .expect("Error while running task 2 on input");

    println!("Task 2: {}", task2_output);
}

pub fn read_to_string(day: u32) -> String
{
    let path = format!("input/day_{}.txt", day);

    return std::fs::read_to_string(path).unwrap()
}

#[macro_export]
macro_rules! aoc_main {
    ($($tt:tt)*) => {
        fn main() {
            run(AdventOfCodeSolution { $($tt)* });
        }
    }
}
