use aoc2021::*;

#[derive(Debug)]
struct ParsedInput {
    lines: Vec<Line>,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    pub fn points(&self) -> Vec<Point> {
        let (x1, x2) = (self.p1.x, self.p2.x);
        let (y1, y2) = (self.p1.y, self.p2.y);
        
        let mut points = vec![];
        let mut current = Point { x: x1, y: y1 };
        let step_x = (x2 - x1).signum();
        let step_y = (y2 - y1).signum();

        while current != self.p2 {
            points.push(current.clone());
            current.x += step_x;
            current.y += step_y;
        }

        points.push(current.clone());

        points
    }

    fn is_diagonal(&self) -> bool {
        return self.p1.x != self.p2.x && self.p1.y != self.p2.y;
    }
}

fn parse(raw_input: &str) -> Result<ParsedInput> {
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, i32, newline};
    use nom::combinator::{all_consuming, map};
    use nom::multi::separated_list0;
    use nom::sequence::separated_pair;

    let point1 = map(separated_pair(i32, char(','), i32), |(x, y)| Point { x, y });
    let point2 = map(separated_pair(i32, char(','), i32), |(x, y)| Point { x, y });
    let line = map(separated_pair(point1, tag(" -> "), point2), |(p1, p2)| Line { p1, p2 });
    let file = map(separated_list0(newline, line), |lines| ParsedInput { lines });
    let parser = all_consuming(file);

    nom_parse(raw_input, parser)
}

fn task_1(input: &ParsedInput) -> Result<i32> {
    let mut counts: HashMap<Point, i32> = HashMap::new();

    for line in &input.lines {
        if line.is_diagonal() {
            continue;
        }

        for point in line.points() {
            let entry = counts.entry(point).or_insert(0);
            *entry += 1;
        }
    }

    let num_points_with_at_least_two_lines = counts.into_iter()
        .filter(|(_p, count)| *count >= 2)
        .count();

    Ok(num_points_with_at_least_two_lines as i32)
}

fn task_2(input: &ParsedInput) -> Result<i32> {
    let mut counts: HashMap<Point, i32> = HashMap::new();

    for line in &input.lines {
        for point in line.points() {
            let entry = counts.entry(point).or_insert(0);
            *entry += 1;
        }
    }

    let num_points_with_at_least_two_lines = counts.into_iter()
        .filter(|(_p, count)| *count >= 2)
        .count();

    Ok(num_points_with_at_least_two_lines as i32)
}

aoc_main!(
    day: 5,
    test_input: r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#,
    parser: parse,
    task_1: task_1,
    expected_1: 5,
    task_2: task_2,
    expected_2: 12,
);
