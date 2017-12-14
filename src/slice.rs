//! Provides efficent, immutable storage of a 2D Array/Matrix

/// Common methods that a matrix must have
trait Matrix<T: Clone>
where
    T: 'static,
{
    fn new(x: usize, y: usize, default: T) -> Box<Matrix<T>>
    where
        Self: Sized;
    fn get(&self, x: usize, y: usize) -> &T;
    fn set(&self, x: usize, y: usize, element: T) -> Box<Matrix<T>>;
    fn x_size(&self) -> usize;
    fn y_size(&self) -> usize;
}

/// Simple, array based matrix
struct ArrayMatrix<T> {
    data: Vec<T>,
    x_size: usize,
    y_size: usize,
}

impl<T: Clone> Matrix<T> for ArrayMatrix<T>
where
    T: 'static,
{
    fn new(x: usize, y: usize, default: T) -> Box<Matrix<T>> {
        let mut vector = Vec::new();
        for _ in 0..(x * y) {
            vector.push(default.clone());
        }

        Box::new(ArrayMatrix {
            data: vector,
            x_size: x,
            y_size: y,
        })
    }

    fn get(&self, x: usize, y: usize) -> &T {
        &self.data[x * self.x_size + y]
    }

    fn set(&self, x: usize, y: usize, element: T) -> Box<Matrix<T>> {
        let mut vector = Vec::new();
        for i in 0..(x * y) {
            vector.push(self.data[i].clone());
        }

        vector[x * self.x_size + y] = element;
        Box::new(ArrayMatrix {
            data: vector,
            x_size: self.x_size,
            y_size: self.y_size,
        })
    }

    fn x_size(&self) -> usize {
        self.x_size
    }

    fn y_size(&self) -> usize {
        self.y_size
    }
}
