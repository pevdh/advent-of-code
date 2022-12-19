use aoc2022::*;

aoc_main!(
    day: 19,
    test_input:
    r#"
    Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 33,
    task_2: task_2,
    expected_2: 3472,
);

#[derive(Debug)]
struct Blueprint {
    id: i64,
    ore_robot_ore_cost: i64,
    clay_robot_ore_cost: i64,
    obsidian_robot_ore_cost: i64,
    obsidian_robot_clay_cost: i64,
    geode_robot_ore_cost: i64,
    geode_robot_obsidian_cost: i64,
}

fn parse(raw_input: &str) -> Result<Vec<Blueprint>> {
    Ok(raw_input.lines().map(parse_blueprint).collect())
}

fn parse_blueprint(line: &str) -> Blueprint {
    let keep_digits = |s: &str| s.chars().filter(|c| c.is_digit(10)).collect::<String>();
    let numbers: Vec<i64> = line
        .split(' ')
        .filter_map(|p| keep_digits(p).parse().ok())
        .collect::<Vec<_>>();

    Blueprint {
        id: numbers[0],
        ore_robot_ore_cost: numbers[1],
        clay_robot_ore_cost: numbers[2],
        obsidian_robot_ore_cost: numbers[3],
        obsidian_robot_clay_cost: numbers[4],
        geode_robot_ore_cost: numbers[5],
        geode_robot_obsidian_cost: numbers[6],
    }
}

#[derive(Debug, Clone)]
struct State {
    n_ore_robots: i64,
    n_clay_robots: i64,
    n_obsidian_robots: i64,
    n_geode_robots: i64,

    n_ores: i64,
    n_clay: i64,
    n_obsidian: i64,
    n_geodes: i64,
}

fn task_1(blueprints: &[Blueprint]) -> Result<i64> {
    Ok(blueprints
        .iter()
        .map(|blueprint| max_geodes(blueprint, 24) * blueprint.id)
        .sum())
}

fn task_2(blueprints: &[Blueprint]) -> Result<i64> {
    let blueprints = &blueprints[..std::cmp::min(blueprints.len(), 3)];
    Ok(blueprints
        .iter()
        .map(|blueprint| max_geodes(blueprint, 32))
        .product())
}

fn max_geodes(blueprint: &Blueprint, minutes: i64) -> i64 {
    let mut states = vec![State {
        n_ore_robots: 1,
        n_clay_robots: 0,
        n_obsidian_robots: 0,
        n_geode_robots: 0,

        n_ores: 0,
        n_clay: 0,
        n_obsidian: 0,
        n_geodes: 0,
    }];

    let mut max_geodes = 0;

    for _ in 1..=minutes {
        let mut new_states = vec![];
        for state in &mut states {
            if state.n_ores >= blueprint.geode_robot_ore_cost
                && state.n_obsidian >= blueprint.geode_robot_obsidian_cost
            {
                let mut clone = state.clone();
                clone.n_ores -= blueprint.geode_robot_ore_cost;
                clone.n_obsidian -= blueprint.geode_robot_obsidian_cost;

                clone.n_ores += clone.n_ore_robots;
                clone.n_clay += clone.n_clay_robots;
                clone.n_obsidian += clone.n_obsidian_robots;
                clone.n_geodes += clone.n_geode_robots;

                clone.n_geode_robots += 1;
                new_states.push(clone);
            }

            if state.n_ores >= blueprint.obsidian_robot_ore_cost
                && state.n_clay >= blueprint.obsidian_robot_clay_cost
            {
                let mut clone = state.clone();
                clone.n_ores -= blueprint.obsidian_robot_ore_cost;
                clone.n_clay -= blueprint.obsidian_robot_clay_cost;

                clone.n_ores += clone.n_ore_robots;
                clone.n_clay += clone.n_clay_robots;
                clone.n_obsidian += clone.n_obsidian_robots;
                clone.n_geodes += clone.n_geode_robots;

                clone.n_obsidian_robots += 1;
                new_states.push(clone);
            }

            if state.n_ores >= blueprint.clay_robot_ore_cost {
                let mut clone = state.clone();
                clone.n_ores -= blueprint.clay_robot_ore_cost;

                clone.n_ores += clone.n_ore_robots;
                clone.n_clay += clone.n_clay_robots;
                clone.n_obsidian += clone.n_obsidian_robots;
                clone.n_geodes += clone.n_geode_robots;

                clone.n_clay_robots += 1;
                new_states.push(clone);
            }

            if state.n_ores >= blueprint.ore_robot_ore_cost {
                let mut clone = state.clone();
                clone.n_ores -= blueprint.ore_robot_ore_cost;

                clone.n_ores += clone.n_ore_robots;
                clone.n_clay += clone.n_clay_robots;
                clone.n_obsidian += clone.n_obsidian_robots;
                clone.n_geodes += clone.n_geode_robots;

                clone.n_ore_robots += 1;
                new_states.push(clone);
            }

            state.n_ores += state.n_ore_robots;
            state.n_clay += state.n_clay_robots;
            state.n_obsidian += state.n_obsidian_robots;
            state.n_geodes += state.n_geode_robots;
        }

        states.extend(new_states.into_iter());

        for state in &mut states {
            if state.n_geodes > max_geodes {
                max_geodes = state.n_geodes;
            }
        }

        states.sort_by(|a, b| {
            if a.n_geode_robots != b.n_geode_robots {
                return b.n_geode_robots.cmp(&a.n_geode_robots);
            }

            if a.n_obsidian_robots != b.n_obsidian_robots {
                return b.n_obsidian_robots.cmp(&a.n_obsidian_robots);
            }

            if a.n_clay_robots != b.n_clay_robots {
                return b.n_clay_robots.cmp(&a.n_clay_robots);
            }

            b.n_ore_robots.cmp(&a.n_ore_robots)
        });

        if states.len() > 1_000_000 {
            states.drain(1_000_000..);
        }
    }

    max_geodes
}
