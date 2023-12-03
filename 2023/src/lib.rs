use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::io;
use std::io::Write;
use std::mem::MaybeUninit;
use std::ops::Range;
use std::time::{Duration, Instant};

use anyhow::Context;
use ascii::{AsciiChar, AsciiStr};
use ndarray::{ArrayBase, Ix2, RawData};
use nom::error::{convert_error, VerboseError};
use nom::{Err, IResult};
use num_traits::PrimInt;

pub use std::collections::VecDeque;

use rustc_hash::{FxHashMap, FxHashSet};
pub type HashSet<V> = FxHashSet<V>;
pub type HashMap<K, V> = FxHashMap<K, V>;

pub use anyhow::anyhow;
pub use itertools::Itertools;
pub use ndarray::{Array2, ArrayView2};

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
        .with_context(|| "Error while running task 1 on test input")?;

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
        (solution.task_1)(input).with_context(|| "Error while running task 1 on input")?;
    let task1_time = task1_start.elapsed();
    println!("Task 1: {}", task1_output);

    let task2_test_output = (solution.task_2)(solution.test_input_2)
        .with_context(|| "Error while running task 2 on test input")?;

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
    let task2_output = (solution.task_2)(input).with_context(|| "While running task 2 on input")?;
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
            match run(&input, &solution) {
                Err(e) => {
                    eprintln!("{:?}", e);
                },
                Ok(timing_info) => {
                    let total = timing_info.task_1 + timing_info.task_2;
                    println!("task 1: {:?} | task 2: {:?} | total: {:?}", timing_info.task_1, timing_info.task_2, total)
                },
            }
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

pub struct Neighborhood {
    neighbors: [MaybeUninit<(usize, usize)>; 8],
    alive: Range<usize>,
}

pub trait Neighbors {
    fn moore_neighborhood(&self, pos: &(usize, usize)) -> Neighborhood;
    fn von_neumann_neighborhood(&self, pos: &(usize, usize)) -> Neighborhood;
}

impl<S: RawData> Neighbors for ArrayBase<S, Ix2> {
    fn moore_neighborhood(&self, pos: &(usize, usize)) -> Neighborhood {
        let mut neighbors = [(0, 0); 8];
        let mut size = 0;

        for rel_pos in [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            let new_pos = (pos.0 as i32 + rel_pos.0, pos.1 as i32 + rel_pos.1);

            let in_bounds = new_pos.0 >= 0
                && new_pos.1 >= 0
                && new_pos.0 < self.nrows() as i32
                && new_pos.1 < self.ncols() as i32;

            if in_bounds {
                neighbors[size] = (new_pos.0 as usize, new_pos.1 as usize);
                size += 1;
            }
        }

        let mut n = Neighborhood {
            neighbors: [unsafe { MaybeUninit::uninit().assume_init() }; 8],
            alive: 0..size,
        };

        let dst_ptr = n.neighbors.as_mut_ptr() as *mut _;
        unsafe { neighbors.as_ptr().copy_to_nonoverlapping(dst_ptr, size) };

        n
    }

    fn von_neumann_neighborhood(&self, pos: &(usize, usize)) -> Neighborhood {
        let mut neighbors = [(0, 0); 4];

        let mut size = 0;
        for rel_pos in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
            let new_pos = (pos.0 as i32 + rel_pos.0, pos.1 as i32 + rel_pos.1);

            let in_bounds = new_pos.0 >= 0
                && new_pos.1 >= 0
                && new_pos.0 < self.nrows() as i32
                && new_pos.1 < self.ncols() as i32;

            if in_bounds {
                neighbors[size] = (new_pos.0 as usize, new_pos.1 as usize);
                size += 1;
            }
        }

        let mut n = Neighborhood {
            neighbors: [unsafe { MaybeUninit::uninit().assume_init() }; 8],
            alive: 0..size,
        };

        let dst_ptr = n.neighbors.as_mut_ptr() as *mut _;
        unsafe { neighbors.as_ptr().copy_to_nonoverlapping(dst_ptr, size) };

        n
    }
}

impl Iterator for Neighborhood {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.alive
            .next()
            .map(|idx| unsafe { self.neighbors.get_unchecked(idx).assume_init() })
    }
}

pub trait FromLines<S: Sized> {
    fn from_2d_text_digits(raw_input: &str) -> Result<S>;
}

impl<T: PrimInt> FromLines<Array2<T>> for Array2<T> {
    fn from_2d_text_digits(raw_input: &str) -> Result<Array2<T>> {
        let cols = raw_input
            .lines()
            .next()
            .map(|l| l.len())
            .ok_or_else(|| anyhow!("Empty input"))?;
        let rows = raw_input.lines().count();

        let data: Result<Vec<T>> = raw_input
            .replace('\n', "")
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map(|d| T::from(d).unwrap())
                    .ok_or_else(|| anyhow!("Unable to convert char to digit"))
            })
            .collect();

        Ok(Array2::from_shape_vec((rows, cols), data?)?)
    }
}

pub trait FromLines2<S: Sized> {
    fn from_2d_text(raw_input: &str) -> Result<S>;
}

impl FromLines2<Array2<char>> for Array2<char> {
    fn from_2d_text(raw_input: &str) -> Result<Array2<char>> {
        let cols = raw_input
            .lines()
            .next()
            .map(|l| l.len())
            .ok_or_else(|| anyhow!("Empty input"))?;
        let rows = raw_input.lines().count();

        let data: Vec<char> = raw_input.replace('\n', "").chars().collect();

        Ok(Array2::from_shape_vec((rows, cols), data)?)
    }
}

pub struct StringColumns<'a> {
    lines: Vec<&'a str>,
    column_index: usize,
    longest_line_len: usize,
    scratch_space: String,
}

impl<'a> Iterator for StringColumns<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.column_index >= self.longest_line_len {
            return None;
        }

        self.scratch_space.clear();

        for line in &self.lines {
            if let Some(c) = line.chars().nth(self.column_index) {
                self.scratch_space.push(c);
            } else {
                self.scratch_space.push('\0');
            }
        }

        self.column_index += 1;

        Some(self.scratch_space.clone())
    }
}

pub trait StringTools<'a> {
    fn columns(&self) -> StringColumns<'a>;
}

impl<'a> StringTools<'a> for &'a str {
    fn columns(&self) -> StringColumns<'a> {
        let lines = self.lines().collect::<Vec<_>>();
        let column_index = 0;

        let longest_line_len = lines.iter().map(|line| line.len()).max().unwrap_or(0);

        StringColumns {
            lines,
            scratch_space: String::with_capacity(longest_line_len),
            column_index,
            longest_line_len,
        }
    }
}

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

pub struct StepIter2<'a, T> {
    view: ArrayView2<'a, T>,
    step: (i32, i32),
    current: (usize, usize),
}

pub trait Array2Tools<'a, T> {
    fn step_from(self, init: (usize, usize), step: (i32, i32)) -> StepIter2<'a, T>;
}

impl<'a, T> Array2Tools<'a, T> for ArrayView2<'a, T> {
    fn step_from(self, init: (usize, usize), step: (i32, i32)) -> StepIter2<'a, T> {
        StepIter2 {
            view: self,
            current: init,
            step,
        }
    }
}

impl<'a, T> Iterator for StepIter2<'a, T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.0 == 0 && self.step.0 < 0 {
            return None;
        }

        if self.current.1 == 0 && self.step.1 < 0 {
            return None;
        }

        self.current.0 = ((self.current.0 as i32) + self.step.0) as usize;
        self.current.1 = ((self.current.1 as i32) + self.step.1) as usize;

        self.view.get((self.current.0, self.current.1)).copied()
    }
}
