use aoc2021::*;
use ndarray::{s, ArrayViewMut2};
use nom::sequence::{preceded, separated_pair};

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
    let mut grid = create_grid_from_dots(&input.dots);
    let instruction = input.fold_instructions.first().unwrap();

    fold(grid.view_mut(), instruction);

    let visible_dots = grid.iter().filter(|&v| *v > 0).count();

    Ok(visible_dots)
}

fn task_2(input: &ParsedInput) -> Result<u8> {
    let mut grid = create_grid_from_dots(&input.dots);
    let mut dim = (grid.nrows(), grid.ncols());

    for instruction in &input.fold_instructions {
        dim = fold(grid.slice_mut(s![..(dim.0), ..(dim.1)]), instruction);
    }

    println!();
    print_grid(&grid, &dim);

    Ok(0)
}

fn create_grid_from_dots(dots: &[(usize, usize)]) -> Array2<u32> {
    let max_x = *dots.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *dots.iter().map(|(_, y)| y).max().unwrap();

    let mut grid = Array2::zeros((max_y + 1, max_x + 1));

    for &(x, y) in dots {
        grid[[y, x]] += 1;
    }

    grid
}

fn fold(grid: ArrayViewMut2<u32>, instruction: &FoldInstruction) -> (usize, usize) {
    match instruction {
        FoldInstruction::Up { y } => fold_up(grid, *y),
        FoldInstruction::Left { x } => fold_left(grid, *x),
    }
}

fn fold_up(mut grid: ArrayViewMut2<u32>, fold_y: usize) -> (usize, usize) {
    let new_values: Vec<((usize, usize), u32)> = grid
        .indexed_iter_mut()
        .filter(|((row, _col), value)| *row > fold_y && **value > 0)
        .map(|((row, col), value)| ((fold_y - (row - fold_y), col), value))
        .map(|((row, col), value)| {
            let new_value = ((row, col), *value);
            *value = 0;

            new_value
        })
        .collect();

    for (pos, value) in new_values {
        grid[pos] += value;
    }

    (fold_y, grid.ncols())
}

fn fold_left(mut grid: ArrayViewMut2<u32>, fold_x: usize) -> (usize, usize) {
    let new_values: Vec<((usize, usize), u32)> = grid
        .indexed_iter_mut()
        .filter(|((_row, col), value)| *col > fold_x && **value > 0)
        .map(|((row, col), value)| ((row, fold_x - (col - fold_x)), value))
        .map(|((row, col), value)| {
            let new_value = ((row, col), *value);
            *value = 0;

            new_value
        })
        .collect();

    for (pos, value) in new_values {
        grid[pos] += value;
    }

    (grid.nrows(), fold_x)
}

fn print_grid(grid: &Array2<u32>, dim: &(usize, usize)) {
    for y in 0..dim.0 {
        for x in 0..dim.1 {
            if grid[[y, x]] > 0 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
