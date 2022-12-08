use aoc2022::*;

aoc_main!(
    day: 8,
    test_input:
    r#"
    30373
    25512
    65332
    33549
    35390
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 21,
    task_2: task_2,
    expected_2: 8,
);

fn parse(raw_input: &str) -> Result<Array2<u32>> {
    Array2::from_2d_text(raw_input)
}

fn task_1(input: &Array2<u32>) -> Result<usize> {
    let visible: usize = input
        .indexed_iter()
        .filter(|&((row, col), &tree)| {
            let obscured_from_left = input
                .view()
                .step_from((row, col), (0, -1))
                .any(|t| t >= tree);

            let obscured_from_right = input
                .view()
                .step_from((row, col), (0, 1))
                .any(|t| t >= tree);

            let obscured_from_top = input
                .view()
                .step_from((row, col), (-1, 0))
                .any(|t| t >= tree);

            let obscured_from_bottom = input
                .view()
                .step_from((row, col), (1, 0))
                .any(|t| t >= tree);

            !obscured_from_left
                || !obscured_from_right
                || !obscured_from_top
                || !obscured_from_bottom
        })
        .count();

    Ok(visible)
}

fn task_2(input: &Array2<u32>) -> Result<usize> {
    input
        .indexed_iter()
        .map(|((row, col), _)| calculate_scenic_score(input, row, col))
        .max()
        .ok_or_else(|| anyhow!("No solution"))
}

fn calculate_scenic_score(tree_map: &Array2<u32>, location_row: usize, location_col: usize) -> usize {
    let location_height = tree_map[[location_row, location_col]];

    let right = tree_map
        .view()
        .step_from((location_row, location_col), (0, 1))
        .take_until(|t: &u32| *t >= location_height)
        .count();

    let left = tree_map
        .view()
        .step_from((location_row, location_col), (0, -1))
        .take_until(|t: &u32| *t >= location_height)
        .count();

    let up = tree_map
        .view()
        .step_from((location_row, location_col), (-1, 0))
        .take_until(|t: &u32| *t >= location_height)
        .count();

    let down = tree_map
        .view()
        .step_from((location_row, location_col), (1, 0))
        .take_until(|t: &u32| *t >= location_height)
        .count();

    right * left * up * down
}
