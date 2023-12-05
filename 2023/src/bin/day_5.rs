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

    let maps: Vec<Map> = parse_maps(&parts.collect::<Vec<&str>>());

    let lowest_location = seeds
        .iter()
        .map(|&seed| {
            let location_number = maps.iter().fold(seed, |current, map| map.convert(current));

            location_number
        })
        .min()
        .unwrap();

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

    let maps = parse_maps(&parts.collect::<Vec<&str>>());

    let location_number_ranges: Vec<(u64, u64)> = maps
        .iter()
        .fold(seeds, |ranges, map| map.convert_ranges(&ranges));

    let min_location = location_number_ranges
        .iter()
        .map(|&(start, _length)| start)
        .min()
        .unwrap();

    Ok(min_location)
}

fn parse_maps(map_parts: &[&str]) -> Vec<Map> {
    let mut maps = vec![];

    for map_part in map_parts {
        let mut map_lines = map_part.lines();

        let _map_name_line = map_lines.next().unwrap();

        let mut map_values = vec![];
        for line in map_lines {
            let mut s = line.split_whitespace();

            let dst = s.next().unwrap().parse::<u64>().unwrap();
            let src = s.next().unwrap().parse::<u64>().unwrap();
            let range = s.next().unwrap().parse::<u64>().unwrap();

            map_values.push((src, dst, range));
        }

        maps.push(Map::from_vec(map_values));
    }

    maps
}

struct Map(Vec</* src dst length */ (u64, u64, u64)>);

impl Map {
    fn from_vec(mut v: Vec<(u64, u64, u64)>) -> Map {
        // important: otherwise algorithm in convert_ranges does not work.
        v.sort();

        Map(v)
    }

    fn convert(&self, value: u64) -> u64 {
        let mut new_value = value;
        for &(src, dst, length) in self.0.iter() {
            if value >= src && value < (src + length) {
                new_value = dst + (value - src);
                break;
            }
        }

        new_value
    }

    fn convert_ranges(&self, ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
        let mut mapped_ranges = Vec::with_capacity(ranges.len());
        let mut current_ranges = Vec::from_iter(ranges.iter().cloned());
        let mut new_ranges = Vec::new();

        for &(src, dst, length) in self.0.iter() {
            current_ranges.extend(new_ranges.iter());
            new_ranges.clear();

            while let Some((range_start, range_length)) = current_ranges.pop() {
                let start_in_range = range_start >= src && range_start < (src + length);
                let end_in_range = (range_start + range_length) >= src
                    && (range_start + range_length) < (src + length);

                if start_in_range && end_in_range {
                    mapped_ranges.push((dst + (range_start - src), range_length));
                } else if start_in_range {
                    // Split :(
                    let left_num = src + length - range_start;
                    let right_num = range_length - left_num;

                    let left_start = range_start;
                    let right_start = src + length;

                    let left_mapped_start = dst + (left_start - src);
                    mapped_ranges.push((left_mapped_start, left_num));

                    new_ranges.push((right_start, right_num));
                } else {
                    new_ranges.push((range_start, range_length));
                }
            }
        }

        mapped_ranges.extend(new_ranges.iter());

        mapped_ranges
    }
}
