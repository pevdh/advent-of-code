use crate::*;
use eyre::eyre;
use std::fmt::Formatter;
use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq, Eq)]
pub struct CharGrid {
    data: Array2<char>,
}

impl Array2Backed<char> for CharGrid {
    fn backing_array(&self) -> &Array2<char> {
        &self.data
    }
}

impl CharGrid {
    pub fn from_shape_vec(shape: (i64, i64), data: Vec<char>) -> Result<CharGrid> {
        if shape.0 <= 0 || shape.1 <= 0 {
            return Err(eyre!("invalid shape {:?}", shape));
        }

        if (shape.0 * shape.1) as usize > data.len() {
            return Err(eyre!(
                "invalid shape {:?} for vector with length {:?}",
                shape,
                data.len()
            ));
        }

        Ok(CharGrid {
            data: Array2::new(shape, data),
        })
    }

    pub fn from_text(raw_input: &str) -> Result<CharGrid> {
        let cols = raw_input
            .lines()
            .next()
            .map(|l| l.len())
            .ok_or_else(|| eyre!("empty input"))?;

        let rows = raw_input.lines().count();

        let data: Vec<char> = raw_input.replace('\n', "").chars().collect();

        Self::from_shape_vec((rows as i64, cols as i64), data)
    }

    pub fn transposed(&self) -> CharGrid {
        CharGrid {
            data: self.data.transposed(),
        }
    }

    pub fn in_bounds(&self, pos: &(i64, i64)) -> bool {
        pos.0 >= 0 && pos.0 < self.nrows() && pos.1 >= 0 && pos.1 < self.ncols()
    }
}

impl Debug for CharGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.nrows() {
            for col in 0..self.ncols() {
                write!(f, "{}", self[(row, col)])?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl IntoIterator for CharGrid {
    type Item = char;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl Index<(i64, i64)> for CharGrid {
    type Output = char;

    fn index(&self, index: (i64, i64)) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<(i64, i64)> for CharGrid {
    fn index_mut(&mut self, index: (i64, i64)) -> &mut Self::Output {
        &mut self.data[index]
    }
}

pub trait GridLike<T> {
    fn at(&self, pos: (i64, i64)) -> Option<T>;
}

pub trait GridTools<T> {
    fn step_from(&self, init: (i64, i64), step: (i64, i64)) -> impl Iterator<Item = T>;
    fn von_neumann_neighborhood(&self, pos: (i64, i64)) -> impl Iterator<Item = T>;
    fn moore_neighborhood(&self, pos: (i64, i64)) -> impl Iterator<Item = T>;
}

impl<T, G> GridTools<T> for G
where
    G: GridLike<T>,
{
    fn step_from(&self, init: (i64, i64), step: (i64, i64)) -> impl Iterator<Item = T> {
        let mut current = init;

        std::iter::from_fn(move || {
            let result = self.at(current);
            result.as_ref()?;

            current.0 += step.0;
            current.1 += step.1;

            result
        })
    }

    fn von_neumann_neighborhood(&self, pos: (i64, i64)) -> impl Iterator<Item = T> {
        let neighborhood = [
            (0, -1), // up
            (-1, 0), // left
            (1, 0),  // right
            (0, 1),  // down
        ];

        let mut idx = 0;

        std::iter::from_fn(move || {
            while idx < neighborhood.len() {
                let next_pos = (pos.0 + neighborhood[idx].0, pos.1 + neighborhood[idx].1);

                idx += 1;

                if let Some(ch) = self.at(next_pos) {
                    return Some(ch);
                }
            }

            None
        })
    }

    fn moore_neighborhood(&self, pos: (i64, i64)) -> impl Iterator<Item = T> {
        let neighborhood = [
            (-1, -1), // top-left
            (0, -1),  // top
            (1, -1),  // top-right
            (-1, 0),  // left
            (1, 0),   // right
            (-1, 1),  // bottom-left
            (0, 1),   // bottom
            (1, 1),   // bottom-right
        ];

        let mut idx = 0;

        std::iter::from_fn(move || {
            while idx < neighborhood.len() {
                let next_pos = (pos.0 + neighborhood[idx].0, pos.1 + neighborhood[idx].1);

                idx += 1;

                if let Some(ch) = self.at(next_pos) {
                    return Some(ch);
                }
            }

            None
        })
    }
}
