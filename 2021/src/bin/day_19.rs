use aoc2021::*;
use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;

aoc_main!(
    day: 19,
    test_input: r#"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14"#,
    parser: parse,
    task_1: task_1,
    expected_1: 79,
    task_2: task_2,
    expected_2: 3621,
);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Rotation {
    x: u8,
    y: u8,
    z: u8,
}

#[derive(Debug)]
struct ScannerReport {
    relative_beacon_coordinates: Vec<Coordinates>,
}

#[derive(Debug)]
struct KnownScannerLocation {
    coords: Coordinates,
    beacon_coords: Vec<Coordinates>,
}

type ParsedInput = Vec<ScannerReport>;

fn parse(raw_input: &str) -> Result<ParsedInput> {
    let mut reports = Vec::new();
    let mut relative_beacon_coordinates = Vec::new();
    for line in raw_input.lines() {
        if line.starts_with("---") {
            continue;
        }

        if line == "" {
            reports.push(ScannerReport {
                relative_beacon_coordinates,
            });

            relative_beacon_coordinates = Vec::new();
            continue;
        }

        let mut split = line.split(",");
        let coordinate = Coordinates {
            x: split.next().unwrap().parse()?,
            y: split.next().unwrap().parse()?,
            z: split.next().unwrap().parse()?,
        };

        relative_beacon_coordinates.push(coordinate);
    }

    reports.push(ScannerReport {
        relative_beacon_coordinates,
    });

    return Ok(reports);
}

fn task_1(reports: &ParsedInput) -> Result<usize> {
    let known_scanner_locations = find_scanner_locations(reports);

    let known_beacon_coordinates: HashSet<&Coordinates> = HashSet::from_iter(
        known_scanner_locations
            .iter()
            .flat_map(|l| &l.beacon_coords),
    );

    return Ok(known_beacon_coordinates.len());
}

fn task_2(reports: &ParsedInput) -> Result<i32> {
    let known_scanner_locations = find_scanner_locations(reports);
    let largest_manhattan_distance = known_scanner_locations
        .iter()
        .cartesian_product(known_scanner_locations.iter())
        .map(|(a, b)| manhattan_distance(&a.coords, &b.coords))
        .max();

    return largest_manhattan_distance.ok_or(anyhow!("No solution"));
}

fn find_scanner_locations(reports: &Vec<ScannerReport>) -> Vec<KnownScannerLocation> {
    let mut unknown_scanner_reports: VecDeque<&ScannerReport> =
        VecDeque::from_iter(reports.iter().skip(1));
    let rotations = compute_rotations();

    // Use scanner 0's report as our absolute coordinate system
    let mut known_scanner_locations = vec![KnownScannerLocation {
        coords: Coordinates { x: 0, y: 0, z: 0 },
        beacon_coords: reports[0].relative_beacon_coordinates.clone(),
    }];

    while let Some(unknown_scanner_report) = unknown_scanner_reports.pop_front() {
        let found = known_scanner_locations
            .iter()
            .map(|known_scanner_location| {
                find_scanner_location(known_scanner_location, unknown_scanner_report, &rotations)
            })
            .filter(|found| found.is_some())
            .next()
            .flatten();

        if let Some((scanner_location, rotation)) = found {
            known_scanner_locations.push(KnownScannerLocation {
                beacon_coords: unknown_scanner_report
                    .relative_beacon_coordinates
                    .iter()
                    .map(|c| add(&scanner_location, &rotate(&c, &rotation)))
                    .collect(),
                coords: scanner_location,
            });
        } else {
            unknown_scanner_reports.push_back(unknown_scanner_report);
        }
    }

    return known_scanner_locations;
}

fn compute_rotations() -> Vec<Rotation> {
    let times = [0u8, 1, 2, 3];

    let mut rotated_coords = HashSet::new();
    let mut rotations = Vec::new();
    let coords = Coordinates { x: 1, y: 2, z: 3 };
    for (&rotate_x_times, &rotate_y_times, &rotate_z_times) in
        itertools::iproduct!(&times, &times, &times)
    {
        let rotation = Rotation {
            x: rotate_x_times,
            y: rotate_y_times,
            z: rotate_z_times,
        };

        let rotated = rotate(&coords, &rotation);
        if !rotated_coords.contains(&rotated) {
            rotated_coords.insert(rotated);
            rotations.push(rotation);
        }
    }

    return rotations;
}

fn find_scanner_location(
    known_scanner_location: &KnownScannerLocation,
    report: &ScannerReport,
    rotations: &Vec<Rotation>,
) -> Option<(Coordinates, Rotation)> {
    for rotation in rotations {
        let rotated_relative_coordinates: Vec<Coordinates> = report
            .relative_beacon_coordinates
            .iter()
            .map(|c| rotate(c, &rotation))
            .collect();

        for rotated_relative_coords in &rotated_relative_coordinates {
            for known_beacon_coords in &known_scanner_location.beacon_coords {
                let potential_scanner_coords =
                    add(known_beacon_coords, &invert(rotated_relative_coords));

                let absolute_beacon_coords: Vec<Coordinates> = rotated_relative_coordinates
                    .iter()
                    .map(|c| add(&potential_scanner_coords, c))
                    .collect();

                let num_overlapping = absolute_beacon_coords
                    .iter()
                    .filter(|abs_coord| known_scanner_location.beacon_coords.contains(abs_coord))
                    .take(12)
                    .count();

                if num_overlapping >= 12 {
                    return Some((potential_scanner_coords, rotation.clone()));
                }
            }
        }
    }

    return None;
}

fn add(a: &Coordinates, b: &Coordinates) -> Coordinates {
    Coordinates {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
}

fn invert(a: &Coordinates) -> Coordinates {
    Coordinates {
        x: -a.x,
        y: -a.y,
        z: -a.z,
    }
}

fn rotate_x(c: &Coordinates, times: u8) -> Coordinates {
    match times {
        0 => (c.x, c.y, c.z),
        1 => (c.x, c.z, -c.y),
        2 => (c.x, -c.y, -c.z),
        3 => (c.x, -c.z, c.y),
        _ => panic!("too many rotations"),
    }
    .into()
}

fn rotate_y(c: &Coordinates, times: u8) -> Coordinates {
    match times {
        0 => (c.x, c.y, c.z),
        1 => (-c.z, c.y, c.x),
        2 => (-c.x, c.y, -c.z),
        3 => (c.z, c.y, -c.x),
        _ => panic!("too many rotations"),
    }
    .into()
}

fn rotate_z(c: &Coordinates, times: u8) -> Coordinates {
    match times {
        0 => (c.x, c.y, c.z),
        1 => (-c.y, c.x, c.z),
        2 => (-c.x, -c.y, c.z),
        3 => (c.y, -c.x, c.z),
        _ => panic!("too many rotations"),
    }
    .into()
}

fn rotate(c: &Coordinates, rotation: &Rotation) -> Coordinates {
    return rotate_x(&rotate_y(&rotate_z(c, rotation.z), rotation.y), rotation.x);
}

impl From<(i32, i32, i32)> for Coordinates {
    fn from(c: (i32, i32, i32)) -> Self {
        Coordinates {
            x: c.0,
            y: c.1,
            z: c.2,
        }
    }
}

fn manhattan_distance(a: &Coordinates, b: &Coordinates) -> i32 {
    return (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs();
}
