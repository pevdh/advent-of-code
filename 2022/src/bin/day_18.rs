use aoc2022::*;

aoc_main!(
    day: 18,
    test_input:
    r#"
    2,2,2
    1,2,2
    3,2,2
    2,1,2
    2,3,2
    2,2,1
    2,2,3
    2,2,4
    2,2,6
    1,2,5
    3,2,5
    2,1,5
    2,3,5
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 64,
    task_2: task_2,
    expected_2: 58,
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl From<(i32, i32, i32)> for Coord {
    fn from(t: (i32, i32, i32)) -> Self {
        Coord {
            x: t.0,
            y: t.1,
            z: t.2,
        }
    }
}

fn parse(raw_input: &str) -> Result<Vec<Coord>> {
    Ok(raw_input
        .lines()
        .map(|line| {
            let mut s = line.split(',');

            Coord {
                x: s.next().unwrap().parse().unwrap(),
                y: s.next().unwrap().parse().unwrap(),
                z: s.next().unwrap().parse().unwrap(),
            }
        })
        .collect())
}

fn task_1(cubes: &[Coord]) -> Result<usize> {
    let cubes: HashSet<Coord> = HashSet::from_iter(cubes.iter().copied());

    let mut faces = 0;

    for cube in &cubes {
        let adjacent_rel_coords = [
            (1, 0, 0),
            (0, 1, 0),
            (0, 0, 1),
            (-1, 0, 0),
            (0, -1, 0),
            (0, 0, -1),
        ];

        let adjacent_cubes = adjacent_rel_coords.map(|rel_coord| Coord {
            x: cube.x + rel_coord.0,
            y: cube.y + rel_coord.1,
            z: cube.z + rel_coord.2,
        });

        faces += adjacent_cubes
            .iter()
            .filter(|adjacent_cube| !cubes.contains(adjacent_cube))
            .count();
    }

    Ok(faces)
}

fn task_2(cubes: &[Coord]) -> Result<usize> {
    let cubes: HashSet<Coord> = HashSet::from_iter(cubes.iter().copied());
    let mut known_trapped_cubes: HashSet<Coord> = HashSet::default();
    let mut known_untrapped_cubes: HashSet<Coord> = HashSet::default();

    let bb = compute_bounding_box(&cubes);

    let mut faces = 0;

    for cube in &cubes {
        let adjacent_rel_coords = [
            (1, 0, 0),
            (0, 1, 0),
            (0, 0, 1),
            (-1, 0, 0),
            (0, -1, 0),
            (0, 0, -1),
        ];

        let adjacent_cubes = adjacent_rel_coords.map(|rel_coord| Coord {
            x: cube.x + rel_coord.0,
            y: cube.y + rel_coord.1,
            z: cube.z + rel_coord.2,
        });

        for adjacent_cube in adjacent_cubes {
            if !cubes.contains(&adjacent_cube) {
                if known_trapped_cubes.contains(&adjacent_cube) {
                    continue;
                }

                if known_untrapped_cubes.contains(&adjacent_cube) {
                    faces += 1;
                    continue;
                }

                let (is_trapped, cubes) = compute_trapped_cubes(&cubes, &bb, adjacent_cube);
                if is_trapped {
                    known_trapped_cubes.extend(cubes.iter());
                } else {
                    faces += 1;
                    known_untrapped_cubes.extend(cubes.iter());
                }
            }
        }
    }

    Ok(faces)
}

struct BoundingBox {
    min: Coord,
    max: Coord,
}

fn compute_bounding_box(cubes: &HashSet<Coord>) -> BoundingBox {
    let min_x = cubes.iter().map(|c| c.x).min().unwrap();
    let max_x = cubes.iter().map(|c| c.x).max().unwrap();
    let min_y = cubes.iter().map(|c| c.y).min().unwrap();
    let max_y = cubes.iter().map(|c| c.y).max().unwrap();
    let min_z = cubes.iter().map(|c| c.z).min().unwrap();
    let max_z = cubes.iter().map(|c| c.z).max().unwrap();

    BoundingBox {
        min: (min_x, min_y, min_z).into(),
        max: (max_x, max_y, max_z).into(),
    }
}

fn compute_trapped_cubes(
    cubes: &HashSet<Coord>,
    bounding_box: &BoundingBox,
    start: Coord,
) -> (bool, HashSet<Coord>) {
    let adjacent_rel_coords = [
        (1, 0, 0),
        (0, 1, 0),
        (0, 0, 1),
        (-1, 0, 0),
        (0, -1, 0),
        (0, 0, -1),
    ];

    let mut to_visit = VecDeque::new();
    let mut visited = HashSet::default();

    to_visit.push_back(start);
    visited.insert(start);

    while let Some(current) = to_visit.pop_front() {
        let adjacent_cubes = adjacent_rel_coords.map(|rel_coord| Coord {
            x: current.x + rel_coord.0,
            y: current.y + rel_coord.1,
            z: current.z + rel_coord.2,
        });

        for adjacent_cube in adjacent_cubes {
            if adjacent_cube.x < bounding_box.min.x
                || adjacent_cube.x > bounding_box.max.x
                || adjacent_cube.y < bounding_box.min.y
                || adjacent_cube.y > bounding_box.max.y
                || adjacent_cube.z < bounding_box.min.z
                || adjacent_cube.z > bounding_box.max.z
            {
                return (false, visited);
            }

            if !cubes.contains(&adjacent_cube) && !visited.contains(&adjacent_cube) {
                visited.insert(adjacent_cube);
                to_visit.push_back(adjacent_cube);
            }
        }
    }

    (true, visited)
}
