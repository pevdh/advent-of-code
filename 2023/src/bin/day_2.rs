use aoc2022::*;

aoc_main!(
    day: 2,
    test_input: r#"
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#,
    test_input_2: r#"
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#,
    parser: parse,
    task_1: task_1,
    expected_1: 8,
    task_2: task_2,
    expected_2: 2286,
);

struct Game {
    id: u64,
    sets: Vec<Set>,
}

struct Set {
    red: i64,
    green: i64,
    blue: i64,
}

fn parse(raw_input: &str) -> Result<Vec<Game>> {
    Ok(raw_input
        .lines()
        .map(|line| {
            let (s1, s2) = line.split_once(": ").unwrap();

            let s3 = &s1["Game ".len()..];
            let game_id = s3.parse::<u64>().unwrap();

            let mut sets = vec![];
            for set_str in s2.split("; ") {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;

                for cube_str in set_str.split(", ") {
                    let (left, right) = cube_str.split_once(' ').unwrap();
                    let num = left.parse::<i64>().unwrap();

                    match right {
                        "red" => red = num,
                        "green" => green = num,
                        "blue" => blue = num,
                        _ => panic!("Invalid cube color: {}", right),
                    };
                }

                sets.push(Set { red, green, blue });
            }

            Game { id: game_id, sets }
        })
        .collect())
}

fn task_1(games: &[Game]) -> Result<u64> {
    Ok(games
        .iter()
        .filter(|game| {
            let possible = game
                .sets
                .iter()
                .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14);

            possible
        })
        .map(|game| game.id)
        .sum())
}

fn task_2(games: &[Game]) -> Result<i64> {
    Ok(games
        .iter()
        .map(|game| {
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;

            for set in game.sets.iter() {
                if min_red < set.red {
                    min_red = set.red;
                }

                if min_green < set.green {
                    min_green = set.green;
                }

                if min_blue < set.blue {
                    min_blue = set.blue;
                }
            }

            min_red * min_green * min_blue
        })
        .sum())
}
