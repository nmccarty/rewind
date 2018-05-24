//! This module contains datastructures describing transactions

/// Repusents a Transaction ID
///
/// Id is the major time, sub_id is the minor time used for resolving conflicts
#[derive(Copy, Clone)]
pub struct TransactionID {
    id: u32,
    sub_id: u32,
}

impl TransactionID {
    /// Returns a new TransactionID, starting from zero, with a new Uuid
    pub fn new() -> TransactionID {
        TransactionID { id: 0, sub_id: 0 }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_sub_id(&self) -> u32 {
        self.sub_id
    }

    pub fn new_from_parts(id: u32, sub_id: u32) -> TransactionID {
        TransactionID { id, sub_id }
    }

    /// Increments the major id (id) by one
    pub fn increment_major(&self) -> TransactionID {
        let id = self.id + 1;
        let sub_id = self.sub_id;
        TransactionID { id, sub_id }
    }

    /// Increments the minor id (sub_id) by one
    pub fn increment_minor(&self) -> TransactionID {
        let id = self.id;
        let sub_id = self.sub_id;
        TransactionID { id, sub_id }
    }
}
