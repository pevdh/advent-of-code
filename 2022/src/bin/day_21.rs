use aoc2022::*;

aoc_main!(
    day: 21,
    test_input:
    r#"
    root: pppw + sjmn
    dbpl: 5
    cczh: sllz + lgvd
    zczc: 2
    ptdq: humn - dvpt
    dvpt: 3
    lfqf: 4
    humn: 5
    ljgn: 2
    sjmn: drzm * dbpl
    sllz: 4
    pppw: cczh / lfqf
    lgvd: ljgn * ptdq
    drzm: hmdt - zczc
    hmdt: 32
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 152,
    task_2: task_2,
    expected_2: 301,
);

fn parse(raw_input: &str) -> Result<Vec<Monkey>> {
    Ok(raw_input.lines().map(parse_monkey).collect())
}

fn parse_monkey(line: &str) -> Monkey {
    use nom::branch::alt;
    use nom::bytes::complete::{tag, take};
    use nom::character::complete::char;
    use nom::character::complete::i64;
    use nom::combinator::map;
    use nom::sequence::tuple;

    let monkey_id = |i| map(take(4usize), |s: &str| s.to_owned())(i);

    let op = |i| {
        alt((
            map(char('*'), |_| Op::Mul),
            map(char('/'), |_| Op::Div),
            map(char('+'), |_| Op::Add),
            map(char('-'), |_| Op::Sub),
        ))(i)
    };

    let binop = |i| {
        map(
            tuple((monkey_id, char(' '), op, char(' '), monkey_id)),
            |(left, _, op, _, right)| Expr::BinOp { left, op, right },
        )(i)
    };
    let num = |i| map(i64, Expr::Num)(i);

    let expr = |i| alt((num, binop))(i);

    let monkey = |i| {
        map(tuple((monkey_id, tag(": "), expr)), |(id, _, expr)| {
            Monkey { id, expr }
        })(i)
    };

    nom_parse(line, monkey).unwrap()
}

type MonkeyId = String;

#[derive(Debug, Clone)]
struct Monkey {
    id: MonkeyId,
    expr: Expr,
}

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Num(i64),
    BinOp {
        left: MonkeyId,
        op: Op,
        right: MonkeyId,
    },
    X,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

fn task_1(monkeys: &[Monkey]) -> Result<i64> {
    let monkeys = HashMap::from_iter(monkeys.iter().cloned().map(|m| (m.id.clone(), m)));

    if let EvalResult::Num(n) = eval(&monkeys, &"root".to_string()) {
        Ok(n)
    } else {
        Err(anyhow!("No solution"))
    }
}

#[derive(Debug, Copy, Clone)]
enum EvalResult {
    Num(i64),
    Unknown,
}

fn eval(monkeys: &HashMap<MonkeyId, Monkey>, monkey_id: &MonkeyId) -> EvalResult {
    match &monkeys[monkey_id].expr {
        Expr::Num(n) => EvalResult::Num(*n),
        Expr::X => EvalResult::Unknown,
        Expr::BinOp { left, op, right } => {
            let left = eval(monkeys, left);
            let right = eval(monkeys, right);

            let (left, right) = match (left, right) {
                (EvalResult::Unknown, _) => return EvalResult::Unknown,
                (_, EvalResult::Unknown) => return EvalResult::Unknown,
                (EvalResult::Num(i), EvalResult::Num(j)) => (i, j),
            };

            match op {
                Op::Add => EvalResult::Num(left + right),
                Op::Sub => EvalResult::Num(left - right),
                Op::Mul => EvalResult::Num(left * right),
                Op::Div => EvalResult::Num(left / right),
            }
        }
    }
}

fn task_2(monkeys: &[Monkey]) -> Result<i64> {
    let mut monkeys: HashMap<String, Monkey> =
        HashMap::from_iter(monkeys.iter().cloned().map(|m| (m.id.clone(), m)));
    monkeys.get_mut(&"humn".to_owned()).unwrap().expr = Expr::X;

    if let Expr::BinOp { left, right, .. } = &monkeys[&"root".to_owned()].expr {
        let x = solve_for_x(&monkeys, left, right);

        return Ok(x);
    }

    Err(anyhow!("No solution"))
}

fn solve_for_x(monkeys: &HashMap<String, Monkey>, left: &MonkeyId, right: &MonkeyId) -> i64 {
    let left_val = eval(monkeys, left);
    let right_val = eval(monkeys, right);

    let (mut unknown, mut target_val) = match (left_val, right_val) {
        (EvalResult::Unknown, EvalResult::Num(n)) => (left, n),
        (EvalResult::Num(n), EvalResult::Unknown) => (right, n),
        _ => panic!("Both sides unknown"),
    };

    loop {
        if let Expr::X = &monkeys[unknown].expr {
            break;
        }

        let (left, op, right) = match &monkeys[unknown].expr {
            Expr::BinOp { left, op, right } => (left, op, right),
            _ => panic!("Expression is not a binop"),
        };

        match (eval(monkeys, left), eval(monkeys, right)) {
            (EvalResult::Unknown, EvalResult::Num(n)) => {
                target_val = match op {
                    Op::Add => target_val - n,
                    Op::Sub => target_val + n,
                    Op::Mul => target_val / n,
                    Op::Div => target_val * n,
                };

                unknown = left;
            }
            (EvalResult::Num(n), EvalResult::Unknown) => {
                target_val = match op {
                    Op::Add => target_val - n,
                    Op::Sub => n - target_val,
                    Op::Mul => target_val / n,
                    Op::Div => n / target_val,
                };

                unknown = right;
            }
            _ => panic!(),
        }
    }

    target_val
}
