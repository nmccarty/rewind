//! Provides a persistent array with immutable elements

use std::sync::Arc;
use std::ops::Index;

/// Persistent array
pub struct Purse<T> {
    /// Uses Arc for thread saftey
    contents: Vec<Arc<T>>,
}

impl<T> Purse<T> {
    /// Creates a new, empty Purse
    pub fn new() -> Purse<T> {
        Purse { contents: Vec::new() }
    }

    /// Returns the length of the Purse
    pub fn len(&self) -> usize {
        self.contents.len()
    }

    /// "Sets" the value of the Purse at a given location
    ///
    /// # Panics
    ///
    /// Panics if the given index is out of bounds
    pub fn set(&self, index: usize, element: T) -> Purse<T> {
        let mut new_purse = self.clone();
        new_purse.contents[index] = Arc::new(element);
        new_purse
    }
}

impl<T> Clone for Purse<T> {
    fn clone(&self) -> Purse<T> {
        Purse { contents: self.contents.clone() }
    }
}

impl<T> Index<usize> for Purse<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        &self.contents[i]
    }
}
