use crate::array::Array2Ops;
use crate::array::{Array2, Array2Backed};
use crate::*;
use eyre::eyre;
use std::ops::{AddAssign, IndexMut};

#[derive(Eq, PartialEq, Clone)]
pub struct Mat {
    data: Array2<i64>,
}

impl Mat {
    pub fn from_shape_vec(shape: (i64, i64), data: Vec<i64>) -> Mat {
        if shape.0 <= 0 || shape.1 <= 0 {
            panic!("invalid shape {:?}", shape);
        }

        if (shape.0 * shape.1) as usize > data.len() {
            panic!(
                "invalid shape {:?} for vector with length {:?}",
                shape,
                data.len()
            );
        }

        Mat {
            data: Array2::new(shape, data),
        }
    }

    pub fn from_single_digits(raw_input: &str) -> Result<Mat> {
        let cols = raw_input
            .lines()
            .next()
            .map(|l| l.len())
            .ok_or_else(|| eyre!("empty input"))?;

        let rows = raw_input.lines().count();

        let data: Vec<i64> = raw_input
            .replace("\n", "")
            .chars()
            .map(|c| c.to_digit(10).map(|d| d as i64).ok_or_parse_error())
            .collect::<Result<_>>()?;

        Ok(Self::from_shape_vec((rows as i64, cols as i64), data))
    }

    pub fn from_text(input: &str) -> Result<Mat> {
        let mut lines = input.lines();
        let first_line = lines.next().ok_or(eyre!("empty input"))?;

        let mut data = parse_nums(first_line)?;
        let ncols = data.len() as i64;
        let mut nrows = 1;
        for line in lines {
            nrows += 1;
            let row = parse_nums(line)?;
            if row.len() != ncols as usize {
                return Err(eyre!("unexpected row length {:?}, expected {:?} based on the length of the first row", row.len(), ncols));
            }
            data.extend(parse_nums(line)?.iter())
        }

        Ok(Self::from_shape_vec((nrows, ncols), data))
    }

    pub fn zeros(shape: (i64, i64)) -> Mat {
        if shape.0 < 0 || shape.1 < 0 {
            panic!("invalid shape: {:?}", shape)
        }

        let data = vec![0; (shape.0 * shape.1) as usize];

        Mat {
            data: Array2::new(shape, data),
        }
    }

    pub fn transposed(&self) -> Mat {
        Mat {
            data: self.data.transposed(),
        }
    }
}

impl Array2Backed<i64> for Mat {
    fn backing_array(&self) -> &Array2<i64> {
        &self.data
    }
}

impl Debug for Mat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in self.rows() {
            s.push_str(&format!("{:?}\n", row));
        }
        write!(f, "{}", s)
    }
}

impl Index<(i64, i64)> for Mat {
    type Output = i64;

    fn index(&self, index: (i64, i64)) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<(i64, i64)> for Mat {
    fn index_mut(&mut self, index: (i64, i64)) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl AddAssign<Mat> for Mat {
    fn add_assign(&mut self, rhs: Mat) {
        for index in self.indices() {
            self[index] += rhs[index];
        }
    }
}

impl AddAssign<&Mat> for Mat {
    fn add_assign(&mut self, rhs: &Mat) {
        for index in self.indices() {
            self[index] += rhs[index];
        }
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn from_text_should_parse_text() {
        let text = "
            1 2 3 4 5 6
            7 8 9 10 11 12
        ".trim();
        
        assert_eq!(
            Mat::from_text(text).unwrap(),
            Mat::from_shape_vec((2, 6), vec![
                1, 2, 3, 4, 5, 6,
                7, 8, 9, 10, 11, 12
            ])
        )
    }
    
    #[test]
    pub fn from_text_should_fail_on_invalid_input() {
        let text = "
            1 2 3 4 5 6
            7 8 9 10 11
        ".trim(); // second row has one less element
        
        assert!(Mat::from_text(text).is_err())
    }
    
    #[test]
    pub fn test_parse_from_single_digits() {
        let input = "123\n456\n789";
    
        let expected = Mat::from_shape_vec((3, 3), vec![
            1, 2, 3,
            4, 5, 6,
            7, 8, 9,
        ]);
    
        assert_eq!(
            Mat::from_single_digits(input).unwrap(),
            expected,
        )
    }
    
    #[test]
    pub fn test_zeros() {
        let mat = Mat::zeros((2, 3));
        
        assert_eq!(mat.shape(), (2, 3));
        assert_eq!(mat.data(), vec![0, 0, 0, 0, 0, 0]);
    }
    
    #[test]
    pub fn test_transposed() {
        let data = vec![
            1, 2, 3,
            4, 5, 6,
            7, 8, 9,
        ];
    
        let mat = Mat::from_shape_vec((3, 3), data);
        
        let data_transposed = vec![
            1, 4, 7,
            2, 5, 8,
            3, 6, 9,
        ];
        let expected_transposed = Mat::from_shape_vec((3, 3), data_transposed);
        
        assert_eq!(
            mat.transposed(),
            expected_transposed,
        )
    }
    
    #[test]
    pub fn test_rows() {
        let data = vec![
            1, 2, 3,
            4, 5, 6,
            7, 8, 9,
        ];
    
        let mat = Mat::from_shape_vec((3, 3), data);
        
        let expected_rows = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        
        assert_eq!(
            mat.rows().collect::<Vec<_>>(),
            expected_rows,
        )
    }
    
    #[test]
    pub fn test_indices() {
        let data = vec![
            1, 2, 3,
            4, 5, 6,
            7, 8, 9,
        ];

        let mat = Mat::from_shape_vec((3, 3), data);

        let expected_indices = vec![
            (0, 0), (0, 1), (0, 2),
            (1, 0), (1, 1), (1, 2),
            (2, 0), (2, 1), (2, 2),
        ];

        let actual_indices = mat.indices().collect_vec();

        assert_eq!(
            actual_indices,
            expected_indices,
        );
    }
    
    #[test]
    pub fn test_indices_row_major() {
        let data = vec![
            1, 2, 3,
            4, 5, 6,
            7, 8, 9,
        ];

        let mat = Mat::from_shape_vec((3, 3), data);

        let expected_indices = vec![
            (0, 0), (0, 1), (0, 2),
            (1, 0), (1, 1), (1, 2),
            (2, 0), (2, 1), (2, 2),
        ];

        let actual_indices = mat.indices().collect_vec();

        assert_eq!(
            actual_indices,
            expected_indices,
        );
    }
    
    #[test]
    pub fn test_indices_column_major() {
        let data = vec![
            1, 2, 3,
            4, 5, 6,
            7, 8, 9,
        ];

        let mat = Mat::from_shape_vec((3, 3), data);

        let expected_indices = vec![
            (0, 0), (1, 0), (2, 0),
            (0, 1), (1, 1), (2, 1),
            (0, 2), (1, 2), (2, 2),
        ];

        assert_eq!(
            expected_indices,
            mat.indices_col_major().collect_vec(),
        );
    }
    
    #[test]
    pub fn test_debug_print() {
        let data = vec![
            1, 2, 3,
            4, 5, 6,
        ];
        
        let mat = Mat::from_shape_vec((2, 3), data);
        
        assert_eq!(
            format!("{:?}", mat),
            "[1, 2, 3]\n[4, 5, 6]\n",
        )
    }
}
