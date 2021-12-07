use ndarray::{Array1, Array2};
use aoc2021::*;

#[derive(Debug, Clone)]
struct ParsedInput {
    drawn_numbers: Array1<i32>,
    boards: Vec<Board>,
}

#[derive(Clone, Debug)]
struct Board {
    numbers: Array2<i32>,
    marked: Array2<i32>,
}

impl Board {
    fn from_2d_vecs(rows: Vec<Vec<i32>>) -> Board {
        let n_rows = rows.len();
        let n_cols = rows.first().map_or(0, |row| row.len());

        let mut data = Vec::with_capacity(n_rows * n_cols);
        for row in rows {
            data.extend_from_slice(&row);
        }

        Board::from_array2(Array2::from_shape_vec((n_rows, n_cols), data).unwrap())
    }

    fn from_array2(board: Array2<i32>) -> Board {
        Board {
            marked: Array2::zeros((board.nrows(), board.ncols())),
            numbers: board,
        }
    }

    fn mark_number(&mut self, number_to_mark: i32) {
        for (idx, &num) in self.numbers.iter().enumerate() {
            if num == number_to_mark {
                self.marked.as_slice_mut().unwrap()[idx] = 1;
            }
        }
    }

    fn wins(&self) -> bool {
        for column in self.marked.columns() {
            if column.sum() == column.len() as i32{
                return true;
            }
        }

        for row in self.marked.rows() {
            if row.sum() == row.len() as i32 {
                return true;
            }
        }

        return false;
    }

    fn score(&self, last_drawn_number: i32) -> i32 {
        let mut sum_of_unmarked_numbers = 0;

        for (idx, &marked) in self.marked.iter().enumerate() {
            if marked == 0 {
                sum_of_unmarked_numbers += self.numbers.as_slice().unwrap()[idx];
            }
        }

        last_drawn_number * sum_of_unmarked_numbers
    }
}

fn parse(raw_input: &str) -> Result<ParsedInput> {
    use nom::character::complete::{char, i32, newline};
    use nom::combinator::{all_consuming, map, opt};
    use nom::multi::{many0, many1, separated_list1};
    use nom::sequence::{preceded, terminated, tuple};

    let drawn_numbers = map(terminated(separated_list1(char(','), i32), newline), |n| Array1::from_vec(n));

    let single_board_number = preceded(many0(char(' ')), i32);

    let board_row = terminated(many1(single_board_number), opt(newline));
    let board = map(many1(board_row), Board::from_2d_vecs);

    let file = all_consuming(tuple((terminated(drawn_numbers, newline), separated_list1(newline, board))));

    let parser = map(file, |(drawn_numbers, boards)| {
        ParsedInput { drawn_numbers, boards }
    });

    nom_parse(raw_input, parser)
}

fn task_1(input: &ParsedInput) -> Result<i32> {
    let mut boards: Vec<Board> = input.boards.clone();

    for &drawn_number in &input.drawn_numbers {
        for board in &mut boards {
            board.mark_number(drawn_number);

            if board.wins() {
                return Ok(board.score(drawn_number));
            }
        }
    }

    Ok(0)
}

fn task_2(input: &ParsedInput) -> Result<i32> {
    let mut boards: Vec<Board> = input.boards.clone();

    for &drawn_number in &input.drawn_numbers {
        assert!(boards.len() > 0);

        let is_last_board = boards.len() == 1;
        for board in &mut boards {
            board.mark_number(drawn_number);

            if is_last_board && board.wins() {
                return Ok(board.score(drawn_number));
            }
        }

        // Remove all winning boards
        boards.retain(|b| !b.wins());
    }

    Ok(0)
}

aoc_main!(
    day: 4,
    test_input: r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#,
    parser: parse,
    task_1: task_1,
    expected_1: 4512,
    task_2: task_2,
    expected_2: 1924,
);
