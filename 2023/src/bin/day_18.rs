use core::panic;
use std::{collections::HashMap, cmp::{min, max}};

use aoc2023::*;
use rayon::iter::MapInit;

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
    let map = compute_map(input)?;

    // println!();
    // for row in 0..map.nrows() {
    //     for col in 0..map.ncols() {
    //         if map[(row, col)] {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    // println!();

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

    Ok(compute_area_from_instructions(&instructions, &map))
}

fn task_2(input: &str) -> Result<u64> {
    let map = compute_map(input)?;

    let mut instructions = vec![];

    for line in input.lines() {
        let rgb = line.split(' ').skip(2).next().ok_or_parse_error()?;
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

    Ok(compute_area_from_instructions(&instructions, &map))
}

fn compute_area_from_instructions(instructions: &[(char, i64)], map: &Array2<bool>) -> u64 {
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
                    .or_insert(vec![])
                    .push(col_range);

                current_col += dist;

                min_col = min(min_col, current_col);
                max_col = max(max_col, current_col);
            }
            'L' => {
                let col_range = (current_col - dist, current_col);
                col_ranges_by_row
                    .entry(current_row)
                    .or_insert(vec![])
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

    dbg!(min_row, max_row);
    dbg!(min_col, max_col);

    let mut area = 0_u64;
    let mut current_col_ranges: Vec<(i64, i64)> = vec![];

    // #######   7
    // #.....#   14
    // ###...#   21
    // ..#...#   26
    // ..#...#   31
    // ###.###   38
    // #...#..   43
    // ##..###   50
    // .#....#   56
    // .######   62
    for row in min_row..=max_row {
        let area_at_start = area;
        if let Some(col_ranges_in_this_row) = col_ranges_by_row.get(&row) {
            let mut new_col_ranges = current_col_ranges.clone();

            let mut col_ranges_in_this_row = col_ranges_in_this_row.clone();

            while let Some(col_range) = col_ranges_in_this_row.pop() {
                let overlapping_col_ranges = new_col_ranges
                    .iter()
                    .cloned()
                    .filter(|&range| range.overlaps_with(&col_range))
                    .collect::<Vec<_>>();

                let mut has_overlapping = false;
                for overlapping_existing_range in overlapping_col_ranges {
                    new_col_ranges.retain(|&e| e != overlapping_existing_range);
                    has_overlapping = true;

                    if col_range.contains(&overlapping_existing_range) {
                        area += (overlapping_existing_range.1 - overlapping_existing_range.0 + 1)
                            as u64;
                        continue;
                    }

                    let (left, right, range_removed) =
                        overlapping_existing_range.subtract(&col_range);

                    if let Some(left) = left {
                        new_col_ranges.push(left);
                    }

                    if let Some(right) = right {
                        new_col_ranges.push(right);
                    }

                    if let Some(range_removed) = range_removed {
                        area += range_removed as u64;
                    }
                }

                if !has_overlapping {
                    new_col_ranges.push(col_range);
                }
            }

            current_col_ranges = new_col_ranges;
        }

        current_col_ranges = merge(&mut current_col_ranges);

        for col_range in &current_col_ranges {
            area += (col_range.1 - col_range.0 + 1) as u64;
        }

        let area_at_end = area;

        // let mut our_row = Array2::from_elem((1, map.ncols()), '.');

        // for col_range in &current_col_ranges {
        //     // dbg!(col_range);
        //     for i in (col_range.0 - min_col)..(col_range.1 - min_col){
        //         our_row[(0, i as usize)] = '#';
        //     }
        // }

        // for c in 0..map.ncols() {
        //     print!("{}", our_row[(0, c)]);
        // }
        // println!();

        let area_in_this_row = area_at_end - area_at_start;

        // let map_in_row = map.row((row - min_row) as usize).iter().filter(|&e| *e).count() as u64;
        // if area_in_this_row != map_in_row {
        //     eprintln!("error is at row {} here: {} map: {} diff: {}", row - min_row, area_in_this_row, map_in_row, (area_in_this_row as i64 - map_in_row as i64).abs());

        //     println!();
        //     println!();

        //     println!("our row:");
        //     println!("{}", row);

        //     let mut our_row = Array2::from_elem((1, map.ncols()), '.');

        //     for col_range in &current_col_ranges {
        //         for i in (col_range.0-min_col)..=(col_range.1 - min_col) {
        //             our_row[(0, i as usize)] = '#';
        //         }
        //     }

        //     for c in 0..map.ncols() {
        //         print!("{}", our_row[(0, c)]);
        //     }
        //     println!();

        //     println!("their row:");
        //     for c in 0..map.ncols() {
        //         print!("{}", if map[((row as i64 - min_row as i64) as usize, c)] { '#' } else {'.'});
        //     }
        //     println!();

        //     println!("full map until here:");
        //     for r in 0..((row - min_row + 2) as usize) {
        //         for c in 0..map.ncols() {
        //             print!("{}", if map[(r as usize, c)] { '#' } else {'.'});
        //         }
        //         println!();
        //     }
        //     println!();

        //     panic!();
        // }
    }

    area
}

type Range = (i64, i64);

trait RangeExt {
    fn overlaps_with(&self, other: &Range) -> bool;
    fn fully_contains(&self, other: &Range) -> bool;
    fn contains(&self, other: &Range) -> bool;
    fn is_left_adjacent_to(&self, other: &Range) -> bool;
    fn is_right_adjacent_to(&self, other: &Range) -> bool;
    fn subtract(&self, other: &Range) -> (Option<Range>, Option<Range>, Option<i64>);
    fn try_merge(&self, other: &Range) -> Option<Range>;
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

    fn is_left_adjacent_to(&self, other: &Range) -> bool {
        self.1 == other.0
    }

    fn is_right_adjacent_to(&self, other: &Range) -> bool {
        self.0 == other.1
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

        // if self.is_left_adjacent_to(other) {
        //     // ----------
        //     //          ----------
        //     let left = (self.0, self.1 - 1);

        //     return (Some(left), None);
        // }

        // if self.is_right_adjacent_to(other) {
        //     //          ---------
        //     // ----------
        //     let right = (self.0 + 1, self.1);

        //     return (None, Some(right));
        // }

        panic!("do not know how to subtract {:?} from {:?}", other, self);
    }
}

fn merge(ranges: &mut [Range]) -> Vec<Range> {
    if ranges.is_empty() {
        return vec![];
    }

    ranges.sort();

    let mut merged = vec![];

    let mut ranges_it = ranges.iter();

    let mut curr = *ranges_it.next().unwrap();
    
    for next in ranges_it {
        if let Some(merged_range) = curr.try_merge(next) {
            curr = merged_range;
        } else {
            merged.push(curr);
            curr = *next;
        }
    }

    merged.push(curr);

    merged
}

fn compute_map(input: &str) -> Result<Array2<bool>> {
    let mut holes: HashSet<(i64, i64)> = HashSet::default();

    let mut current = (0_i64, 0_i64);
    holes.insert(current);

    for line in input.lines() {
        let mut parts = line.split(' ');

        let dir = parts.next().ok_or_parse_error()?;
        let n = parts
            .next()
            .map(|d| d.parse::<u64>().context("while parsing u64"))
            .unwrap_or(Err(eyre!("parse error")))?;

        let step = match dir {
            "R" => (0, 1),
            "L" => (0, -1),
            "U" => (-1, 0),
            "D" => (1, 0),
            _ => panic!("unexpected direction: {}", dir),
        };

        for _ in 0..n {
            current = (current.0 + step.0, current.1 + step.1);
            holes.insert(current);
        }
    }

    let (min_row, max_row) = match holes.iter().map(|(row, _)| row).minmax() {
        itertools::MinMaxResult::MinMax(min, max) => (*min, *max),
        _ => panic!("no solution"),
    };

    let (min_col, max_col) = match holes.iter().map(|(_, col)| col).minmax() {
        itertools::MinMaxResult::MinMax(min, max) => (*min, *max),
        _ => panic!("no solution"),
    };

    let nrows = (max_row - min_row) as usize + 1;
    let ncols = (max_col - min_col) as usize + 1;

    let mut map = Array2::from_shape_fn((nrows, ncols), |idx| {
        holes
            .get(&(idx.0 as i64 + min_row, idx.1 as i64 + min_col))
            .is_some()
    });


    let mut to_visit: HashSet<(usize, usize)> = HashSet::from_iter(map.indexed_iter().map(|(pos, _)| pos));
    let mut inside: HashSet<(usize, usize)> = HashSet::default();

    while let Some(&(row, col)) = to_visit.iter().next() {
        to_visit.remove(&(row, col));

        if map[(row, col)] {
            continue;
        }

        let flood = |(row, col)| {
            let mut visited = HashSet::default();
            let mut to_visit = vec![(row, col)];
            let mut is_inside = true;
            
            while let Some(current) = to_visit.pop() {
                if visited.contains(&current) {
                    continue;
                }

                let neighbors = map.von_neumann_neighborhood(&current)
                    .filter(|&neighbor| !map[neighbor] && !visited.contains(&neighbor));
                
                to_visit.extend(neighbors);

                visited.insert(current);

                if current.0 == 0 || current.1 == 0 || current.0 == (nrows - 1) || current.1 == (ncols - 1) {
                    is_inside = false;
                }
            }
            
            // println!("{:?}", visited);

            (visited, is_inside)
        };

        let (set, is_inside) = flood((row, col));
        if is_inside {
            inside.extend(set.iter());
        }

        to_visit = to_visit.difference(&set).cloned().collect();
        
    }

    for row in 0..nrows {
        for col in 0..ncols {
            map[(row, col)] = map[(row, col)] || inside.contains(&(row, col))
        }
    }
    
    Ok(map)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let instr = vec![('R', 3), ('D', 3), ('L', 3), ('U', 3)];

        assert_eq!(16, compute_area_from_instructions(&instr));
    }

    #[test]
    fn test2() {
        let instr = vec![
            ('R', 6),
            ('D', 3),
            ('L', 2),
            ('U', 2),
            ('L', 2),
            ('D', 2),
            ('L', 2),
            ('U', 3),
        ];

        assert_eq!(26, compute_area_from_instructions(&instr));
    }

    #[test]
    fn test3() {
        let instr = vec![('R', 4), ('D', 3), ('L', 3), ('U', 2), ('L', 1), ('U', 1)];

        assert_eq!(18, compute_area_from_instructions(&instr));
    }

    #[test]
    fn test4() {
        // ### ###
        // # #####
        // ######
        let instr = vec![('R', 2), ('D', 1), ('R', 2), ('U', 1), ('R', 2), ('D', 1), ('L', 1), ('D', 1), ('L', 5), ('U', 2)];

        assert_eq!(19, compute_area_from_instructions(&instr));
    }
}
