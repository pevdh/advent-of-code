use aoc2021::*;

aoc_main!(
    day: 10,
    test_input: r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#,
    parser: parse,
    task_1: task_1,
    expected_1: 26397,
    task_2: task_2,
    expected_2: 288957,
);

fn parse(raw_input: &str) -> Result<String> {
    Ok(raw_input.to_string())
}

fn task_1(inp: &str) -> Result<i64> {
    fn score_line(inp: &str) -> Option<i64> {
        let mut st = VecDeque::new();

        for ch in inp.chars() {
            match ch {
                '['|'<'|'{'|'(' => {
                    st.push_front(ch);
                },
                ']'|'>'|'}'|')' => {
                    let t = st.pop_front();

                    return match (t, ch) {
                        (Some('('), ')') => continue,
                        (Some(_), ')') => Some(3),
                        (Some('['), ']') => continue,
                        (Some(_), ']') => Some(57),
                        (Some('{'), '}') => continue,
                        (Some(_), '}') => Some(1197),
                        (Some('<'), '>') => continue,
                        (Some(_), '>') => Some(25137),
                        _ => continue,
                    };
                },
                _ => unreachable!(),
            }
        }

        None
    }

    let mut total = 0;
    for line in inp.lines() {
        if let Some(sc) = score_line(line) {
            total += sc;
        }
    }

    Ok(total)
}

fn task_2(input: &str) -> Result<i64> {
    fn score_line(inp: &str) -> Option<i64> {
        let mut st = VecDeque::new();

        for ch in inp.chars() {
            match ch {
                '['|'<'|'{'|'(' => {
                    st.push_front(match ch {
                        '(' => ')',
                        '[' => ']',
                        '{' => '}',
                        '<' => '>',
                        _ => unreachable!(),
                    });
                },
                ']'|'>'|'}'|')' => {
                    let t = st.pop_front();

                    if let Some(t_c) = t {
                        if t_c == ch {
                            continue;
                        } else {
                            return None;
                        }
                    }
                },
                _ => unreachable!(),
            }
        }

        if !st.is_empty() {
            let mut sc = 0;

            for &ch in st.iter() {
                sc *= 5;
                sc += match ch {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => unreachable!(),
                };
            }

            return Some(sc)
        }

        None
    }

    let sc: Vec<i64> = input.lines().map(score_line)
        .flatten()
        .sorted()
        .collect();

    Ok(sc[sc.len() / 2])
}

