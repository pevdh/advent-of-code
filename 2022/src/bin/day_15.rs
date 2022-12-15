use aoc2022::*;

struct Sensor {
    location: (i64, i64),
    beacon_location: (i64, i64),
}

fn parse(raw_input: &str) -> Result<Vec<Sensor>> {
    Ok(raw_input.lines().map(parse_sensor).collect())
}

fn parse_sensor(line: &str) -> Sensor {
    use nom::bytes::complete::tag;
    use nom::character::complete::i64;
    use nom::combinator::map;
    use nom::sequence::preceded;
    use nom::sequence::separated_pair;
    use nom::sequence::tuple;

    let location = |i| {
        separated_pair(
            preceded(tag("x="), i64),
            tag(", "),
            preceded(tag("y="), i64),
        )(i)
    };

    let sensor = map(
        tuple((
            tag("Sensor at "),
            location,
            tag(": closest beacon is at "),
            location,
        )),
        |(_, sensor, _, beacon)| Sensor {
            location: sensor,
            beacon_location: beacon,
        },
    );

    nom_parse(line, sensor).unwrap()
}

fn task_1(sensors: &[Sensor]) -> Result<usize> {
    let mut covered_locations = HashSet::new();
    let y = 2000000;

    for sensor in sensors {
        for loc in find_covered_locations_at_y(sensor.location, sensor.beacon_location, y) {
            covered_locations.insert(loc);
        }
    }

    for sensor in sensors {
        if sensor.beacon_location.1 == y {
            covered_locations.remove(&sensor.beacon_location);
        }
    }

    let covered_at_y = covered_locations.len();

    Ok(covered_at_y)
}

fn manhattan_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn find_covered_locations_at_y(
    sensor_location: (i64, i64),
    closest_beacon: (i64, i64),
    y: i64,
) -> impl Iterator<Item = (i64, i64)> {
    let covered_x = (sensor_location.0 - closest_beacon.0).abs();
    let covered_y = (sensor_location.1 - closest_beacon.1).abs();
    let beacon_dist = covered_x + covered_y;

    ((sensor_location.0 - beacon_dist)..=(sensor_location.0 + beacon_dist))
        .filter(move |x| manhattan_distance((*x, y), sensor_location) <= beacon_dist)
        .map(move |x| (x, y))
}

fn task_2(sensors: &[Sensor]) -> Result<i64> {
    let search_space = 4_000_000;

    for s1 in sensors {
        for p in points_outside(s1) {
            if p.0 >= 0 && p.0 <= search_space && p.1 >= 0 && p.1 <= search_space {
                let mut covered = false;
                for s2 in sensors {
                    let beacon_dist = manhattan_distance(s2.location, s2.beacon_location);
                    let p_dist = manhattan_distance(p, s2.location);

                    if p_dist <= beacon_dist {
                        covered = true;
                        break;
                    }
                }

                if !covered {
                    return Ok(p.0 * 4_000_000 + p.1);
                }
            }
        }
    }

    Err(anyhow!("No solution"))
}

fn points_outside(sensor: &Sensor) -> impl Iterator<Item = (i64, i64)> + '_ {
    let beacon_dist = manhattan_distance(sensor.location, sensor.beacon_location);

    let min_x = sensor.location.0 - beacon_dist - 1;
    let max_x = sensor.location.0 + beacon_dist + 1;

    let mut rel_y = 0i64;
    let mut step = 1;
    let mut x_range = (min_x)..=max_x;

    std::iter::from_fn(move || {
        let x = x_range.next()?;

        let top = (x, sensor.location.1 - rel_y);
        let bottom = (x, sensor.location.1 + rel_y);

        if x == sensor.location.0 {
            step = -step;
        }

        rel_y += step;

        Some((top, bottom))
    })
    .flat_map(|(a, b)| [a, b].into_iter())
}

aoc_main!(
    day: 15,
    test_input:
    r#"
    Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 26,
    task_2: task_2,
    expected_2: 56000011,
);

#[cfg(test)]
mod tests {
    use crate::{points_outside, Sensor};

    #[test]
    fn it_works() {
        let sensor = Sensor {
            location: (0, 0),
            beacon_location: (1, 0),
        };

        assert_eq!(
            points_outside(&sensor),
            vec![
                (-2, 0),
                (2, 0),
                (-1, -1),
                (-1, 1),
                (0, -2),
                (0, 2),
                (1, -1),
                (1, 1),
            ]
        )
    }
}
