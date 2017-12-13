//! Provides a persistent, immutable array with immutable elements

use std::sync::Arc;
use std::ops::Index;

/// Persistent array
pub struct Purse<T> {
    /// Uses Arc for thread saftey
    contents: Vec<Arc<T>>,
}

impl<T> Purse<T> {
    pub fn new() -> Purse<T> {
        Purse { contents: Vec::new() }
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
