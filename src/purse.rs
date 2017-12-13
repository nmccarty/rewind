//! Provides a persistent, immutable array with immutable elements

use std::sync::Arc;

/// Persistent array
struct Purse<T> {
    contents: Vec<Arc<T>>,
}

impl<T> Purse<T> {
    fn new() -> Purse<T> {
        Purse { contents: Vec::new(), }
    }
}
