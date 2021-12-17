use aoc2021::*;
use nom::sequence::tuple;
use std::cmp::min;

aoc_main!(
    day: 17,
    test_input: r#"target area: x=20..30, y=-10..-5"#,
    parser: parse,
    task_1: task_1,
    expected_1: 45,
    task_2: task_2,
    expected_2: 112,
);

struct TargetArea {
    x_range: (i32, i32),
    y_range: (i32, i32),
}

fn parse(raw_input: &str) -> Result<TargetArea> {
    use nom::bytes::complete::tag;
    use nom::character::complete::i32;
    use nom::combinator::map;
    use nom::sequence::separated_pair;

    let range = |i| separated_pair(i32, tag(".."), i32)(i);
    let target_area = |i| {
        map(
            tuple((tag("target area: x="), range, tag(", y="), range)),
            |(_, x_range, _, y_range)| TargetArea { x_range, y_range },
        )(i)
    };

    nom_parse(raw_input, target_area)
}

fn task_1(target_area: &TargetArea) -> Result<i32> {
    let mut overall_highest_y = i32::MIN;

    for (_, initial_y) in simulate_possible_trajectories(target_area) {
        let top = triangular(initial_y);

        if top > overall_highest_y {
            overall_highest_y = top;
        }
    }

    Ok(overall_highest_y)
}

fn task_2(target_area: &TargetArea) -> Result<i32> {
    let mut count = 0;

    for (_, _) in simulate_possible_trajectories(target_area) {
        count += 1;
    }

    Ok(count)
}

fn triangular(n: i32) -> i32 {
    (n * n + n) / 2
}

fn simulate_possible_trajectories(target_area: &TargetArea) -> HashSet<(i32, i32)> {
    let mut x_step_candidates: HashMap<usize, Vec<i32>> = HashMap::new();
    for initial_x in -1000..1000 {
        let new_x_step_candidates =
            find_x_steps_that_cross_area(initial_x, target_area.x_range.0, target_area.x_range.1);

        for step in new_x_step_candidates {
            x_step_candidates
                .entry(step)
                .or_insert_with(Vec::new)
                .push(initial_x);
        }
    }

    let mut y_step_candidates: HashMap<usize, Vec<i32>> = HashMap::new();
    for initial_y in -1000..1000 {
        let new_y_step_candidates =
            find_y_steps_that_cross_area(initial_y, target_area.y_range.0, target_area.y_range.1);

        for step in new_y_step_candidates {
            y_step_candidates
                .entry(step)
                .or_insert_with(Vec::new)
                .push(initial_y);
        }
    }

    let x_steps: HashSet<usize> = x_step_candidates.keys().cloned().collect();
    let y_steps: HashSet<usize> = y_step_candidates.keys().cloned().collect();

    let mut trajectories = HashSet::new();
    for &step in x_steps.intersection(&y_steps) {
        for &initial_x in x_step_candidates.get(&step).unwrap() {
            for &initial_y in y_step_candidates.get(&step).unwrap() {
                trajectories.insert((initial_x, initial_y));
            }
        }
    }

    trajectories
}

fn find_x_steps_that_cross_area(initial_x: i32, x_left: i32, x_right: i32) -> Vec<usize> {
    let mut intersection_steps = Vec::new();

    let crosses_target_area = |x| x_left <= x && x <= x_right;
    let will_never_cross_target_area_again =
        |x_pos, x_vel| (x_vel > 0 && x_pos > x_right) || (x_vel < 0 && x_pos < x_left);

    let mut x_vel = initial_x;
    let mut x_pos = 0;
    for step in 1..min(1000, triangular(initial_x.abs()) as usize) {
        x_pos += x_vel;
        x_vel -= x_vel.signum();

        if crosses_target_area(x_pos) {
            intersection_steps.push(step);
        }

        if will_never_cross_target_area_again(x_pos, x_vel) {
            return intersection_steps;
        }
    }

    intersection_steps
}

fn find_y_steps_that_cross_area(initial_y: i32, y_bottom: i32, y_top: i32) -> Vec<usize> {
    let mut intersection_steps = Vec::new();

    let crosses_target_area = |y| y_bottom <= y && y <= y_top;
    let will_never_cross_target_area_again = |y_pos, y_vel| y_pos < y_bottom && y_vel < 0;

    let mut y_vel = initial_y;
    let mut y_pos = 0;

    for step in 1.. {
        y_pos += y_vel;
        y_vel -= 1;

        if crosses_target_area(y_pos) {
            intersection_steps.push(step);
        }

        if will_never_cross_target_area_again(y_pos, y_vel) {
            return intersection_steps;
        }
    }

    intersection_steps
}
