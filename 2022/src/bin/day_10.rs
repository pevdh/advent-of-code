use aoc2022::*;

enum Instruction {
    Addx { value: i64 },
    Noop,
}

fn parse(raw_input: &str) -> Result<Vec<Instruction>> {
    Ok(raw_input
        .lines()
        .map(|line| {
            if line == "noop" {
                Instruction::Noop
            } else {
                let value = line.split(' ').nth(1).unwrap().parse().unwrap();

                Instruction::Addx { value }
            }
        })
        .collect())
}

fn task_1(instructions: &[Instruction]) -> Result<i64> {
    Ok(simulate_cpu_cycles(instructions)
        .filter(|(cycle, _x)| [20, 60, 100, 140, 180, 220].contains(cycle))
        .take(6)
        .map(|(cycle, x)| (cycle as i64) * x)
        .sum())
}

fn task_2(instructions: &[Instruction]) -> Result<usize> {
    let crt = simulate_cpu_cycles(instructions)
        .take(6 * 40)
        .map(|(cycle, x)| {
            let cycle = cycle as i64;
            let crt_row = (cycle - 1) / 40;
            let crt_col = (cycle - 1) % 40;
            let lit = (crt_col == x - 1) || (crt_col == x) || (crt_col == x + 1);

            (crt_row, crt_col, lit)
        })
        .fold(
            String::with_capacity(6 * 40),
            |mut screen, (_crt_row, crt_col, lit)| {
                screen.push(if lit { '#' } else { '.' });
                if crt_col == 39 {
                    screen.push('\n');
                }

                screen
            },
        );

    println!("\n{}", crt);

    Ok(0)
}

struct ProcessorState<'a> {
    cycle: usize,
    x_value: i64,
    instructions: &'a [Instruction],
    program_counter: usize,
    cycles_left: usize,
}

fn simulate_cpu_cycles(instructions: &[Instruction]) -> impl Iterator<Item = (usize, i64)> + '_ {
    let mut state = ProcessorState {
        cycle: 0,
        x_value: 1,
        program_counter: 0,
        instructions,
        cycles_left: instructions[0].cycles(),
    };

    std::iter::from_fn(move || {
        tick(&mut state);

        Some((state.cycle, state.x_value))
    })
}

fn tick(state: &mut ProcessorState) {
    state.cycle += 1;

    if state.cycles_left == 0 {
        if let Instruction::Addx { value } = state.instructions[state.program_counter] {
            state.x_value += value;
        }

        state.program_counter += 1;
        state.cycles_left = state.instructions[state.program_counter].cycles() - 1;
    } else {
        state.cycles_left -= 1;
    }
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx { .. } => 2,
        }
    }
}

aoc_main!(
    day: 10,
    test_input:
    r#"
    addx 15
    addx -11
    addx 6
    addx -3
    addx 5
    addx -1
    addx -8
    addx 13
    addx 4
    noop
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx -35
    addx 1
    addx 24
    addx -19
    addx 1
    addx 16
    addx -11
    noop
    noop
    addx 21
    addx -15
    noop
    noop
    addx -3
    addx 9
    addx 1
    addx -3
    addx 8
    addx 1
    addx 5
    noop
    noop
    noop
    noop
    noop
    addx -36
    noop
    addx 1
    addx 7
    noop
    noop
    noop
    addx 2
    addx 6
    noop
    noop
    noop
    noop
    noop
    addx 1
    noop
    noop
    addx 7
    addx 1
    noop
    addx -13
    addx 13
    addx 7
    noop
    addx 1
    addx -33
    noop
    noop
    noop
    addx 2
    noop
    noop
    noop
    addx 8
    noop
    addx -1
    addx 2
    addx 1
    noop
    addx 17
    addx -9
    addx 1
    addx 1
    addx -3
    addx 11
    noop
    noop
    addx 1
    noop
    addx 1
    noop
    noop
    addx -13
    addx -19
    addx 1
    addx 3
    addx 26
    addx -30
    addx 12
    addx -1
    addx 3
    addx 1
    noop
    noop
    noop
    addx -9
    addx 18
    addx 1
    addx 2
    noop
    noop
    addx 9
    noop
    noop
    noop
    addx -1
    addx 2
    addx -37
    addx 1
    addx 3
    noop
    addx 15
    addx -21
    addx 22
    addx -6
    addx 1
    noop
    addx 2
    addx 1
    noop
    addx -10
    noop
    noop
    addx 20
    addx 1
    addx 2
    addx 2
    addx -6
    addx -11
    noop
    noop
    noop
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 13140,
    task_2: task_2,
    expected_2: 0,
);
