use core::panic;
use std::{
    cmp::{max, min},
    collections::HashMap,
};

use aoc2023::*;

aoc_main!(
    day: 18,
    test_input: r#"
    R 6 (#70c710)
    D 5 (#0dc571)
    L 2 (#5713f0)
    D 2 (#d2c081)
    R 2 (#59c680)
    D 2 (#411b91)
    L 5 (#8ceee2)
    U 2 (#caa173)
    L 1 (#1b58a2)
    U 2 (#caa171)
    R 2 (#7807d2)
    U 3 (#a77fa3)
    L 2 (#015232)
    U 2 (#7a21e3)"#,
    task_1: task_1,
    expected_1: 62,
    task_2: task_2,
    expected_2: 952408144115,
);

fn task_1(input: &str) -> Result<u64> {
    let mut instructions = vec![];

    for line in input.lines() {
        let mut parts = line.split(' ');

        let dir = parts.next().ok_or_parse_error()?;
        let dist = parts
            .next()
            .map(|d| d.parse::<i64>().context("while parsing u64"))
            .unwrap_or(Err(eyre!("parse error")))?;

        instructions.push((dir.chars().next().ok_or_parse_error()?, dist));
    }

    Ok(compute_area_from_instructions(&instructions))
}

fn task_2(input: &str) -> Result<u64> {
    let mut instructions = vec![];

    for line in input.lines() {
        let rgb = line.split(' ').nth(2).ok_or_parse_error()?;
        let rgb = &rgb[2..rgb.len() - 1];

        let dist_str = &rgb[..rgb.len() - 1];
        let dir_str = &rgb[rgb.len() - 1..];

        let dist = i64::from_str_radix(dist_str, 16)?;
        let dir = match dir_str {
            "0" => 'R',
            "1" => 'D',
            "2" => 'L',
            "3" => 'U',
            _ => panic!("unknown direction: {}", dir_str),
        };

        instructions.push((dir, dist));
    }

    Ok(compute_area_from_instructions(&instructions))
}

fn compute_area_from_instructions(instructions: &[(char, i64)]) -> u64 {
    let mut col_ranges_by_row: HashMap<i64, Vec<(i64, i64)>> = HashMap::default();
    let (mut current_row, mut current_col) = (0_i64, 0_i64);

    let mut min_row = i64::MAX;
    let mut max_row = i64::MIN;

    let mut min_col = i64::MAX;
    let mut max_col = i64::MIN;

    for &(dir, dist) in instructions {
        match dir {
            'R' => {
                let col_range = (current_col, current_col + dist);
                col_ranges_by_row
                    .entry(current_row)
                    .or_default()
                    .push(col_range);

                current_col += dist;

                min_col = min(min_col, current_col);
                max_col = max(max_col, current_col);
            }
            'L' => {
                let col_range = (current_col - dist, current_col);
                col_ranges_by_row
                    .entry(current_row)
                    .or_default()
                    .push(col_range);

                current_col -= dist;

                min_col = min(min_col, current_col);
                max_col = max(max_col, current_col);
            }
            'U' => {
                current_row -= dist;

                min_row = min(min_row, current_row);
                max_row = max(max_row, current_row);
            }
            'D' => {
                current_row += dist;

                min_row = min(min_row, current_row);
                max_row = max(max_row, current_row);
            }
            _ => panic!("unknown dir: {}", dir),
        }
    }

    let mut area = 0_u64;
    let mut current_col_ranges: Vec<(i64, i64)> = vec![];

    for row in min_row..=max_row {
        area += update_col_ranges(row, &col_ranges_by_row, &mut current_col_ranges);

        merge(&mut current_col_ranges);

        area += current_col_ranges
            .iter()
            .map(|range| range.area())
            .sum::<u64>();
    }

    area
}

fn update_col_ranges(
    row: i64,
    col_ranges_by_row: &HashMap<i64, Vec<(i64, i64)>>,
    current_col_ranges: &mut Vec<Range>,
) -> u64 {
    let col_ranges_in_this_row = match col_ranges_by_row.get(&row) {
        Some(col_ranges_in_this_row) => col_ranges_in_this_row,
        None => return 0,
    };

    let mut area = 0;

    for col_range in col_ranges_in_this_row {
        let overlapping_col_ranges = current_col_ranges
            .iter()
            .cloned()
            .filter(|range| range.overlaps_with(col_range))
            .collect::<Vec<_>>();

        if overlapping_col_ranges.is_empty() {
            current_col_ranges.push(*col_range);
            continue;
        }

        for overlapping_existing_range in overlapping_col_ranges {
            current_col_ranges.retain(|&e| e != overlapping_existing_range);

            if col_range.contains(&overlapping_existing_range) {
                area += overlapping_existing_range.area();
                continue;
            }

            let (left, right, range_removed) = overlapping_existing_range.subtract(col_range);

            if let Some(left) = left {
                current_col_ranges.push(left);
            }

            if let Some(right) = right {
                current_col_ranges.push(right);
            }

            if let Some(range_removed) = range_removed {
                area += range_removed as u64;
            }
        }
    }

    area
}

type Range = (i64, i64);

trait RangeExt {
    fn overlaps_with(&self, other: &Range) -> bool;
    fn fully_contains(&self, other: &Range) -> bool;
    fn contains(&self, other: &Range) -> bool;
    fn subtract(&self, other: &Range) -> (Option<Range>, Option<Range>, Option<i64>);
    fn try_merge(&self, other: &Range) -> Option<Range>;
    fn area(&self) -> u64;
}

impl RangeExt for Range {
    fn overlaps_with(&self, other: &Range) -> bool {
        self.contains(other)
            || other.contains(self)
            // || self.is_left_adjacent_to(other)
            // || self.is_right_adjacent_to(other)
            || (self.0 < other.0 && other.0 < self.1)
            || (other.0 < self.0 && self.0 < other.1)
    }

    fn fully_contains(&self, other: &Range) -> bool {
        self.0 < other.0 && other.1 < self.1
    }

    fn contains(&self, other: &Range) -> bool {
        self.0 <= other.0 && other.1 <= self.1
    }

    fn try_merge(&self, other: &Range) -> Option<Range> {
        if self.1 == other.0 {
            return Some((self.0, other.1));
        }

        if other.1 == self.0 {
            return Some((other.0, self.1));
        }

        None
    }

    fn subtract(&self, other: &Range) -> (Option<Range>, Option<Range>, Option<i64>) {
        if self.fully_contains(other) {
            // ----------
            //   ------
            let left = (self.0, other.0);
            let right = (other.1, self.1);

            return (Some(left), Some(right), Some(other.1 - other.0 - 1));
        }

        if self == other {
            // -------
            // -------
            return (None, None, Some(other.1 - other.0 + 1));
        }

        if self.0 == other.0 && other.1 < self.1 {
            // ---------
            // -------
            let right = (other.1, self.1);
            return (None, Some(right), Some(other.1 - other.0));
        }

        if self.1 == other.1 && self.0 < other.0 {
            // ---------
            //    ------
            let left = (self.0, other.0);

            return (Some(left), None, Some(other.1 - other.0));
        }

        if self.0 < other.0 && other.0 < self.1 {
            // ----------
            //     ----------
            let left = (self.0, other.0);

            return (Some(left), None, Some(self.1 - other.0));
        }

        if other.0 < self.0 && self.0 < other.1 {
            //     ----------
            // ---------
            let right = (other.1, self.1);

            return (None, Some(right), Some(other.1 - self.0));
        }

        panic!("do not know how to subtract {:?} from {:?}", other, self);
    }

    fn area(&self) -> u64 {
        (self.1 - self.0 + 1) as u64
    }
}

fn merge(ranges: &mut Vec<Range>) {
    ranges.sort();

    let mut idx = 0;

    for cur_idx in 1..ranges.len() {
        if let Some(merged_range) = ranges[idx].try_merge(&ranges[cur_idx]) {
            ranges[idx] = merged_range;
        } else {
            ranges[idx + 1] = ranges[cur_idx];
            idx += 1;
        }
    }

    ranges.truncate(idx + 1);
}
