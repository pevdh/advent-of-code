pub use std::collections::{
    HashMap,
    HashSet
};
use std::fmt::{Debug, Display};
use anyhow::anyhow;
use nom::{IResult, Err};
use nom::error::{convert_error, VerboseError};

pub type Result<T, E = anyhow::Error> = anyhow::Result<T, E>;

pub fn nom_parse<'a, O>(input: &'a str, mut parser: impl FnMut(&'a str) -> IResult<&'a str, O, VerboseError<&'a str>>) -> Result<O> {
    let nom_parse_result = parser(&input);

    nom_parse_result
        .map(|(_i, o)| o)
        .map_err(|e| match e {
            Err::Error(e) => anyhow!("{}", convert_error(input, e)),
            Err::Incomplete(_e) => anyhow!("Parser error: incomplete input"),
            Err::Failure(_e) => anyhow!("Parser error: failure"),
        })
}

pub struct AdventOfCodeSolution<
    Parser,
    Task1Output,
    Task1Fn,
    Task2Output,
    Task2Fn,
> {
    pub day: u32,
    pub test_input: &'static str,
    pub parser: Parser,
    pub task_1: Task1Fn,
    pub expected_1: Task1Output,
    pub task_2: Task2Fn,
    pub expected_2: Task2Output,
}

pub fn run<Parser, ParserOutput, Task1Output, Task1Fn, Task2Output, Task2Fn>(solution: AdventOfCodeSolution<Parser, Task1Output, Task1Fn, Task2Output, Task2Fn>)
    where
        Parser: Fn(&str) -> Result<ParserOutput>,
        Task1Output: PartialEq + Debug + Display,
        Task1Fn: Fn(&ParserOutput) -> Result<Task1Output>,
        Task2Output: PartialEq + Debug + Display,
        Task2Fn: Fn(&ParserOutput) -> Result<Task2Output>,
{
    let input_string = read_to_string(solution.day);

    let parsed_test_input = match (solution.parser)(solution.test_input) {
        Ok(parsed_test_input) => parsed_test_input,
        Err(e) => {
            eprintln!(
                "Errors occurred while parsing test input:\n{}",
                e,
            );
            return;
        }
    };

    let parsed_input = match (solution.parser)(input_string.as_str()) {
        Ok(i) => i,
        Err(e) => {
            eprintln!(
                "Errors occurred while parsing input from inputs/day_{}.txt:\n{}",
                solution.day,
                e,
            );
            return;
        }
    };

    let task1_test_output = (solution.task_1)(&parsed_test_input)
        .expect("Error while running task 1 on test input");

    if task1_test_output == solution.expected_1 {
        print!("[TEST OK] ");
    } else {
        print!("[TEST FAILED] ");
    }

    let task1_output = (solution.task_1)(&parsed_input)
        .expect("Error while running task 1 on input");
    println!("Task 1: {}\n", task1_output);

    let task2_test_output = (solution.task_2)(&parsed_test_input)
        .expect("Error while running task 2 on test input");

    if task2_test_output == solution.expected_2 {
        print!("[TEST OK] ");
    } else {
        print!("[TEST FAILED] ");
    }

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
