use aoc2022::*;

aoc_main!(
    day: 9,
    test_input:
    r#"
    R 4
    U 4
    L 3
    D 1
    R 4
    D 1
    L 5
    R 2
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 13,
    task_2: task_2,
    expected_2: 1,
);

struct Motion {
    direction: char,
    number_of_steps: usize,
}

fn parse(raw_input: &str) -> Result<Vec<Motion>> {
    Ok(raw_input
        .lines()
        .map(|line| {
            let mut spl = line.split(' ');
            let direction = spl.next().unwrap().chars().next().unwrap();
            let number_of_steps = spl.next().unwrap().parse().unwrap();

            Motion {
                direction,
                number_of_steps,
            }
        })
        .collect())
}

fn task_1(input: &[Motion]) -> Result<usize> {
    use std::iter;

    let num_tail_locations = input
        .iter()
        .flat_map(|m| iter::repeat(m.direction).take(m.number_of_steps))
        .scan([(0, 0); 2], |rope, dir| {
            move_rope(dir, rope);

            Some(*rope)
        })
        .map(|rope| rope[1])
        .unique()
        .count();

    Ok(num_tail_locations)
}

fn task_2(input: &[Motion]) -> Result<usize> {
    use std::iter;

    let num_tail_locations = input
        .iter()
        .flat_map(|m| iter::repeat(m.direction).take(m.number_of_steps))
        .scan([(0, 0); 10], |rope, dir| {
            move_rope(dir, rope);

            Some(*rope)
        })
        .map(|rope| rope[8])
        .unique()
        .count();

    Ok(num_tail_locations)
}

fn move_rope<const N: usize>(dir: char, rope: &mut [(i32, i32); N]) {
    match dir {
        'R' => rope[0].0 += 1,
        'L' => rope[0].0 -= 1,
        'U' => rope[0].1 += 1,
        'D' => rope[0].1 -= 1,
        _ => panic!("Invalid direction"),
    }

    for i in 1..N {
        let prev = rope[i - 1];

        let hor_sq_dist = (prev.0 - rope[i].0) * (prev.0 - rope[i].0);
        let vert_sq_dist = (prev.1 - rope[i].1) * (prev.1 - rope[i].1);

        if hor_sq_dist > 1 || vert_sq_dist > 1 {
            rope[i].0 += (prev.0 - rope[i].0).clamp(-1, 1);
            rope[i].1 += (prev.1 - rope[i].1).clamp(-1, 1);
        }
    }
}
