//! This module contains datastructures describing transactions

use uuid::Uuid;

/// Repusents a Transaction ID
///
/// node_id counts up from zero, and branch_id is changed every time a branching
/// operation occurs
#[derive(Copy, Clone)]
pub struct TransactionID {
    node_id: u32,
    branch_id: Uuid,
    
}

impl TransactionID {
    /// Returns a new TransactionID, starting from zero, with a new Uuid
    pub fn new() -> TransactionID {
        TransactionID {
            node_id: 0,
            branch_id: Uuid::new_v4(),
        }
    }
}



