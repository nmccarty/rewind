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

    pub fn new_filled(length: usize, element: T) -> Purse<T> {
        let element = Arc::new(element);
        let mut new_purse = Purse::new();
        for _ in 0..length {
            new_purse.contents.push(element.clone());
        }
        new_purse
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

    /// Adds a value to the end of the Purse
    pub fn push(&self, element: T) -> Purse<T> {
        let mut new_purse = self.clone();
        new_purse.contents.push(Arc::new(element));
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



pub struct PurseIter<'a, T: 'a> {
    purse: &'a Purse<T>,
    index: usize,
}

impl<'a, T> Iterator for PurseIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.index >= self.purse.len() {
            None
        } else {
            let next = &self.purse.contents[self.index];
            self.index = self.index + 1;
            Some(next)
        }
    }
}

impl<'a, T> IntoIterator for &'a Purse<T> {
    type Item = &'a T;
    type IntoIter = PurseIter<'a, T>;

    fn into_iter(self) -> PurseIter<'a, T> {
        PurseIter {
            purse: &self,
            index: 0,
        }
    }
}
