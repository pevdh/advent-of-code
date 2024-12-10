use crate::GridLike;
use std::ops::{Index, IndexMut};

#[derive(Clone, Eq, PartialEq)]
pub struct Array2<T> {
    shape: (i64, i64),
    data: Vec<T>,
}

impl<T: Copy> Array2<T> {
    pub fn new(shape: (i64, i64), data: Vec<T>) -> Self {
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

    pub fn shape(&self) -> (i64, i64) {
        self.shape
    }

    pub fn transposed(&self) -> Self {
        let mut transposed_data = Vec::with_capacity(self.data.len());
        for col in 0..self.shape.1 {
            for row in 0..self.shape.0 {
                transposed_data.push(self[(row, col)]);
            }
        }

        Self {
            shape: (self.shape.1, self.shape.0),
            data: transposed_data,
        }
    }

    pub fn at(&self, pos: (i64, i64)) -> Option<T> {
        if pos.0 < 0 || pos.0 >= self.shape.0 || pos.1 < 0 || pos.1 >= self.shape.1 {
            None
        } else {
            Some(self[pos])
        }
    }

    pub fn rows(&self) -> Array2Rows<T> {
        Array2Rows {
            array: self,
            row: 0,
        }
    }

    pub fn cols(&self) -> Array2Cols<T> {
        Array2Cols {
            array: self,
            col: 0,
        }
    }

    pub fn indices(&self) -> Array2Indices {
        self.indices_row_major()
    }

    pub fn indices_row_major(&self) -> Array2Indices {
        Array2Indices {
            shape: self.shape,
            row: 0,
            col: 0,
            order: Order::RowMajor,
        }
    }

    pub fn indices_col_major(&self) -> Array2Indices {
        Array2Indices {
            shape: self.shape,
            row: 0,
            col: 0,
            order: Order::ColMajor,
        }
    }

    pub fn indexed_iter(&self) -> Array2IndexedIter<T> {
        Array2IndexedIter {
            array: self,
            indices_iter: self.indices(),
        }
    }

    pub fn iter_row_major(&self) -> Array2Iter<T> {
        Array2Iter {
            array: self,
            indices_iter: self.indices_row_major(),
        }
    }

    pub fn iter_col_major(&self) -> Array2Iter<T> {
        Array2Iter {
            array: self,
            indices_iter: self.indices_col_major(),
        }
    }
}

impl<T> Index<(i64, i64)> for Array2<T> {
    type Output = T;

    fn index(&self, index: (i64, i64)) -> &Self::Output {
        let row = index.0;
        let col = index.1;
        let idx = (row * self.shape.0 + col) as usize;

        if idx >= self.data.len() {
            panic!(
                "index {:?} out of bounds for Array2 with shape {:?}",
                index, self.shape
            );
        }

        &self.data[idx]
    }
}

impl<T> IndexMut<(i64, i64)> for Array2<T> {
    fn index_mut(&mut self, index: (i64, i64)) -> &mut Self::Output {
        let row = index.0;
        let col = index.1;
        let idx = (row * self.shape.0 + col) as usize;

        if idx >= self.data.len() {
            panic!(
                "index {:?} out of bounds for Array2 with shape {:?}",
                index, self.shape
            );
        }

        &mut self.data[idx]
    }
}

impl<T> IntoIterator for Array2<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

pub trait Array2Backed<T> {
    fn backing_array(&self) -> &Array2<T>;
}

pub trait Array2Ops<T> {
    fn nrows(&self) -> i64;
    fn ncols(&self) -> i64;
    fn shape(&self) -> (i64, i64);
    fn data(&self) -> &[T];

    fn transposed(&self) -> Array2<T>;
    fn at(&self, pos: (i64, i64)) -> Option<T>;

    fn rows(&self) -> Array2Rows<T>;
    fn cols(&self) -> Array2Cols<T>;
    fn indices(&self) -> Array2Indices;
    fn indices_row_major(&self) -> Array2Indices;
    fn indices_col_major(&self) -> Array2Indices;

    fn indexed_iter(&self) -> Array2IndexedIter<'_, T>;
    fn iter_row_major(&self) -> Array2Iter<'_, T>;
    fn iter_col_major(&self) -> Array2Iter<'_, T>;
}

pub struct Array2Rows<'a, T> {
    array: &'a Array2<T>,
    row: i64,
}

impl<'a, T> Iterator for Array2Rows<'a, T> {
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

pub struct Array2Cols<'a, T> {
    array: &'a Array2<T>,
    col: i64,
}

impl<T> Iterator for Array2Cols<'_, T>
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

pub struct Array2Indices {
    shape: (i64, i64),
    row: i64,
    col: i64,
    order: Order,
}

impl Iterator for Array2Indices {
    type Item = (i64, i64);

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
}

pub struct Array2IndexedIter<'a, T> {
    array: &'a Array2<T>,
    indices_iter: Array2Indices,
}

impl<T: Copy> Iterator for Array2IndexedIter<'_, T> {
    type Item = ((i64, i64), T);

    fn next(&mut self) -> Option<Self::Item> {
        self.indices_iter.next().map(|idx| (idx, self.array[idx]))
    }
}

pub struct Array2Iter<'a, T> {
    array: &'a Array2<T>,
    indices_iter: Array2Indices,
}

impl<T: Copy> Iterator for Array2Iter<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.indices_iter.next().map(|idx| self.array[idx])
    }
}

impl<A, T> Array2Ops<T> for A
where
    A: Array2Backed<T>,
    T: Copy,
{
    fn nrows(&self) -> i64 {
        self.backing_array().shape.0
    }

    fn ncols(&self) -> i64 {
        self.backing_array().shape.1
    }

    fn shape(&self) -> (i64, i64) {
        self.backing_array().shape()
    }

    fn data(&self) -> &[T] {
        &self.backing_array().data
    }

    fn transposed(&self) -> Array2<T> {
        self.backing_array().transposed()
    }

    fn at(&self, pos: (i64, i64)) -> Option<T> {
        self.backing_array().at(pos)
    }

    fn rows(&self) -> Array2Rows<T> {
        self.backing_array().rows()
    }

    fn cols(&self) -> Array2Cols<T> {
        self.backing_array().cols()
    }

    fn indices(&self) -> Array2Indices {
        self.backing_array().indices()
    }

    fn indices_row_major(&self) -> Array2Indices {
        self.backing_array().indices_row_major()
    }

    fn indices_col_major(&self) -> Array2Indices {
        self.backing_array().indices_col_major()
    }

    fn indexed_iter(&self) -> Array2IndexedIter<T> {
        self.backing_array().indexed_iter()
    }

    fn iter_row_major(&self) -> Array2Iter<T> {
        self.backing_array().iter_row_major()
    }

    fn iter_col_major(&self) -> Array2Iter<T> {
        self.backing_array().iter_col_major()
    }
}

impl<T, A> GridLike<T> for A
where
    A: Array2Backed<T>,
    T: Copy,
{
    fn at(&self, pos: (i64, i64)) -> Option<T> {
        self.backing_array().at(pos)
    }
}
