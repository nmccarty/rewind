//! This module provides the data structures need by the other parts of the appication

pub mod cuboid;
pub mod purse;
pub mod slice;

use std::sync::Arc;

/// Implements a simple LinkedList
pub enum AList<T> {
    /// Uses Arc to manage owernship for fast cloning and thread saftey
    Node(Arc<T>, Arc<AList<T>>),
    End,
}

impl<T> AList<T> {
    /// Constructs a new, empty ALList
    pub fn new() -> AList<T> {
        AList::End
    }

    /// adds an element to the front of the ALList
    pub fn push(&self, element: T) -> AList<T> {
        let prev = self.clone();
        AList::Node(Arc::new(element), Arc::new(prev))
    }

    /// Advances the list one position
    fn self_next(&mut self) {
        if let AList::Node(_element, next) = self {
            let next: &AList<T> = next;
            match next {
                AList::End => *self = AList::End,
                AList::Node(element, new_next) => {
                    *self = AList::Node(element.clone(), new_next.clone())
                }
            }
        }
    }

    /// Returns the first element of the list, if one exists, and advances the head of the list
    /// forward by one positon
    pub fn pop(&mut self) -> Option<&T> {
        match self {
            AList::End => None,
            AList::Node(element, next) => {
                let new_next: &AList<T> = next;
                *self = new_next.clone();
                Some(element)
            }
        }
    }
}

impl<T> Clone for AList<T> {
    fn clone(&self) -> AList<T> {
        match self {
            AList::End => AList::End,
            AList::Node(arc1, arc2) => AList::Node(arc1.clone(), arc2.clone()),
        }
    }
}
