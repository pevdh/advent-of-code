use aoc2022::*;

use std::rc::Rc;

aoc_main!(
    day: 17,
    test_input:
    r#"
    >>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 3068,
    task_2: task_2,
    expected_2: 1514285714288,
);

#[derive(Debug, Copy, Clone)]
enum JetDir {
    PushLeft,
    PushRight,
}

fn parse(raw_input: &str) -> Result<Vec<JetDir>> {
    Ok(raw_input
        .trim()
        .chars()
        .map(|c| match c {
            '<' => JetDir::PushLeft,
            '>' => JetDir::PushRight,
            _ => panic!("Unexpected character in input: {}", c),
        })
        .collect())
}

fn task_1(jet_dirs: &[JetDir]) -> Result<u64> {
    let mut stopped_rocks: HashSet<(u64, u64)> = HashSet::default();
    let mut highest_rock = 0;
    let mut jet_dirs_idx = 0;

    for nth in 0..2022 {
        let mut falling_rock = FallingRocks::spawn(nth, highest_rock);

        loop {
            let jet_dir = jet_dirs[jet_dirs_idx];
            jet_dirs_idx = (jet_dirs_idx + 1) % jet_dirs.len();

            match jet_dir {
                JetDir::PushLeft => falling_rock.try_move_left(&stopped_rocks),
                JetDir::PushRight => falling_rock.try_move_right(&stopped_rocks),
            };

            if !falling_rock.try_move_down(&stopped_rocks) {
                let highest_rock_of_falling_rock = falling_rock.highest_rock();
                stopped_rocks.extend(falling_rock.rocks());

                if highest_rock_of_falling_rock > highest_rock {
                    highest_rock = highest_rock_of_falling_rock;
                }

                break;
            }
        }
    }

    Ok(highest_rock)
}

fn task_2(jet_dirs: &[JetDir]) -> Result<u64> {
    const N: u64 = 10_000;
    const TARGET: u64 = 1_000_000_000_000;
    assert_eq!(TARGET % N, 0);

    let mut cache: Vec<Option<Rc<State>>> = vec![None; jet_dirs.len()];

    let initial_state = State {
        added_height: 0,
        jet_dir_idx: 0,
        highest_rock: 0,
        stopped_rocks: HashSet::default(),
    };

    let start_state = simulate_n_falling_rocks(jet_dirs, &initial_state, N);
    let mut height = start_state.added_height;
    let mut n_dropped = N;

    let mut last_state = Rc::new(start_state);
    while n_dropped < TARGET {
        if let Some(cached_state) = &cache[last_state.jet_dir_idx] {
            last_state = cached_state.clone();
        } else {
            let new_state = Rc::new(simulate_n_falling_rocks(jet_dirs, last_state.as_ref(), N));

            cache[last_state.jet_dir_idx] = Some(new_state.clone());
            last_state = new_state.clone();
        }

        height += last_state.added_height;

        n_dropped += N;
    }

    Ok(height)
}

#[derive(Debug)]
struct State {
    jet_dir_idx: usize,
    stopped_rocks: HashSet<(u64, u64)>,
    highest_rock: u64,
    added_height: u64,
}

fn simulate_n_falling_rocks(jet_dirs: &[JetDir], initial_state: &State, n: u64) -> State {
    let mut stopped_rocks = initial_state.stopped_rocks.clone();
    let mut jet_dir_idx = initial_state.jet_dir_idx;

    let mut new_stopped_rocks = HashSet::default();
    let highest_rock_at_start = initial_state.highest_rock;

    let mut highest_rock = initial_state.highest_rock;

    for nth in 0..n {
        let mut falling_rock = FallingRocks::spawn(nth, highest_rock);

        loop {
            let jet_dir = jet_dirs[jet_dir_idx];
            jet_dir_idx = (jet_dir_idx + 1) % jet_dirs.len();

            match jet_dir {
                JetDir::PushLeft => falling_rock.try_move_left(&stopped_rocks),
                JetDir::PushRight => falling_rock.try_move_right(&stopped_rocks),
            };

            if !falling_rock.try_move_down(&stopped_rocks) {
                let highest_rock_of_falling_rock = falling_rock.highest_rock();
                stopped_rocks.extend(falling_rock.rocks());
                new_stopped_rocks.extend(falling_rock.rocks());

                if highest_rock_of_falling_rock > highest_rock {
                    highest_rock = highest_rock_of_falling_rock;
                }

                break;
            }
        }
    }

    State {
        added_height: highest_rock - highest_rock_at_start,
        jet_dir_idx,
        stopped_rocks: new_stopped_rocks,
        highest_rock,
    }
}

struct FallingRocks {
    bottom_left: (u64, u64),
    units: &'static [(u64, u64)],
}

const FIRST_ROCK: [(u64, u64); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
const SECOND_ROCK: [(u64, u64); 5] = [(1, 0), (0, 1), (1, 1), (1, 2), (2, 1)];
const THIRD_ROCK: [(u64, u64); 5] = [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
const FOURTH_ROCK: [(u64, u64); 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];
const FIFTH_ROCK: [(u64, u64); 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];
const NUM_ROCKS: u64 = 5;

impl FallingRocks {
    fn spawn(nth: u64, highest_rock_or_floor: u64) -> FallingRocks {
        let bottom_left = (2, highest_rock_or_floor + 3);

        match nth % NUM_ROCKS {
            0 => FallingRocks {
                bottom_left,
                units: &FIRST_ROCK,
            },
            1 => FallingRocks {
                bottom_left,
                units: &SECOND_ROCK,
            },
            2 => FallingRocks {
                bottom_left,
                units: &THIRD_ROCK,
            },
            3 => FallingRocks {
                bottom_left,
                units: &FOURTH_ROCK,
            },
            4 => FallingRocks {
                bottom_left,
                units: &FIFTH_ROCK,
            },
            _ => panic!("Invalid rock number"),
        }
    }

    fn try_move_down(&mut self, stopped_rocks: &HashSet<(u64, u64)>) -> bool {
        if self.bottom_left.1 == 0 {
            return false;
        }

        let new_bottom_left = (self.bottom_left.0, self.bottom_left.1 - 1);

        for translated in self.translated_units(new_bottom_left) {
            if stopped_rocks.contains(&translated) {
                return false;
            }
        }

        self.bottom_left = new_bottom_left;

        true
    }

    fn try_move_left(&mut self, stopped_rocks: &HashSet<(u64, u64)>) -> bool {
        if self.bottom_left.0 == 0 {
            return false;
        }

        let new_bottom_left = (self.bottom_left.0 - 1, self.bottom_left.1);

        for translated in self.translated_units(new_bottom_left) {
            if stopped_rocks.contains(&translated) {
                return false;
            }
        }

        self.bottom_left = new_bottom_left;

        true
    }

    fn try_move_right(&mut self, stopped_rocks: &HashSet<(u64, u64)>) -> bool {
        let new_bottom_left = (self.bottom_left.0 + 1, self.bottom_left.1);

        for translated in self.translated_units(new_bottom_left) {
            if translated.0 > 6 || stopped_rocks.contains(&translated) {
                return false;
            }
        }

        self.bottom_left = new_bottom_left;

        true
    }

    fn translated_units(&self, bottom_left: (u64, u64)) -> impl Iterator<Item = (u64, u64)> {
        self.units
            .iter()
            .map(move |(x, y)| (*x + bottom_left.0, *y + bottom_left.1))
    }

    fn rocks(&self) -> impl Iterator<Item = (u64, u64)> {
        self.translated_units(self.bottom_left)
    }

    fn highest_rock(&self) -> u64 {
        let mut highest = 0;

        for translated in self.translated_units(self.bottom_left) {
            if translated.1 > highest {
                highest = translated.1;
            }
        }

        highest + 1
    }
}
