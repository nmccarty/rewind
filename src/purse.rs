//! Provides a persistent array

use std::rc::Rc;

/// Persistent array
struct Purse<T> {
    contents: Vec<Rc<T>>,
}
