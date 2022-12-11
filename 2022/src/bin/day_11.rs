use crate::Value::OldValue;
use aoc2022::*;

#[derive(Debug)]
struct MonkeyConfiguration {
    starting_items: Vec<i64>,
    operation: (Value, Op, Value),
    test_divisor: i64,
    true_monkey: usize,
    false_monkey: usize,
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Int(i32),
    OldValue,
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mult,
}

fn parse(raw_input: &str) -> Result<Vec<MonkeyConfiguration>> {
    Ok(raw_input
        .split("\n\n")
        .map(|monkey_config_s: &str| {
            let lines = monkey_config_s.lines().collect::<Vec<_>>();
            // Example:
            //
            // Monkey 0:
            //   Starting items: 79, 98
            //   Operation: new = old * 19
            //   Test: divisible by 23
            //     If true: throw to monkey 2
            //     If false: throw to monkey 3

            let starting_items: Vec<i64> = lines[1]
                .split(", ")
                .map(|i| i.chars().filter(|c| c.is_ascii_digit()).collect::<String>())
                .map(|i32_s| i32_s.parse().unwrap())
                .collect();

            let operation = parse_operation_from_line(lines[2]);

            let test_divisor = lines[3].split(' ').last().unwrap().parse().unwrap();
            let true_monkey = lines[4].split(' ').last().unwrap().parse().unwrap();
            let false_monkey = lines[5].split(' ').last().unwrap().parse().unwrap();

            MonkeyConfiguration {
                starting_items,
                operation,
                test_divisor,
                true_monkey,
                false_monkey,
            }
        })
        .collect())
}

fn parse_operation_from_line(line: &str) -> (Value, Op, Value) {
    let mut operation_s = line.split(' ').skip(5);
    let left_val_s = operation_s.next().unwrap();
    let op_s = operation_s.next().unwrap();
    let right_s = operation_s.next().unwrap();

    (parse_val(left_val_s), parse_op(op_s), parse_val(right_s))
}

fn parse_val(val_s: &str) -> Value {
    match val_s {
        "old" => Value::OldValue,
        num => Value::Int(num.parse().unwrap()),
    }
}

fn parse_op(op_s: &str) -> Op {
    match op_s {
        "*" => Op::Mult,
        "+" => Op::Add,
        _ => panic!("Invalid operation"),
    }
}

struct MonkeyState {
    items: VecDeque<i64>,
    inspect_count: i64,
}

fn task_1(monkey_configs: &[MonkeyConfiguration]) -> Result<i64> {
    let mut monkey_states = monkey_configs
        .iter()
        .map(|config| MonkeyState {
            items: VecDeque::from_iter(config.starting_items.iter().copied()),
            inspect_count: 0,
        })
        .collect::<Vec<_>>();

    for _ in 0..20 {
        simulate_round(&mut monkey_states, monkey_configs);
    }

    let monkey_business_level = monkey_states
        .iter()
        .map(|s| s.inspect_count)
        .sorted()
        .rev()
        .take(2)
        .product();

    Ok(monkey_business_level)
}

fn task_2(monkey_configs: &[MonkeyConfiguration]) -> Result<i64> {
    let mut monkey_states = monkey_configs
        .iter()
        .map(|config| MonkeyState {
            items: VecDeque::from_iter(config.starting_items.iter().copied()),
            inspect_count: 0,
        })
        .collect::<Vec<_>>();

    // All divisors are primes, which means that the least common multiple is simply the product of all divisors.
    let least_common_multiple: i64 = monkey_configs.iter().map(|c| c.test_divisor).product();

    for _ in 0..10000 {
        simulate_round_2(&mut monkey_states, monkey_configs, least_common_multiple);
    }

    let monkey_business_level = monkey_states
        .iter()
        .map(|s| s.inspect_count)
        .sorted()
        .rev()
        .take(2)
        .product();

    Ok(monkey_business_level)
}

fn simulate_round(monkey_states: &mut [MonkeyState], monkey_configs: &[MonkeyConfiguration]) {
    for monkey_idx in 0..monkey_states.len() {
        while let Some(worry_level) = monkey_states[monkey_idx].items.pop_front() {
            monkey_states[monkey_idx].inspect_count += 1;

            let worry_level = apply_operation(worry_level, monkey_configs[monkey_idx].operation);
            let worry_level = worry_level / 3;

            if worry_level % monkey_configs[monkey_idx].test_divisor as i64 == 0 {
                monkey_states[monkey_configs[monkey_idx].true_monkey]
                    .items
                    .push_back(worry_level);
            } else {
                monkey_states[monkey_configs[monkey_idx].false_monkey]
                    .items
                    .push_back(worry_level);
            }
        }
    }
}

fn simulate_round_2(
    monkey_states: &mut [MonkeyState],
    monkey_configs: &[MonkeyConfiguration],
    least_common_multiple: i64,
) {
    for monkey_idx in 0..monkey_states.len() {
        while let Some(worry_level) = monkey_states[monkey_idx].items.pop_front() {
            monkey_states[monkey_idx].inspect_count += 1;

            let worry_level = apply_operation(worry_level, monkey_configs[monkey_idx].operation);
            let worry_level = worry_level % least_common_multiple;

            if worry_level % monkey_configs[monkey_idx].test_divisor as i64 == 0 {
                monkey_states[monkey_configs[monkey_idx].true_monkey]
                    .items
                    .push_back(worry_level);
            } else {
                monkey_states[monkey_configs[monkey_idx].false_monkey]
                    .items
                    .push_back(worry_level);
            }
        }
    }
}

fn apply_operation(worry_level: i64, operation: (Value, Op, Value)) -> i64 {
    let eval_left = match operation.0 {
        Value::Int(v) => v as i64,
        OldValue => worry_level,
    };

    let eval_right = match operation.2 {
        Value::Int(v) => v as i64,
        OldValue => worry_level,
    };

    match operation.1 {
        Op::Add => eval_left + eval_right,
        Op::Mult => eval_left * eval_right,
    }
}

aoc_main!(
    day: 11,
    test_input:
    r#"
    Monkey 0:
      Starting items: 79, 98
      Operation: new = old * 19
      Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3

    Monkey 1:
      Starting items: 54, 65, 75, 74
      Operation: new = old + 6
      Test: divisible by 19
        If true: throw to monkey 2
        If false: throw to monkey 0

    Monkey 2:
      Starting items: 79, 60, 97
      Operation: new = old * old
      Test: divisible by 13
        If true: throw to monkey 1
        If false: throw to monkey 3

    Monkey 3:
      Starting items: 74
      Operation: new = old + 3
      Test: divisible by 17
        If true: throw to monkey 0
        If false: throw to monkey 1
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 10605,
    task_2: task_2,
    expected_2: 2713310158,
);
