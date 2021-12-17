use aoc2021::*;
use nom::sequence::tuple;
use std::cmp::max;
use std::ops::RangeInclusive;

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
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

fn parse(raw_input: &str) -> Result<TargetArea> {
    use nom::bytes::complete::tag;
    use nom::character::complete::i32;
    use nom::combinator::map;
    use nom::sequence::separated_pair;

    let range = |i| map(separated_pair(i32, tag(".."), i32), |(from, to)| from..=to)(i);
    let target_area = |i| {
        map(
            tuple((tag("target area: x="), range, tag(", y="), range)),
            |(_, x_range, _, y_range)| TargetArea { x_range, y_range },
        )(i)
    };

    nom_parse(raw_input, target_area)
}

type Position = (i32, i32);
type Velocity = (i32, i32);

fn task_1(target_area: &TargetArea) -> Result<i32> {
    let mut overall_highest_velocity = i32::MIN;

    for initial_x in -1000..1000 {
        for initial_y in -1000..1000 {
            let (hit_target, highest_veliocity) = simulate(&(initial_x, initial_y), target_area);

            if hit_target {
                overall_highest_velocity = max(overall_highest_velocity, highest_veliocity);
            }
        }
    }

    Ok(overall_highest_velocity)
}

fn task_2(target_area: &TargetArea) -> Result<i32> {
    let mut count = 0;

    for initial_x in -1000..1000 {
        for initial_y in -1000..1000 {
            let (hit_target, _) = simulate(&(initial_x, initial_y), target_area);

            if hit_target {
                count += 1;
            }
        }
    }

    Ok(count)
}

fn simulate(initial_velocity: &Velocity, target_area: &TargetArea) -> (bool, i32) {
    let mut velocity = *initial_velocity;
    let mut position = (0, 0);
    let mut highest_y = i32::MIN;

    for _ in 0..10000 {
        let (new_position, new_velocity) = step(&position, &velocity);
        position = new_position;
        velocity = new_velocity;

        if position.1 > highest_y {
            highest_y = position.1;
        }

        if target_area.x_range.contains(&position.0) && target_area.y_range.contains(&position.1) {
            return (true, highest_y);
        }
    }

    (false, highest_y)
}

fn step(pos: &Position, vel: &Velocity) -> (Position, Velocity) {
    let (new_x, new_y) = (pos.0 + vel.0, pos.1 + vel.1);
    let new_vel_x = vel.0 - vel.0.signum();
    let new_vel_y = vel.1 - 1;

    ((new_x, new_y), (new_vel_x, new_vel_y))
}
