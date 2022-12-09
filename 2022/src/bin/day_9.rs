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
        .fold(HashSet::new(), {
            let mut head = (0, 0);
            let mut tail = (0, 0);

            move |mut acc, dir| {
                match dir {
                    'R' => head.0 += 1,
                    'L' => head.0 -= 1,
                    'U' => head.1 += 1,
                    'D' => head.1 -= 1,
                    _ => panic!("Invalid direction"),
                }

                let hor_sq_dist = (head.0 - tail.0) * (head.0 - tail.0);
                let vert_sq_dist = (head.1 - tail.1) * (head.1 - tail.1);

                if hor_sq_dist > 1 || vert_sq_dist > 1 {
                    tail.0 += (head.0 - tail.0).clamp(-1, 1);
                    tail.1 += (head.1 - tail.1).clamp(-1, 1);
                }

                acc.insert(tail);

                acc
            }
        })
        .len();

    Ok(num_tail_locations)
}

fn task_2(input: &[Motion]) -> Result<usize> {
    use std::iter;

    let num_tail_locations = input
        .iter()
        .flat_map(|m| iter::repeat(m.direction).take(m.number_of_steps))
        .fold(HashSet::new(), {
            let mut head = (0, 0);
            let mut tails = [(0, 0); 9];

            move |mut acc, dir| {
                match dir {
                    'R' => head.0 += 1,
                    'L' => head.0 -= 1,
                    'U' => head.1 += 1,
                    'D' => head.1 -= 1,
                    _ => panic!("Invalid direction"),
                }

                update_tail(head, &mut tails);

                acc.insert(tails[8]);

                acc
            }
        })
        .len();

    Ok(num_tail_locations)
}

fn update_tail<const N: usize>(head: (i32, i32), tails: &mut [(i32, i32); N]) {
    for i in 0..N {
        let prev = if i == 0 { head } else { tails[i - 1] };

        let hor_sq_dist = (prev.0 - tails[i].0) * (prev.0 - tails[i].0);
        let vert_sq_dist = (prev.1 - tails[i].1) * (prev.1 - tails[i].1);

        if hor_sq_dist > 1 || vert_sq_dist > 1 {
            tails[i].0 += (prev.0 - tails[i].0).clamp(-1, 1);
            tails[i].1 += (prev.1 - tails[i].1).clamp(-1, 1);
        }
    }
}
