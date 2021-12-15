use aoc2021::*;
use nom::sequence::{preceded, separated_pair};
use std::iter::FromIterator;

aoc_main!(
    day: 13,
    test_input: r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#,
    parser: parse,
    task_1: task_1,
    expected_1: 17,
    task_2: task_2,
    expected_2: 0,
);

#[derive(Debug)]
enum FoldInstruction {
    Up { y: usize },
    Left { x: usize },
}

#[derive(Debug)]
struct ParsedInput {
    dots: Vec<(usize, usize)>,
    fold_instructions: Vec<FoldInstruction>,
}

fn parse(raw_input: &str) -> Result<ParsedInput> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, line_ending, u32};
    use nom::combinator::map;
    use nom::multi::{count, separated_list1};

    let usize = |i| map(u32, |n| n as usize)(i);
    let dot = separated_pair(usize, char(','), usize);
    let dots = separated_list1(line_ending, dot);

    let fold_instruction_up = map(preceded(tag("fold along y="), usize), |y| {
        FoldInstruction::Up { y }
    });
    let fold_instruction_left = map(preceded(tag("fold along x="), usize), |x| {
        FoldInstruction::Left { x }
    });

    let fold_instruction = alt((fold_instruction_up, fold_instruction_left));
    let fold_instructions = separated_list1(line_ending, fold_instruction);

    let input = map(
        separated_pair(dots, count(line_ending, 2), fold_instructions),
        |(dots, fold_instructions)| ParsedInput {
            dots,
            fold_instructions,
        },
    );

    nom_parse(raw_input, input)
}

fn task_1(input: &ParsedInput) -> Result<usize> {
    let mut dots = HashSet::from_iter(input.dots.iter().copied());
    let instruction = input.fold_instructions.first().unwrap();

    fold(&mut dots, instruction);

    Ok(dots.len())
}

fn task_2(input: &ParsedInput) -> Result<u8> {
    let mut dots = HashSet::from_iter(input.dots.iter().copied());

    for instruction in &input.fold_instructions {
        fold(&mut dots, instruction);
    }

    println!();
    print_grid(&dots);

    Ok(0)
}

fn fold(dots: &mut HashSet<(usize, usize)>, instruction: &FoldInstruction) {
    match instruction {
        FoldInstruction::Up { y } => fold_up(dots, *y),
        FoldInstruction::Left { x } => fold_left(dots, *x),
    }
}

fn fold_up(dots: &mut HashSet<(usize, usize)>, fold_y: usize) {
    let translated_dots: Vec<((usize, usize), (usize, usize))> = dots
        .iter()
        .filter(|&(_, y)| *y > fold_y)
        .map(|(x, y)| ((*x, *y), (*x, fold_y - (*y - fold_y))))
        .collect();

    for (old_dot, new_dot) in translated_dots {
        dots.remove(&old_dot);
        dots.insert(new_dot);
    }
}

fn fold_left(dots: &mut HashSet<(usize, usize)>, fold_x: usize) {
    let translated_dots: Vec<((usize, usize), (usize, usize))> = dots
        .iter()
        .filter(|&(x, _)| *x > fold_x)
        .map(|(x, y)| ((*x, *y), (fold_x - (*x - fold_x), *y)))
        .collect();

    for (old_dot, new_dot) in translated_dots {
        dots.remove(&old_dot);
        dots.insert(new_dot);
    }
}

fn print_grid(dots: &HashSet<(usize, usize)>) {
    let max_y = *dots.iter().map(|(_, y)| y).max().unwrap();
    let max_x = *dots.iter().map(|(x, _)| x).max().unwrap();

    for y in 0..(max_y + 1) {
        for x in 0..(max_x + 1) {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
