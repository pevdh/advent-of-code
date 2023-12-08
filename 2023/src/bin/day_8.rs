use aoc2023::*;

aoc_main!(
    day: 8,
    test_input: r#"
    LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)"#,
    task_1: task_1,
    expected_1: 6,
    task_2: task_2,
    expected_2: 6,
);

fn task_1(input: &str) -> Result<u64> {
    let mut lines = input.lines();
    let instructions = lines.next().ok_or_parse_error()?.chars().cycle();

    let nodes: HashMap<String, (String, String)> = lines
        .skip(1)
        .map(|line| {
            let (from, left_right) = line.split_once(" = ").unwrap();

            let left_right = left_right.trim_matches(&['(', ')'] as &[_]);

            let (left, right) = left_right.split_once(", ").unwrap();

            (from.to_string(), (left.to_string(), right.to_string()))
        })
        .collect();

    find_num_steps_to_exit("AAA", &nodes, instructions)
}

fn find_num_steps_to_exit<'a, I>(
    mut current: &'a str,
    nodes: &'a HashMap<String, (String, String)>,
    instructions: I,
) -> Result<u64>
where
    I: Iterator<Item = char>,
{
    for (step, instruction) in (1..).zip(instructions) {
        if instruction == 'L' {
            current = nodes.get(current).unwrap().0.as_str();
        } else {
            current = nodes.get(current).unwrap().1.as_str();
        }

        if current == "ZZZ" {
            return Ok(step);
        }
    }

    Err(eyre!("no solution"))
}

fn task_2(input: &str) -> Result<u64> {
    let mut lines = input.lines();
    let instructions: Vec<char> = lines.next().ok_or_parse_error()?.chars().collect();

    let nodes: HashMap<String, (String, String)> = lines
        .skip(1)
        .map(|line| {
            let (from, left_right) = line.split_once(" = ").unwrap();

            let left_right = left_right.trim_matches(&['(', ')'] as &[_]);

            let (left, right) = left_right.split_once(", ").unwrap();

            (from.to_string(), (left.to_string(), right.to_string()))
        })
        .collect();

    let node_to_idx: HashMap<&str, usize> = nodes
        .keys()
        .enumerate()
        .map(|(idx, node)| (node.as_str(), idx))
        .collect();

    let idx_to_node: HashMap<usize, &str> = node_to_idx.iter().map(|(&n, &i)| (i, n)).collect();

    let mut current_nodes: Vec<State> = nodes
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|s| State {
            node: *node_to_idx.get(s.as_str()).unwrap(),
            steps: 0,
            next_instr_idx: 0,
        })
        .collect();

    let mut jump_table: Vec<Option<Jump>> = vec![None; node_to_idx.len()];

    let mut largest_number_of_steps = 0;

    loop {
        let mut all_same_step = true;
        for i in 0..current_nodes.len() {
            if largest_number_of_steps != 0 && current_nodes[i].steps == largest_number_of_steps {
                continue;
            }

            if let Some(jump) = &jump_table[current_nodes[i].node] {
                current_nodes[i] = State {
                    steps: current_nodes[i].steps + jump.steps_to_exit,
                    next_instr_idx: jump.next_instr_idx,
                    node: jump.exit_node,
                };
            } else {
                let node_str = idx_to_node.get(&current_nodes[i].node).unwrap();
                let jump = find_jump(
                    node_str,
                    current_nodes[i].next_instr_idx,
                    &nodes,
                    &node_to_idx,
                    &instructions,
                );

                let from = current_nodes[i].node;
                current_nodes[i] = State {
                    steps: current_nodes[i].steps + jump.steps_to_exit,
                    next_instr_idx: jump.next_instr_idx,
                    node: jump.exit_node,
                };

                jump_table[from] = Some(jump);
            }

            all_same_step = current_nodes[i].steps == largest_number_of_steps && all_same_step;
            largest_number_of_steps =
                std::cmp::max(largest_number_of_steps, current_nodes[i].steps);
        }

        if all_same_step {
            return Ok(current_nodes[0].steps);
        }
    }
}

fn find_jump<'a>(
    mut current_node: &'a str,
    mut instr_idx: usize,
    nodes: &'a HashMap<String, (String, String)>,
    nodes_to_idx: &HashMap<&'a str, usize>,
    instructions: &[char],
) -> Jump {
    let mut step = 0;

    loop {
        step += 1;
        let instruction = instructions[instr_idx];

        if instruction == 'L' {
            current_node = nodes.get(current_node).unwrap().0.as_str();
        } else {
            current_node = nodes.get(current_node).unwrap().1.as_str();
        }

        if current_node.ends_with('Z') {
            return Jump {
                exit_node: *nodes_to_idx.get(current_node).unwrap(),
                steps_to_exit: step,
                next_instr_idx: (instr_idx + 1) % instructions.len(),
            };
        }

        instr_idx = (instr_idx + 1) % instructions.len();
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct State {
    steps: u64,
    next_instr_idx: usize,
    node: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Jump {
    exit_node: usize,
    steps_to_exit: u64,
    next_instr_idx: usize,
}
