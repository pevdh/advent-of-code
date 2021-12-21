use std::cmp;
use aoc2021::*;

aoc_main!(
    day: 21,
    test_input: r#"Player 1 starting position: 4
Player 2 starting position: 8"#,
    parser: parse,
    task_1: task_1,
    expected_1: 739785,
    task_2: task_2,
    expected_2: 444356092776315,
);

fn parse(raw_input: &str) -> Result<(usize, usize)> {
    let p1 = raw_input.lines().next().unwrap().split_whitespace().last().unwrap().parse()?;
    let p2 = raw_input.lines().skip(1).next().unwrap().split_whitespace().last().unwrap().parse()?;

    Ok((p1, p2))
}


fn task_1(input: &(usize, usize)) -> Result<usize> {
    struct Die {
        value: usize,
        rolls: usize,
    }

    impl Die {
        fn new() -> Die {
            Die { value: 1 , rolls: 0 }
        }
    }

    impl Iterator for Die {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            let v = self.value;
            self.rolls += 1;

            self.value += 1;
            if self.value > 100 {
                self.value = 1;
            }

            Some(v)
        }
    }

    let mut die = Die::new();

    let mut p1_pos = input.0;
    let mut p1_score = 0usize;

    let mut p2_pos = input.1;
    let mut p2_score = 0usize;

    loop {
        let p1_roll: usize = die.next().unwrap() + die.next().unwrap() + die.next().unwrap();

        p1_pos += p1_roll;

        while p1_pos > 10 {
            p1_pos -= 10;
        }

        p1_score += p1_pos;

        if p1_score >= 1000 {
            return Ok(die.rolls * p2_score);
        }

        let p2_roll: usize = die.next().unwrap() + die.next().unwrap() + die.next().unwrap();

        p2_pos += p2_roll;

        while p2_pos > 10 {
            p2_pos -= 10;
        }

        p2_score += p2_pos;

        if p2_score >= 1000 {
            return Ok(die.rolls * p1_score);
        }
    }
}

fn task_2(input: &(usize, usize)) -> Result<u64> {
    #[derive(Debug, Eq, PartialEq, Hash)]
    struct GameState {
        p1_pos: usize,
        p1_score: u64,
        p2_pos: usize,
        p2_score: u64,
    }

    let mut universe_counts: HashMap<GameState, u64> = HashMap::new();

    universe_counts.insert(GameState {
        p1_pos: input.0 - 1,
        p1_score: 0,
        p2_pos: input.1 - 1,
        p2_score: 0,
    }, 1);

    let mut p1_won = 0u64;
    let mut p2_won = 0u64;

    let rolls = [
        (3, 1),
        (4, 3),
        (5, 6),
        (6, 7),
        (7, 6),
        (8, 3),
        (9, 1),
    ];

    while !universe_counts.is_empty() {
        let mut new_universe_counts = HashMap::new();

        for (game_state, &universe_count) in &universe_counts {
            for (p1_num, p1_times) in rolls {
                let new_p1_pos = (game_state.p1_pos + p1_num) % 10;
                let new_p1_score = game_state.p1_score + (new_p1_pos + 1) as u64;

                if new_p1_score >= 21 {
                    p1_won += universe_count * p1_times;
                    continue;
                }

                for (p2_num, p2_times) in rolls {
                    let new_p2_pos = (game_state.p2_pos + p2_num) % 10;
                    let new_p2_score = game_state.p2_score + (new_p2_pos + 1) as u64;

                    if new_p2_score >= 21 {
                        p2_won += universe_count * p2_times;
                    } else {
                        let new_game_state = GameState {
                            p1_pos: new_p1_pos,
                            p2_pos: new_p2_pos,
                            p1_score: new_p1_score,
                            p2_score: new_p2_score,
                        };

                        *new_universe_counts.entry(new_game_state).or_insert(0) += universe_count * p1_times * p2_times;
                    }
                }
            }
        }

        universe_counts = new_universe_counts;
    }

    Ok(cmp::max(p1_won, p2_won))
}

