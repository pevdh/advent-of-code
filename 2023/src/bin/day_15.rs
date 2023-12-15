use aoc2023::*;
use ascii::ToAsciiChar;

aoc_main!(
    day: 15,
    test_input: r#"
    rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#,
    task_1: task_1,
    expected_1: 1320,
    task_2: task_2,
    expected_2: 145,
);

fn task_1(input: &str) -> Result<u64> {
    let result = input
        .split(',')
        .map(|step| hash(step).unwrap() as u64)
        .sum();

    Ok(result)
}

fn task_2(input: &str) -> Result<u64> {
    let mut hm = HASHMAP::new();

    for step in input.split(',') {
        if let Some((label, focal_length)) = step.split_once('=') {
            hm.insert(label, focal_length.parse::<u64>()?)?;
        } else {
            let label = step.trim_end_matches('-');
            hm.remove(label)?;
        }
    }

    Ok(hm.focusing_power())
}

fn hash(s: &str) -> Result<usize> {
    let mut hash = 0;
    for ch in s.chars() {
        let ch = ch.to_ascii_char()?;
        hash = ((hash + ch.as_byte() as usize) * 17) % 256;
    }

    Ok(hash)
}

struct Lens<'a> {
    label: &'a str,
    focal_length: u64,
}

#[allow(clippy::upper_case_acronyms)]
struct HASHMAP<'a> {
    boxes: [Vec<Lens<'a>>; 256],
}

impl<'a> HASHMAP<'a> {
    fn new() -> HASHMAP<'a> {
        let boxes = [(); 256].map(|_| vec![]);
        HASHMAP { boxes }
    }

    fn remove(&mut self, label: &str) -> Result<()> {
        let box_idx = hash(label)?;
        let bx = &mut self.boxes[box_idx];

        bx.retain(|lens| lens.label != label);

        Ok(())
    }

    fn insert(&mut self, label: &'a str, focal_length: u64) -> Result<()> {
        let box_idx = hash(label)?;
        let bx = &mut self.boxes[box_idx];

        if let Some(pos) = bx.iter().position(|lens| lens.label == label) {
            bx[pos] = Lens {
                label,
                focal_length,
            };
        } else {
            bx.push(Lens {
                label,
                focal_length,
            });
        }

        Ok(())
    }

    fn focusing_power(&self) -> u64 {
        let mut total_focusing_power = 0u64;

        for (box_idx, bx) in self.boxes.iter().enumerate() {
            for (lens_idx, lens) in bx.iter().enumerate() {
                total_focusing_power +=
                    (1 + box_idx as u64) * (1 + lens_idx as u64) * lens.focal_length;
            }
        }

        total_focusing_power
    }
}
