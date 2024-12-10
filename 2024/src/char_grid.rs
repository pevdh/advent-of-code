/// A 2D grid of characters
use crate::*;
use eyre::eyre;
use std::fmt::Formatter;

pub type CharGrid = Array2D<char>;

impl CharGrid {
    pub fn from_text(raw_input: &str) -> Result<CharGrid> {
        let cols = raw_input
            .lines()
            .next()
            .map(|l| l.len())
            .ok_or_else(|| eyre!("empty input"))?;

        let rows = raw_input.lines().count();

        let data: Vec<char> = raw_input.replace('\n', "").chars().collect();

        Ok(Self::from_shape_vec((rows as i64, cols as i64), data))
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
