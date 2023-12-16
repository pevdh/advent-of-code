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
    let hashes: Vec<u64> = input
        .split(',')
        .map(|step| hash(step).map(|hash| hash % 256))
        .collect::<Result<_>>()?;

    Ok(hashes.iter().sum())
}

fn task_2(input: &str) -> Result<u64> {
    let mut hm: HASHMAP<'_, 256> = HASHMAP::new();

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

fn hash(s: &str) -> Result<u64> {
    s.chars()
        .map(|c| {
            c.to_ascii_char().wrap_err_with(|| {
                format!("while attempting to interpret \"{}\" as an ASCII char", c)
            })
        })
        .try_fold(0_u64, |acc, ch| {
            ch.map(|ch| (acc + ch.as_byte() as u64) * 17)
        })
}

struct Lens<'a> {
    label: &'a str,
    focal_length: u64,
}

#[allow(clippy::upper_case_acronyms)]
struct HASHMAP<'a, const N: usize> {
    boxes: [Vec<Lens<'a>>; N],
}

impl<'a, const N: usize> HASHMAP<'a, N> {
    fn new() -> HASHMAP<'a, N> {
        let boxes = [(); N].map(|_| vec![]);
        HASHMAP { boxes }
    }

    fn idx(&self, label: &str) -> Result<usize> {
        Ok(hash(label)? as usize % N)
    }

    fn remove(&mut self, label: &str) -> Result<()> {
        let bx = &mut self.boxes[self.idx(label)?];

        bx.retain(|lens| lens.label != label);

        Ok(())
    }

    fn insert(&mut self, label: &'a str, focal_length: u64) -> Result<()> {
        let bx = &mut self.boxes[self.idx(label)?];

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
