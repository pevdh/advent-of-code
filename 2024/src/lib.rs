mod array;
mod char_grid;
mod ext;
mod mat;
mod mat_ops;
mod parsing;

use std::fmt::{Debug, Display};
use std::io;
use std::io::Write;
use std::time::{Duration, Instant};

use rustc_hash::{FxHashMap, FxHashSet};

pub use std::collections::VecDeque;
pub type HashSet<V> = FxHashSet<V>;
pub type HashMap<K, V> = FxHashMap<K, V>;

pub use array::*;
pub use char_grid::*;
pub use ext::*;
pub use mat::*;
pub use parsing::*;

pub use eyre::eyre;
pub use eyre::Context;
pub use itertools::Itertools;

pub type Result<T, E = eyre::Error> = eyre::Result<T, E>;

pub struct AdventOfCodeSolution<Task1Output, Task1Fn, Task2Output, Task2Fn> {
    pub day: u32,
    pub test_input: &'static str,
    pub task_1: Task1Fn,
    pub expected_1: Task1Output,
    pub test_input_2: &'static str,
    pub task_2: Task2Fn,
    pub expected_2: Task2Output,
}

pub struct TimingInfo {
    pub task_1: Duration,
    pub task_2: Duration,
}

pub fn run<Task1Output, Task1Fn, Task2Output, Task2Fn>(
    input: &str,
    solution: &AdventOfCodeSolution<Task1Output, Task1Fn, Task2Output, Task2Fn>,
) -> Result<TimingInfo>
where
    Task1Output: PartialEq + Debug + Display,
    Task1Fn: Fn(&str) -> Result<Task1Output>,
    Task2Output: PartialEq + Debug + Display,
    Task2Fn: Fn(&str) -> Result<Task2Output>,
{
    let task1_test_output = (solution.task_1)(solution.test_input)
        .wrap_err_with(|| "while running task 1 on test input")?;

    if task1_test_output == solution.expected_1 {
        print!("[TEST OK] ");
    } else {
        print!(
            "[TEST FAILED Expected: {} Got: {}] ",
            solution.expected_1, task1_test_output
        );
    }
    io::stdout().flush().unwrap();

    let task1_start = Instant::now();
    let task1_output =
        (solution.task_1)(input).wrap_err_with(|| "while running task 1 on input")?;
    let task1_time = task1_start.elapsed();
    println!("Task 1: {}", task1_output);

    let task2_test_output = (solution.task_2)(solution.test_input_2)
        .wrap_err_with(|| "while running task 2 on test input")?;

    if task2_test_output == solution.expected_2 {
        print!("[TEST OK] ");
    } else {
        print!(
            "[TEST FAILED Expected: {} Got: {}] ",
            solution.expected_2, task2_test_output
        );
    }
    io::stdout().flush().unwrap();

    let task2_start = Instant::now();
    let task2_output =
        (solution.task_2)(input).wrap_err_with(|| "while running task 2 on input")?;
    let task2_time = task2_start.elapsed();
    println!("Task 2: {}", task2_output);

    Ok(TimingInfo {
        task_1: task1_time,
        task_2: task2_time,
    })
}

#[macro_export]
macro_rules! aoc_main {
    (day: $day:expr, test_input: $test_input:expr, test_input_2: $test_input_2:expr, $($tt:tt)*) => {

        fn main() {
            let input_file_path: &str = concat!("../../input/day_", stringify!($day), ".txt");
            let input: &str = include_str!(concat!("../../input/day_", stringify!($day), ".txt"));

            use indoc::indoc;
            let solution = AdventOfCodeSolution {
                day: $day,
                // Remove leading spaces from test input at compile time
                test_input: indoc! { $test_input },
                test_input_2: indoc! { $test_input_2 },
                $($tt)*
            };

            use std::time::Instant;
            let start = Instant::now();
            color_eyre::install().unwrap();

            let timing_info = run(&input, &solution).unwrap();
            let total = timing_info.task_1 + timing_info.task_2;
            println!("task 1: {:?} | task 2: {:?} | total: {:?}", timing_info.task_1, timing_info.task_2, total)
        }
    };

    (day: $day:expr, test_input: $test_input:expr, $($tt:tt)*) => {
        aoc_main!(day: $day, test_input: $test_input, test_input_2: $test_input, $($tt)*);
    };
}
