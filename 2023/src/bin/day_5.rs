use aoc2023::*;

aoc_main!(
    day: 5,
    test_input: r#"
    seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4"#,
    task_1: task_1,
    expected_1: 35,
    task_2: task_2,
    expected_2: 46,
);

fn task_1(input: &str) -> Result<u64> {
    let mut parts = input.split("\n\n");

    let seeds_part = parts.next().unwrap();
    let seeds: Vec<u64> = seeds_part["seeds: ".len()..]
        .split_whitespace()
        .map(|d| d.parse::<u64>().unwrap())
        .collect();

    let mut maps: HashMap<(&str, &str), Vec<(u64, u64, u64)>> = HashMap::default();

    for map_part in parts {
        let mut map_lines = map_part.lines();

        let map_name_line = map_lines.next().unwrap();
        let (from, to) = map_name_line
            .trim_end_matches(" map:")
            .split_once("-to-")
            .unwrap();

        let mut map_values = vec![];
        for line in map_lines {
            let mut s = line.split_whitespace();

            let dst = s.next().unwrap().parse::<u64>().unwrap();
            let src = s.next().unwrap().parse::<u64>().unwrap();
            let range = s.next().unwrap().parse::<u64>().unwrap();

            map_values.push((dst, src, range));
        }

        maps.insert((from, to), map_values);
    }

    let path = vec![
        "soil",
        "fertilizer",
        "water",
        "light",
        "temperature",
        "humidity",
        "location",
    ];

    let mut lowest_location = u64::MAX;

    for seed in seeds {
        let mut current_step = "seed";
        let mut current_value = seed;

        for next_step in &path {
            let map = maps.get(&(current_step, next_step)).unwrap();

            let mut mapped_value = current_value;
            for &(dst, src, range) in map {
                if current_value >= src && current_value < (src + range) {
                    mapped_value = dst + (current_value - src);
                    break;
                }
            }

            current_value = mapped_value;
            current_step = next_step;
        }

        if current_value < lowest_location {
            lowest_location = current_value;
        }
    }

    Ok(lowest_location)
}

fn task_2(input: &str) -> Result<u64> {
    let mut parts = input.split("\n\n");

    let seeds_part = parts.next().unwrap();
    let seeds: Vec<(u64, u64)> = seeds_part["seeds: ".len()..]
        .split_whitespace()
        .map(|d| d.parse::<u64>().unwrap())
        .tuples()
        .collect();

    let mut maps: HashMap<(&str, &str), Vec<(u64, u64, u64)>> = HashMap::default();

    for map_part in parts {
        let mut map_lines = map_part.lines();

        let map_name_line = map_lines.next().unwrap();
        let (from, to) = map_name_line
            .trim_end_matches(" map:")
            .split_once("-to-")
            .unwrap();

        let mut map_values = vec![];
        for line in map_lines {
            let mut s = line.split_whitespace();

            let dst = s.next().unwrap().parse::<u64>().unwrap();
            let src = s.next().unwrap().parse::<u64>().unwrap();
            let range = s.next().unwrap().parse::<u64>().unwrap();

            map_values.push((dst, src, range));
        }

        maps.insert((from, to), map_values);
    }

    let path = vec![
        "soil",
        "fertilizer",
        "water",
        "light",
        "temperature",
        "humidity",
        "location",
    ];

    let mut current_values = seeds.clone();

    let mut current_step = "seed";
    for next_step in &path {
        let map = maps.get(&(current_step, next_step)).unwrap();

        let mut next_values = vec![];

        while let Some((current_start, current_num)) = current_values.pop() {
            let mut found = false;
            for &(dst, src, range) in map {
                let start_in_range = current_start >= src && current_start < (src + range);
                let end_in_range = (current_start + current_num) >= src
                    && (current_start + current_num) < (src + range);

                if start_in_range && end_in_range {
                    next_values.push((dst + (current_start - src), current_num));
                    found = true;
                    break;
                } else if start_in_range {
                    // Split :(
                    let left_num = src + range - current_start;
                    let right_num = current_num - left_num;

                    let left_start = current_start;
                    let right_start = src + range;

                    let left_mapped_start = dst + (left_start - src);
                    next_values.push((left_mapped_start, left_num));

                    current_values.push((right_start, right_num));

                    found = true;
                    break;
                }
            }

            if !found {
                next_values.push((current_start, current_num));
            }
        }

        current_values = next_values;
        current_step = next_step;
    }

    Ok(current_values.iter().map(|&(s, _)| s).min().unwrap())
}
