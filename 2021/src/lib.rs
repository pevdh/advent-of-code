use anyhow::Context;
use nom::error::{convert_error, VerboseError};
use nom::{Err, IResult};
use std::borrow::Borrow;
use std::fmt::{Debug, Display};
use std::time::Instant;

pub use anyhow::anyhow;
pub use itertools::Itertools;
pub use ndarray::{Array2, ArrayView2};
pub use smallvec::smallvec_inline as smallvec;
pub use smallvec::SmallVec;
pub use std::collections::{HashMap, HashSet, VecDeque};

pub type Result<T, E = anyhow::Error> = anyhow::Result<T, E>;

pub fn nom_parse<'a, O>(
    input: &'a str,
    mut parser: impl FnMut(&'a str) -> IResult<&'a str, O, VerboseError<&'a str>>,
) -> Result<O> {
    let nom_parse_result = parser(input);

    nom_parse_result.map(|(_i, o)| o).map_err(|e| match e {
        Err::Error(e) => anyhow!("Parse errors:\n{}", convert_error(input, e)),
        Err::Incomplete(_e) => anyhow!("Parse error: incomplete input"),
        Err::Failure(_e) => anyhow!("Parse error: failure"),
    })
}

pub struct AdventOfCodeSolution<Parser, Task1Output, Task1Fn, Task2Output, Task2Fn> {
    pub day: u32,
    pub test_input: &'static str,
    pub parser: Parser,
    pub task_1: Task1Fn,
    pub expected_1: Task1Output,
    pub task_2: Task2Fn,
    pub expected_2: Task2Output,
}

pub fn run<
    Parser,
    ParserOutput,
    Task1Input,
    Task1Output,
    Task1Fn,
    Task2Input,
    Task2Output,
    Task2Fn,
>(
    input_file_path: &str,
    input: &str,
    solution: AdventOfCodeSolution<Parser, Task1Output, Task1Fn, Task2Output, Task2Fn>,
) -> Result<()>
where
    Parser: Fn(&str) -> Result<ParserOutput>,
    ParserOutput: Borrow<Task1Input> + Borrow<Task2Input>,
    Task1Input: ?Sized,
    Task1Output: PartialEq + Debug + Display,
    Task1Fn: Fn(&Task1Input) -> Result<Task1Output>,
    Task2Input: ?Sized,
    Task2Output: PartialEq + Debug + Display,
    Task2Fn: Fn(&Task2Input) -> Result<Task2Output>,
{
    let parsed_test_input =
        (solution.parser)(solution.test_input).with_context(|| "Error while parsing test input")?;

    let parsed_input = (solution.parser)(input).with_context(|| {
        format!(
            "Error while parsing input (input originated from \"{}\")",
            input_file_path
        )
    })?;

    let task1_test_output = (solution.task_1)(parsed_test_input.borrow())
        .with_context(|| "Error while running task 1 on test input")?;

    if task1_test_output == solution.expected_1 {
        print!("[TEST OK] ");
    } else {
        print!(
            "[TEST FAILED Expected: {} Got: {}] ",
            solution.expected_1, task1_test_output
        );
    }

    let task1_start = Instant::now();
    let task1_output = (solution.task_1)(parsed_input.borrow())
        .with_context(|| "Error while running task 1 on input")?;
    println!("Task 1: {} ({:?})", task1_output, task1_start.elapsed());

    let task2_test_output = (solution.task_2)(parsed_test_input.borrow())
        .with_context(|| "Error while running task 2 on test input")?;

    if task2_test_output == solution.expected_2 {
        print!("[TEST OK] ");
    } else {
        print!(
            "[TEST FAILED Expected: {} Got: {}] ",
            solution.expected_2, task2_test_output
        );
    }

    let task2_start = Instant::now();
    let task2_output = (solution.task_2)(parsed_input.borrow())
        .with_context(|| "While running task 2 on input")?;
    println!("Task 2: {} ({:?})", task2_output, task2_start.elapsed());

    Ok(())
}

#[macro_export]
macro_rules! aoc_main {
    (day: $day:expr, $($tt:tt)*) => {
        fn main() {
            let input_file_path: &str = concat!("../../input/day_", stringify!($day), ".txt");
            let input: &str = include_str!(concat!("../../input/day_", stringify!($day), ".txt"));
            match run(input_file_path, input, AdventOfCodeSolution { day: $day, $($tt)* }) {
                Err(e) => {
                    eprintln!("{:?}", e);
                },
                _ => {},
            }
        }
    }
}
