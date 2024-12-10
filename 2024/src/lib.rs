mod array;
mod char_grid;
mod mat;

use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::io;
use std::io::Write;
use std::ops::Index;
use std::time::{Duration, Instant};

use ascii::{AsciiChar, AsciiStr};
use nom::error::{convert_error, VerboseError};
use nom::{Err, IResult};
use num_traits::PrimInt;

use rustc_hash::{FxHashMap, FxHashSet};
pub use std::collections::VecDeque;
use std::sync::LazyLock;
pub type HashSet<V> = FxHashSet<V>;
pub type HashMap<K, V> = FxHashMap<K, V>;
pub use array::*;
pub use char_grid::*;
pub use mat::*;

pub use eyre::eyre;
pub use eyre::Context;
pub use itertools::Itertools;
use regex::Regex;

pub type Result<T, E = eyre::Error> = eyre::Result<T, E>;

pub fn nom_parse<'a, O>(
    input: &'a str,
    mut parser: impl FnMut(&'a str) -> IResult<&'a str, O, VerboseError<&'a str>>,
) -> Result<O> {
    let nom_parse_result = parser(input);

    nom_parse_result.map(|(_i, o)| o).map_err(|e| match e {
        Err::Error(e) => eyre!("Parse errors:\n{}", convert_error(input, e)),
        Err::Incomplete(_e) => eyre!("Parse error: incomplete input"),
        Err::Failure(_e) => eyre!("Parse error: failure"),
    })
}

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

pub trait Frequencies<FreqType: PrimInt>: Iterator {
    fn frequencies(self) -> HashMap<Self::Item, FreqType>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        let mut counts = HashMap::default();
        self.for_each(|item| {
            let entry = counts.entry(item).or_insert_with(FreqType::zero);
            *entry = entry.add(FreqType::one());
        });
        counts
    }
}

impl<It: ?Sized, FreqType: PrimInt> Frequencies<FreqType> for It where It: Iterator {}

pub trait AsciiStrTools {
    fn starts_with(&self, s: &AsciiStr) -> bool;
}

impl AsciiStrTools for AsciiStr {
    fn starts_with(&self, s: &AsciiStr) -> bool {
        if s.len() > self.len() {
            return false;
        }

        for i in 0..s.len() {
            if self[i] != s[i] {
                return false;
            }
        }

        true
    }
}

pub trait AsciiCharTools {
    fn to_digit(&self, radix: u32) -> Option<u32>;
}

impl AsciiCharTools for AsciiChar {
    fn to_digit(&self, radix: u32) -> Option<u32> {
        // From: https://doc.rust-lang.org/src/core/char/methods.rs.html#380
        let b = self.as_byte();
        // If not a digit, a number greater than radix will be created.
        let mut digit = (b as u32).wrapping_sub('0' as u32);
        if radix > 10 {
            assert!(radix <= 36, "to_digit: radix is too high (maximum 36)");
            if digit < 10 {
                return Some(digit);
            }
            // Force the 6th bit to be set to ensure ascii is lower case.
            digit = (b as u32 | 0b10_0000)
                .wrapping_sub('a' as u32)
                .saturating_add(10);
        }
        // FIXME: once then_some is const fn, use it here
        if digit < radix {
            Some(digit)
        } else {
            None
        }
    }
}

pub trait MoreItertools: Iterator {
    #[inline]
    fn take_until<P>(self, predicate: P) -> TakeUntil<Self, P>
    where
        Self: Sized,
    {
        TakeUntil {
            it: self,
            flag: false,
            predicate,
        }
    }
}

impl<I: Iterator> MoreItertools for I {}

pub struct TakeUntil<I, P> {
    it: I,
    predicate: P,
    flag: bool,
}

impl<I: Iterator, P> Iterator for TakeUntil<I, P>
where
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        let x = self.it.next()?;

        if self.flag {
            return None;
        }

        if (self.predicate)(&x) {
            self.flag = true;
        }

        Some(x)
    }
}

pub trait OptionTools<T> {
    fn ok_or_parse_error(self) -> Result<T>;
}

impl<T> OptionTools<T> for Option<T> {
    fn ok_or_parse_error(self) -> Result<T> {
        self.ok_or(eyre!("parse error"))
    }
}

static NUM_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+)").unwrap());

pub fn parse_nums(str: &str) -> Result<Vec<i64>> {
    let nums = NUM_REGEX
        .captures_iter(str)
        .map(|c| c.get(1).unwrap())
        .map(|c| c.as_str().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    Ok(nums)
}

pub fn parse_num_pair(str: &str) -> Result<(i64, i64)> {
    let nums = parse_nums(str)?;
    if nums.len() != 2 {
        return Err(eyre!(
            "expected 2 numbers, but found {} numbers",
            nums.len()
        ));
    }

    Ok((nums[0], nums[1]))
}

pub fn parse_num_triple(str: &str) -> Result<(i64, i64, i64)> {
    let nums = parse_nums(str)?;
    if nums.len() != 3 {
        return Err(eyre!(
            "expected 3 numbers, but found {} numbers",
            nums.len()
        ));
    }

    Ok((nums[0], nums[1], nums[2]))
}

pub fn set_of<T: Copy + PartialEq + Eq + Hash>(v1: &[T]) -> HashSet<T> {
    HashSet::from_iter(v1.iter().copied())
}

#[cfg(test)]
mod tests {
    use super::parse_nums;

    #[test]
    pub fn test_parse_nums() {
        assert_eq!(parse_nums("1 2s3\n4#5").unwrap(), vec![1, 2, 3, 4, 5],);
    }
}
