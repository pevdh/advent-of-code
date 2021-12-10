use aoc2021::*;
use std::iter::FromIterator;

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

enum SyntaxError {
    IllegalCharacter(char),
    Incomplete(Vec<char>),
}

fn find_syntax_error(line: &str) -> Option<SyntaxError> {
    let delimiters: HashMap<char, char> =
        HashMap::from_iter([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    let mut stack = Vec::new();
    for ch in line.chars() {
        let stack_top = stack.last();
        match (stack_top, ch) {
            (_, '[' | '<' | '{' | '(') => {
                stack.push(*delimiters.get(&ch).unwrap());
            }
            (Some(&expected_closing_delimiter), ']' | '>' | '}' | ')') => {
                if expected_closing_delimiter != ch {
                    return Some(SyntaxError::IllegalCharacter(ch));
                }

                stack.pop();
            }
            _ => return Some(SyntaxError::IllegalCharacter(ch)),
        }
    }

    if stack.is_empty() {
        None
    } else {
        Some(SyntaxError::Incomplete(stack.into_iter().rev().collect()))
    }
}

fn task_1(inp: &str) -> Result<i64> {
    let score = inp
        .lines()
        .map(|line| match find_syntax_error(line) {
            Some(SyntaxError::IllegalCharacter(')')) => 3,
            Some(SyntaxError::IllegalCharacter(']')) => 57,
            Some(SyntaxError::IllegalCharacter('}')) => 1197,
            Some(SyntaxError::IllegalCharacter('>')) => 25137,
            _ => 0,
        })
        .sum();

    Ok(score)
}

fn task_2(input: &str) -> Result<i64> {
    let scores: Vec<i64> = input
        .lines()
        .filter_map(|line| match find_syntax_error(line) {
            Some(SyntaxError::Incomplete(missing_chars)) => {
                let line_score = missing_chars.into_iter().fold(0, |acc, ch| {
                    let char_score = match ch {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    };

                    acc * 5 + char_score
                });

                Some(line_score)
            }
            _ => None,
        })
        .sorted()
        .collect();

    Ok(scores[scores.len() / 2])
}
