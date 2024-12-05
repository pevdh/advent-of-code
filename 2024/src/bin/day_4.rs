use aoc2024::*;

aoc_main!(
    day: 4,
    test_input: r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#,
    task_1: task_1,
    expected_1: 18,
    task_2: task_2,
    expected_2: 9,
);

fn task_1(input: &str) -> Result<i64> {
    let word_search: Array2<char> = Array2::from_2d_text(input)?;

    #[rustfmt::skip]
    let search_dirs = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1), 
        (1, -1),  (1, 0),  (1, 1),
     ];

    let mut matches = 0;
    for row in 0..word_search.nrows() {
        for col in 0..word_search.ncols() {
            if word_search[(row, col)] != 'X' {
                continue;
            }

            for dir in search_dirs {
                let potential_match = word_search
                    .view()
                    .step_from((row, col), dir)
                    .take("MAS".len());

                if itertools::equal(potential_match, "MAS".chars()) {
                    matches += 1;
                }
            }
        }
    }

    Ok(matches)
}

fn task_2(input: &str) -> Result<i64> {
    let options = [
        // top-left, top-right, bottom-left, bottom-right
        ['M', 'M', 'S', 'S'],
        ['M', 'S', 'M', 'S'],
        ['S', 'M', 'S', 'M'],
        ['S', 'S', 'M', 'M'],
    ];

    let word_search: Array2<char> = Array2::from_2d_text(input)?;

    let mut matches = 0;
    for row in 1..(word_search.nrows() - 1) {
        for col in 1..(word_search.ncols() - 1) {
            if word_search[(row, col)] != 'A' {
                // A is always in the middle
                continue;
            }

            let letters = [
                word_search[(row - 1, col - 1)],
                word_search[(row - 1, col + 1)],
                word_search[(row + 1, col - 1)],
                word_search[(row + 1, col + 1)],
            ];

            if options.contains(&letters) {
                matches += 1;
            }
        }
    }

    Ok(matches)
}
