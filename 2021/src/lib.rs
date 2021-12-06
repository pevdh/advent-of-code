pub use std::collections::{
    HashMap,
    HashSet
};
use std::fmt::{Debug, Display};
use nom::error::{convert_error, VerboseError};
use nom::Finish;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub type ParseResult<'a, S> = nom::IResult<&'a str, S, VerboseError<&'a str>>;

pub struct AdventOfCodeSolution<
    I,
    O,
    P: Fn(&str) -> ParseResult<I>,
    T1: Fn(&I) -> Result<O>,
    T2: Fn(&I) -> Result<O>,
> {
    pub day: u32,
    pub test_input: &'static str,
    pub parser: P,
    pub task_1: T1,
    pub expected_1: O,
    pub task_2: T2,
    pub expected_2: O,
}

pub fn run<I, O, P, T1, T2>(solution: AdventOfCodeSolution<I, O, P, T1, T2>)
    where
        O: PartialEq + Debug + Display,
        P: Fn(&str) -> ParseResult<I>,
        T1: Fn(&I) -> Result<O>,
        T2: Fn(&I) -> Result<O>,
{
    let input_string = read_to_string(solution.day);

    let parsed_test_input = match (solution.parser)(solution.test_input).finish() {
        Ok((_, parsed_test_input)) => parsed_test_input,
        Err(e) => {
            eprintln!(
                "Errors occurred while parsing test input:\n{}",
                convert_error(solution.test_input, e)
            );
            return;
        }
    };

    let parsed_input = match (solution.parser)(input_string.as_str()).finish() {
        Ok((_, i)) => i,
        Err(e) => {
            eprintln!(
                "Errors occurred while parsing input from inputs/day_{}.txt:\n{}",
                solution.day,
                convert_error(input_string.as_str(), e)
            );
            return;
        }
    };

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
