//! Provides efficent, immutable storage of a 2D Array/Matrix

use storage::purse::*;
use std::mem::size_of;

/// Array matrix
struct ArrayMatrix<T> {
    data: Purse<Option<T>>,
    x_size: usize,
    y_size: usize,
}

impl<T> ArrayMatrix<T> {
    fn new(x_size: usize, y_size: usize) -> ArrayMatrix<T> {
        ArrayMatrix {
            data: Purse::new_filled(x_size * y_size, None),
            x_size: x_size,
            y_size: y_size,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data[x * self.x_size + y].as_ref()
    }

    fn set(&self, x: usize, y: usize, data: T) -> ArrayMatrix<T> {
        ArrayMatrix {
            data: self.data.set(x * self.x_size + y, Some(data)),
            x_size: self.x_size,
            y_size: self.y_size,
        }
    }
}

/// Sparse matrix
///
/// Currently implemented with COO for simplicity
struct SparseMatrix<T> {
    coords: Vec<(usize, usize)>,
    data: Purse<T>,
    x_size: usize,
    y_size: usize,
}

impl<T> SparseMatrix<T> {
    fn new(x_size: usize, y_size: usize) -> SparseMatrix<T> {
        SparseMatrix {
            coords: Vec::new(),
            data: Purse::new(),
            x_size: x_size,
            y_size: y_size,
        }
    }

    fn get_index(&self, x: usize, y: usize) -> Option<usize> {
        self.coords.iter().position(|&r| r == (x, y))
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        let index = self.get_index(x, y);
        match index {
            Some(x) => Some(&self.data[x]),
            None => None,
        }
    }

    fn set(&self, x: usize, y: usize, data: T) -> SparseMatrix<T> {
        let index = self.get_index(x, y);
        let mut new_coords = self.coords.clone();
        let new_data;
        if let Some(index) = index {
            new_data = self.data.set(index, data);
        } else {
            new_coords.push((x, y));
            new_data = self.data.push(data);
        }

        SparseMatrix {
            coords: new_coords,
            data: new_data,
            x_size: self.x_size,
            y_size: self.y_size,
        }
    }

    fn size(&self) -> usize {
        let coords_size = self.coords.len() * size_of::<(usize, usize)>();
        let data_size = self.coords.len() * size_of::<T>();
        coords_size + data_size
    }

    fn packable(&self) -> bool {
        self.size() >= self.x_size * self.y_size * size_of::<T>()
    }
}

/// Provides a consistent interface for either type of matrix
enum Matrix<T> {
    SMatrix(SparseMatrix<T>),
    AMatrix(ArrayMatrix<T>),
}

impl<T> Matrix<T> {
    /// Creates a new empty matrix, defaulting to SparseMatrix
    fn new(x_size: usize, y_size: usize) -> Matrix<T> {
        Matrix::SMatrix(SparseMatrix::new(x_size, y_size))
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        match self {
            &Matrix::SMatrix(ref i) => i.get(x, y),
            &Matrix::AMatrix(ref i) => i.get(x, y),
        }
    }

    fn set(&self, x: usize, y: usize, data: T) -> Matrix<T> {
        match self {
            &Matrix::SMatrix(ref m) => {
                let new_matrix = Matrix::SMatrix(m.set(x, y, data));
                // TODO: Implement repacking here
                new_matrix
            }
            &Matrix::AMatrix(ref m) => Matrix::AMatrix(m.set(x, y, data)),
        }
    }
}

/// Provides abstraction of an immutable, 2D array, with a default value
pub struct Slice<T> {
    matrix: Matrix<T>,
    default: T,
    x_size: usize,
    y_size: usize,
}

impl<T> Slice<T> {
    pub fn new(x_size: usize, y_size: usize, default: T) -> Slice<T> {
        Slice {
            matrix: Matrix::new(x_size, y_size),
            default: default,
            x_size: x_size,
            y_size: y_size,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        if x >= self.x_size || y >= self.y_size {
            &self.default
        } else {
            self.matrix.get(x, y).unwrap_or(&self.default)
        }
    }
}
