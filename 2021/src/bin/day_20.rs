use aoc2021::*;
use ndarray::Array1;
use ndarray::s;

aoc_main!(
    day: 20,
    test_input: r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"#,
    parser: parse,
    task_1: task_1,
    expected_1: 35,
    task_2: task_2,
    expected_2: 3351,
);

#[derive(Debug)]
struct ParsedInput {
    algorithm: Array1<u8>,
    input_image: Array2<u8>,
}


fn parse(raw_input: &str) -> Result<ParsedInput> {
    let mut parts = raw_input
        .split("\n\n");

    let algorithm_part = parts.next().unwrap();
    let image_part = parts.next().unwrap();

    let algorithm: Vec<u8> = algorithm_part.chars().filter(|&c| c == '#' || c == '.')
        .map(|c| match c {
            '#' => 1u8,
            '.' => 0u8,
            _ => panic!("Invalid char"),
        })
        .collect();

    let image: Vec<u8> = image_part.lines()
        .filter(|line| line.len() > 0)
        .flat_map(|line| line.chars().map(|c| match c {
            '#' => 1u8,
            '.' => 0u8,
            _ => panic!("Invalid char"),
        }).collect::<Vec<u8>>())
        .collect();

    let width = image_part.lines().next().unwrap().len();
    let height = image_part.lines().count();

    Ok(ParsedInput {
        algorithm: Array1::from_vec(algorithm),
        input_image: Array2::from_shape_vec((height, width), image).unwrap(),
    })
}

fn task_1(input: &ParsedInput) -> Result<usize> {
    let image = InfiniteImage::from_input_image(input.input_image.clone());
    let enhanced_image = image.enhanced(&input.algorithm);
    let enhanced_image = enhanced_image.enhanced(&input.algorithm);

    Ok(enhanced_image.light_pixels().unwrap())
}

fn task_2(input: &ParsedInput) -> Result<usize> {
    let mut infinite_image = InfiniteImage::from_input_image(input.input_image.clone());

    for _ in 0..50 {
        infinite_image = infinite_image.enhanced(&input.algorithm);
    }

    Ok(infinite_image.light_pixels().unwrap())
}

struct InfiniteImage {
    data: Array2<u8>,
    fill_pixel: u8,
}

impl InfiniteImage {

    fn from_input_image(input_image: Array2<u8>) -> InfiniteImage {
        InfiniteImage {
            data: input_image,
            fill_pixel: 0,
        }
    }

    fn light_pixels(&self) -> Option<usize> {
        if self.fill_pixel == 0 {
            Some(self.data.iter().filter(|v| **v == 1).count())
        } else {
            None
        }
    }

    fn enhanced(&self, algorithm: &Array1<u8>) -> InfiniteImage {
        let mut result = Array2::zeros((self.data.nrows() + 6, self.data.ncols() + 6));

        // Pad image with three rows and columns of zeros
        let mut padded_image = Array2::from_elem((self.data.nrows() + 6, self.data.ncols() + 6), self.fill_pixel);
        padded_image
            .slice_mut(s![3..-3, 3..-3])
            .assign(&self.data);

        for i in 1..(padded_image.nrows() - 1) {
            for j in 1..(padded_image.ncols() - 1) {
                let bits = [
                    padded_image[(i - 1, j - 1)],
                    padded_image[(i - 1, j)],
                    padded_image[(i - 1, j + 1)],
                    padded_image[(i, j - 1)],
                    padded_image[(i, j)],
                    padded_image[(i, j + 1)],
                    padded_image[(i + 1, j - 1)],
                    padded_image[(i + 1, j)],
                    padded_image[(i + 1, j + 1)],
                ];

                let idx = to_u16(&bits);

                let value = algorithm[idx as usize];

                result[(i, j)] = value;
            }
        }

        InfiniteImage {
            data: result.slice(s![2..-2,2..-2]).into_owned(),
            fill_pixel: if self.fill_pixel == 0 { algorithm[0] } else { algorithm[511] },
        }

    }
}

fn to_u16(bits: &[u8]) -> u16 {
    let mut result: u16 = 0;

    for &bit in bits {
        result <<= 1;
        result |= bit as u16;
    }

    result
}
