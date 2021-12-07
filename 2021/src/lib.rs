pub use std::collections::{
    HashMap,
    HashSet
};
use std::fmt::{Debug, Display};
use anyhow::{anyhow, Context};
use nom::{IResult, Err};
use nom::error::{convert_error, VerboseError};

pub type Result<T, E = anyhow::Error> = anyhow::Result<T, E>;

pub fn nom_parse<'a, O>(input: &'a str, mut parser: impl FnMut(&'a str) -> IResult<&'a str, O, VerboseError<&'a str>>) -> Result<O> {
    let nom_parse_result = parser(&input);

    nom_parse_result
        .map(|(_i, o)| o)
        .map_err(|e| match e {
            Err::Error(e) => anyhow!("Parse errors:\n{}", convert_error(input, e)),
            Err::Incomplete(_e) => anyhow!("Parse error: incomplete input"),
            Err::Failure(_e) => anyhow!("Parse error: failure"),
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
    -> Result<()>
    where
        Parser: Fn(&str) -> Result<ParserOutput>,
        Task1Output: PartialEq + Debug + Display,
        Task1Fn: Fn(&ParserOutput) -> Result<Task1Output>,
        Task2Output: PartialEq + Debug + Display,
        Task2Fn: Fn(&ParserOutput) -> Result<Task2Output>,
{
    let input_file_path = format!("input/day_{}.txt", solution.day);
    let input_string = std::fs::read_to_string(&input_file_path)
        .with_context(|| format!("Error while reading input from \"{}\"", &input_file_path))?;

    let parsed_test_input = (solution.parser)(solution.test_input)
        .with_context(|| "Error while parsing test input")?;

    let parsed_input = (solution.parser)(&input_string)
        .with_context(|| format!("Error while parsing input from \"{}\"", &input_file_path))?;

    let task1_test_output = (solution.task_1)(&parsed_test_input)
        .with_context(|| "Error while running task 1 on test input")?;

    if task1_test_output == solution.expected_1 {
        print!("[TEST OK] ");
    } else {
        print!("[TEST FAILED] ");
    }

    let task1_output = (solution.task_1)(&parsed_input)
        .with_context(|| "Error while running task 1 on input")?;
    println!("Task 1: {}", task1_output);

    let task2_test_output = (solution.task_2)(&parsed_test_input)
        .with_context(|| "Error while running task 2 on test input")?;

    if task2_test_output == solution.expected_2 {
        print!("[TEST OK] ");
    } else {
        print!("[TEST FAILED] ");
    }

    let task2_output = (solution.task_2)(&parsed_input)
        .with_context(|| "While running task 2 on input")?;
    println!("Task 2: {}", task2_output);

    Ok(())
}

#[macro_export]
macro_rules! aoc_main {
    ($($tt:tt)*) => {
        fn main() {
            match run(AdventOfCodeSolution { $($tt)* }) {
                Err(e) => {
                    eprintln!("{:?}", e);
                },
                _ => {},
            }
        }
    }
}
