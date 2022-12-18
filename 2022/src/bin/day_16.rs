use aoc2022::*;

aoc_main!(
    day: 16,
    test_input:
    r#"
    Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    Valve BB has flow rate=13; tunnels lead to valves CC, AA
    Valve CC has flow rate=2; tunnels lead to valves DD, BB
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    Valve EE has flow rate=3; tunnels lead to valves FF, DD
    Valve FF has flow rate=0; tunnels lead to valves EE, GG
    Valve GG has flow rate=0; tunnels lead to valves FF, HH
    Valve HH has flow rate=22; tunnel leads to valve GG
    Valve II has flow rate=0; tunnels lead to valves AA, JJ
    Valve JJ has flow rate=21; tunnel leads to valve II
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 1651,
    task_2: task_2,
    expected_2: 1707,
);

type ValveId = [char; 2];

#[derive(Debug, Clone)]
struct Valve {
    id: ValveId,
    flow_rate: u32,
    connections: Vec<ValveId>,
}

fn parse(raw_input: &str) -> Result<Vec<Valve>> {
    Ok(raw_input.lines().map(parse_valve).collect())
}

fn parse_valve(line: &str) -> Valve {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, u32};
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::tuple;

    let valve_id = |i| {
        map(alpha1, |cc: &str| {
            [cc.chars().next().unwrap(), cc.chars().nth(1).unwrap()]
        })(i)
    };

    let valve = map(
        tuple((
            tag("Valve "),
            valve_id,
            tag(" has flow rate="),
            u32,
            tag("; "),
            alt((tag("tunnel leads to "), tag("tunnels lead to "))),
            alt((tag("valve "), tag("valves "))),
            separated_list1(tag(", "), valve_id),
        )),
        |(_, id, _, flow_rate, _, _, _, connections)| Valve {
            id,
            flow_rate,
            connections,
        },
    );

    nom_parse(line, valve).unwrap()
}

fn task_1(valves: &[Valve]) -> Result<u32> {
    let valves: HashMap<ValveId, Valve> =
        HashMap::from_iter(valves.iter().map(|v| (v.id, v.clone())));

    let costs = build_valve_path_costs(&valves);

    let pressure = calculate_most_pressure_possible(
        &valves,
        &costs,
        &State {
            current_id: ['A', 'A'],
            current_pressure: 0,
            minutes_left: 30,
            closed_valves: valves
                .iter()
                .filter(|(_, v)| v.flow_rate > 0)
                .map(|(k, _)| k)
                .copied()
                .collect(),
        },
    );

    Ok(pressure)
}

fn task_2(valves: &[Valve]) -> Result<u32> {
    let valves: HashMap<ValveId, Valve> =
        HashMap::from_iter(valves.iter().map(|v| (v.id, v.clone())));

    let costs = build_valve_path_costs(&valves);

    let pressure = calculate_most_pressure_possible_with_elephant(
        &valves,
        &costs,
        &StateWithElephant {
            me_current_id: ['A', 'A'],
            elephant_current_id: ['A', 'A'],
            current_pressure: 0,
            me_minutes_left: 26,
            elephant_minutes_left: 26,
            closed_valves: valves
                .iter()
                .filter(|(_, v)| v.flow_rate > 0)
                .map(|(k, _)| k)
                .copied()
                .collect(),
        },
    );

    Ok(pressure)
}

fn build_valve_path_costs(valves: &HashMap<ValveId, Valve>) -> HashMap<(ValveId, ValveId), u32> {
    let mut costs = HashMap::new();

    for &v1 in valves.keys() {
        for &v2 in valves.keys() {
            costs.insert((v1, v2), cost(valves, v1, v2).unwrap());
        }
    }

    costs
}

fn cost(valves: &HashMap<ValveId, Valve>, from: ValveId, to: ValveId) -> Option<u32> {
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::new();

    to_visit.push_back((from, 0));

    while let Some((current_id, cost)) = to_visit.pop_front() {
        if current_id == to {
            return Some(cost + 1);
        }

        visited.insert(current_id);

        for &connection in &valves[&current_id].connections {
            if !visited.contains(&connection) {
                to_visit.push_back((connection, cost + 1));
            }
        }
    }

    None
}

struct State {
    current_id: ValveId,
    minutes_left: u32,
    current_pressure: u32,
    closed_valves: Vec<ValveId>,
}

fn calculate_most_pressure_possible(
    valves: &HashMap<ValveId, Valve>,
    costs: &HashMap<(ValveId, ValveId), u32>,
    state: &State,
) -> u32 {
    let mut max_pressure = state.current_pressure;

    for &valve in &state.closed_valves {
        // Cost to navigate to and open the valve
        let cost = costs[&(state.current_id, valve)];

        if cost <= state.minutes_left {
            let closed_valves = state
                .closed_valves
                .iter()
                .filter(|v| **v != valve)
                .copied()
                .collect();

            let increased_pressure = valves[&valve].flow_rate * (state.minutes_left - cost);

            let pressure = calculate_most_pressure_possible(
                valves,
                costs,
                &State {
                    current_id: valve,
                    current_pressure: state.current_pressure + increased_pressure,
                    minutes_left: state.minutes_left - cost,
                    closed_valves,
                },
            );

            if pressure > max_pressure {
                max_pressure = pressure;
            }
        }
    }

    max_pressure
}

struct StateWithElephant {
    me_current_id: ValveId,
    elephant_current_id: ValveId,
    me_minutes_left: u32,
    elephant_minutes_left: u32,
    current_pressure: u32,
    closed_valves: Vec<ValveId>,
}

fn calculate_most_pressure_possible_with_elephant(
    valves: &HashMap<ValveId, Valve>,
    costs: &HashMap<(ValveId, ValveId), u32>,
    state: &StateWithElephant,
) -> u32 {
    let mut max_pressure = state.current_pressure;

    let mut ffound = false;
    for &my_valve in &state.closed_valves {
        for &elephant_valve in &state.closed_valves {
            if my_valve == elephant_valve {
                continue;
            }

            // Cost to navigate to and open the valve
            let my_cost = costs[&(state.me_current_id, my_valve)];
            let elephant_cost = costs[&(state.elephant_current_id, elephant_valve)];

            if my_cost <= state.me_minutes_left && elephant_cost <= state.elephant_minutes_left {
                ffound = true;

                let closed_valves = state
                    .closed_valves
                    .iter()
                    .filter(|v| **v != my_valve && **v != elephant_valve)
                    .copied()
                    .collect();
                let my_increased_pressure =
                    valves[&my_valve].flow_rate * (state.me_minutes_left - my_cost);
                let elephant_increased_pressure = valves[&elephant_valve].flow_rate
                    * (state.elephant_minutes_left - elephant_cost);

                let pressure = calculate_most_pressure_possible_with_elephant(
                    valves,
                    costs,
                    &StateWithElephant {
                        me_current_id: my_valve,
                        elephant_current_id: elephant_valve,
                        current_pressure: state.current_pressure
                            + my_increased_pressure
                            + elephant_increased_pressure,
                        me_minutes_left: state.me_minutes_left - my_cost,
                        elephant_minutes_left: state.elephant_minutes_left - elephant_cost,
                        closed_valves,
                    },
                );

                if pressure > max_pressure {
                    max_pressure = pressure;
                }
            }
        }
    }

    if !ffound {
        for &valve in &state.closed_valves {
            let my_cost = costs[&(state.me_current_id, valve)];
            let elephant_cost = costs[&(state.elephant_current_id, valve)];

            if my_cost <= state.me_minutes_left {
                let closed_valves = state
                    .closed_valves
                    .iter()
                    .filter(|v| **v != valve)
                    .copied()
                    .collect();
                let my_increased_pressure =
                    valves[&valve].flow_rate * (state.me_minutes_left - my_cost);

                let pressure = calculate_most_pressure_possible_with_elephant(
                    valves,
                    costs,
                    &StateWithElephant {
                        me_current_id: valve,
                        elephant_current_id: state.elephant_current_id,
                        current_pressure: state.current_pressure + my_increased_pressure,
                        me_minutes_left: state.me_minutes_left - my_cost,
                        elephant_minutes_left: state.elephant_minutes_left,
                        closed_valves,
                    },
                );

                if pressure > max_pressure {
                    max_pressure = pressure;
                }
            }

            if elephant_cost <= state.elephant_minutes_left {
                let closed_valves = state
                    .closed_valves
                    .iter()
                    .filter(|v| **v != valve)
                    .copied()
                    .collect();
                let elephant_increased_pressure =
                    valves[&valve].flow_rate * (state.elephant_minutes_left - elephant_cost);

                let pressure = calculate_most_pressure_possible_with_elephant(
                    valves,
                    costs,
                    &StateWithElephant {
                        me_current_id: state.me_current_id,
                        elephant_current_id: valve,
                        current_pressure: state.current_pressure + elephant_increased_pressure,
                        me_minutes_left: state.me_minutes_left,
                        elephant_minutes_left: state.elephant_minutes_left - elephant_cost,
                        closed_valves,
                    },
                );

                if pressure > max_pressure {
                    max_pressure = pressure;
                }
            }
        }
    }

    max_pressure
}
