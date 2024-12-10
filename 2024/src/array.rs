use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};

/// A 2D array stored in a single contiguous buffer (row-major order).
///
/// Example:
/// ```
/// use aoc2024::Array2D;
///
/// let data = vec![
///    1, 2, 3,
///    4, 5, 6,
/// ];
/// let array = Array2D::from_shape_vec((2, 3), data);
///
/// assert_eq!(array[(0, 0)], 1);
/// assert_eq!(array[(0, 1)], 2);
/// assert_eq!(array[(0, 2)], 3);
/// assert_eq!(array[(1, 0)], 4);
/// assert_eq!(array[(1, 1)], 5);
/// assert_eq!(array[(1, 2)], 6);
/// ```
#[derive(Clone, Eq, PartialEq)]
pub struct Array2D<T> {
    shape: (i64, i64),
    data: Vec<T>,
}

pub type Idx = (i64, i64);

impl<T> Array2D<T> {
    pub fn from_shape_vec(shape: (i64, i64), data: Vec<T>) -> Self {
        if shape.0 <= 0 || shape.1 <= 0 {
            panic!("invalid shape {:?}", shape);
        }

        if (shape.0 * shape.1) as usize != data.len() {
            panic!(
                "shape {:?} does not match data length {}",
                shape,
                data.len()
            );
        }

        Self { shape, data }
    }

    pub fn nrows(&self) -> i64 {
        self.shape.0
    }

    pub fn ncols(&self) -> i64 {
        self.shape.1
    }

    pub fn shape(&self) -> (i64, i64) {
        self.shape
    }

    pub fn in_bounds(&self, idx: Idx) -> bool {
        idx.0 >= 0 && idx.0 < self.shape.0 && idx.1 >= 0 && idx.1 < self.shape.1
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn rows(&self) -> Array2DRowsIter<T> {
        Array2DRowsIter {
            array: self,
            row: 0,
        }
    }

    pub fn indices(&self) -> Array2DIndicesIter {
        self.indices_row_major()
    }

    pub fn indices_row_major(&self) -> Array2DIndicesIter {
        Array2DIndicesIter {
            shape: self.shape,
            row: 0,
            col: 0,
            order: Order::RowMajor,
        }
    }

    pub fn indices_col_major(&self) -> Array2DIndicesIter {
        Array2DIndicesIter {
            shape: self.shape,
            row: 0,
            col: 0,
            order: Order::ColMajor,
        }
    }

    pub fn indexed_iter(&self) -> Array2DIndexedIter<T> {
        Array2DIndexedIter {
            array: self,
            indices_iter: self.indices(),
        }
    }

    pub fn iter_row_major(&self) -> Array2DIter<T> {
        Array2DIter {
            array: self,
            indices_iter: self.indices_row_major(),
        }
    }

    pub fn iter_col_major(&self) -> Array2DIter<T> {
        Array2DIter {
            array: self,
            indices_iter: self.indices_col_major(),
        }
    }

    pub fn at(&self, idx: Idx) -> Option<&T> {
        if self.in_bounds(idx) {
            Some(&self[idx])
        } else {
            None
        }
    }
}

impl<T: Clone> Array2D<T> {
    pub fn transposed(&self) -> Self {
        let mut transposed_data = Vec::with_capacity(self.data.len());
        for col in 0..self.shape.1 {
            for row in 0..self.shape.0 {
                transposed_data.push(self[(row, col)].clone());
            }
        }

        Self {
            shape: (self.shape.1, self.shape.0),
            data: transposed_data,
        }
    }

    pub fn step_from(&self, init: Idx, step: Idx) -> impl Iterator<Item = T> + '_ {
        let mut current = init;

        std::iter::from_fn(move || {
            if let Some(value) = self.at(current).cloned() {
                current.0 += step.0;
                current.1 += step.1;

                Some(value)
            } else {
                None
            }
        })
    }

    pub fn cols(&self) -> Array2DColsIter<T> {
        Array2DColsIter {
            array: self,
            col: 0,
        }
    }

    pub fn von_neumann_neighborhood(&self, pos: Idx) -> impl Iterator<Item = T> + '_ {
        self.indexed_von_neumann_neighborhood(pos)
            .map(|(_pos, value)| value)
    }

    pub fn indexed_von_neumann_neighborhood(&self, pos: Idx) -> RelativeIter<4, T> {
        let neighborhood: [Idx; 4] = [
            (0, -1), // up
            (-1, 0), // left
            (1, 0),  // right
            (0, 1),  // down
        ];

        RelativeIter {
            array: self,
            offsets: neighborhood,
            offsets_idx: 0,
            relative_from: pos,
        }
    }

    pub fn moore_neighborhood(&self, pos: Idx) -> impl Iterator<Item = T> + '_ {
        self.indexed_moore_neighborhood(pos)
            .map(|(_pos, value)| value)
    }

    pub fn indexed_moore_neighborhood(&self, pos: Idx) -> RelativeIter<8, T> {
        let neighborhood: [Idx; 8] = [
            (-1, -1), // top-left
            (0, -1),  // top
            (1, -1),  // top-right
            (-1, 0),  // left
            (1, 0),   // right
            (-1, 1),  // bottom-left
            (0, 1),   // bottom
            (1, 1),   // bottom-right
        ];

        RelativeIter {
            array: self,
            offsets: neighborhood,
            offsets_idx: 0,
            relative_from: pos,
        }
    }
}

impl<T> Index<Idx> for Array2D<T> {
    type Output = T;

    fn index(&self, index: Idx) -> &Self::Output {
        if !self.in_bounds(index) {
            panic!("index {:?} out of bounds for shape {:?}", index, self.shape);
        }

        let idx = index.0 * self.shape.1 + index.1;
        &self.data[idx as usize]
    }
}

impl<T> IndexMut<Idx> for Array2D<T> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        if !self.in_bounds(index) {
            panic!("index {:?} out of bounds for shape {:?}", index, self.shape);
        }

        let idx = index.0 * self.shape.1 + index.1;
        &mut self.data[idx as usize]
    }
}

impl<T> IntoIterator for Array2D<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

pub struct Array2DRowsIter<'a, T> {
    array: &'a Array2D<T>,
    row: i64,
}

impl<'a, T> Iterator for Array2DRowsIter<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.array.shape.0 {
            let row = self.row;
            self.row += 1;

            let start = (row * self.array.shape.1) as usize;
            let end = ((row + 1) * self.array.shape.1) as usize;
            Some(&self.array.data[start..end])
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_rows = self.array.shape.0 - self.row;
        (remaining_rows as usize, Some(remaining_rows as usize))
    }
}

pub struct Array2DColsIter<'a, T: Clone> {
    array: &'a Array2D<T>,
    col: i64,
}

impl<T> Iterator for Array2DColsIter<'_, T>
where
    T: Copy,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < self.array.shape.1 {
            let col = self.col;
            self.col += 1;

            let mut col_data = Vec::with_capacity(self.array.shape.0 as usize);
            for row in 0..self.array.shape.0 {
                col_data.push(self.array[(row, col)]);
            }
            Some(col_data)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_cols = self.array.shape.1 - self.col;
        (remaining_cols as usize, Some(remaining_cols as usize))
    }
}

enum Order {
    RowMajor,
    ColMajor,
}

pub struct Array2DIndicesIter {
    shape: (i64, i64),
    row: i64,
    col: i64,
    order: Order,
}

impl Iterator for Array2DIndicesIter {
    type Item = Idx;

    fn next(&mut self) -> Option<Self::Item> {
        match self.order {
            Order::RowMajor => {
                if self.row < self.shape.0 {
                    let idx = (self.row, self.col);
                    self.col += 1;
                    if self.col == self.shape.1 {
                        self.row += 1;
                        self.col = 0;
                    }
                    Some(idx)
                } else {
                    None
                }
            }
            Order::ColMajor => {
                if self.col < self.shape.1 {
                    let idx = (self.row, self.col);
                    self.row += 1;
                    if self.row == self.shape.0 {
                        self.col += 1;
                        self.row = 0;
                    }
                    Some(idx)
                } else {
                    None
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_elements = (self.shape.0 - self.row) * (self.shape.1 - self.col);
        (
            remaining_elements as usize,
            Some(remaining_elements as usize),
        )
    }
}

pub struct Array2DIndexedIter<'a, T> {
    array: &'a Array2D<T>,
    indices_iter: Array2DIndicesIter,
}

impl<T: Copy> Iterator for Array2DIndexedIter<'_, T> {
    type Item = (Idx, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.indices_iter.next().map(|idx| (idx, self.array[idx]))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.indices_iter.size_hint()
    }
}

pub struct Array2DIter<'a, T> {
    array: &'a Array2D<T>,
    indices_iter: Array2DIndicesIter,
}

impl<T: Copy> Iterator for Array2DIter<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.indices_iter.next().map(|idx| self.array[idx])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.indices_iter.size_hint()
    }
}

impl Debug for Array2D<i64> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_digits = self
            .data
            .iter()
            .map(|&d| d.to_string().len())
            .max()
            .unwrap_or(0);
        let max_digits = std::cmp::max(max_digits, 1);

        for row in self.rows() {
            for &cell in row {
                write!(f, "{:width$} ", cell, width = max_digits)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub struct RelativeIter<'a, const N: usize, T> {
    array: &'a Array2D<T>,
    offsets: [(i64, i64); N],
    offsets_idx: usize,
    relative_from: Idx,
}

impl<const N: usize, T> Iterator for RelativeIter<'_, N, T>
where
    T: Clone,
{
    type Item = (Idx, T);

    fn next(&mut self) -> Option<Self::Item> {
        while self.offsets_idx < N {
            let offset = self.offsets[self.offsets_idx];
            let next_pos = (
                self.relative_from.0 + offset.0,
                self.relative_from.1 + offset.1,
            );

            if self.array.in_bounds(next_pos) {
                let value = self.array[next_pos].clone();
                self.offsets_idx += 1;
                return Some((next_pos, value));
            }

            self.offsets_idx += 1;
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_elements = N - self.offsets_idx;
        (remaining_elements, Some(remaining_elements))
    }
}
