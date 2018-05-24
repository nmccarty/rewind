//! This module contains datastructures describing transactions

use chrono::prelude::*;
use data::block::*;
use std::cmp::*;
use uuid::Uuid;

/// Repusents a Transaction ID
///
/// Id is the major time, sub_id is the minor time used for resolving conflicts
#[derive(Copy, Clone, PartialEq, Eq)]
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

impl Ord for TransactionID {
    fn cmp(&self, other: &TransactionID) -> Ordering {
        let order: Ordering;
        if self.id == other.id {
            order = self.sub_id.cmp(&other.sub_id);
        } else {
            order = self.id.cmp(&other.id);
        }

        order
    }
}

impl PartialOrd for TransactionID {
    fn partial_cmp(&self, other: &TransactionID) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Describes the type of Trasnaction
///
/// Valid Transaction Types are:
/// 1. Set
///    * Blindly sets the block at a specificed location, will not check existing state
/// 2. Replace
///    * Replaces the specificed block at the specified location, will check to make sure
///      the existing block is the same as the specified block.
/// 3. Undo
///    * Undoes the transaction with the given transaction id.
///      Will make the world appear as if that transaction had never existed.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TransactionType {
    Set {
        block_set: Block,
    },
    Replace {
        block_current: Block,
        block_set: Block,
    },
    Undo {
        transaction: TransactionID,
    },
}

impl TransactionType {
    /// Creates a new set transaction
    ///
    /// Takes the block to set to
    pub fn new_set(block: Block) -> TransactionType {
        TransactionType::Set { block_set: block }
    }

    /// Creates a new Replace transaction
    ///
    /// Takes the before and the after block
    ///
    /// This transaction type will only succueed if the block that is being set is still
    /// in the orignal state when the transaction is being processed
    pub fn new_replace(original: Block, replacement: Block) -> TransactionType {
        TransactionType::Replace {
            block_current: original,
            block_set: replacement,
        }
    }

    /// Creates a new undo transaction
    ///
    /// Takes the transaction to undo
    ///
    /// This will make the world appear as if the undone transaction had never occured, retroactivly
    /// failing all the transactions that depended on the deleted one
    pub fn new_undo(transaction: TransactionID) -> TransactionType {
        TransactionType::Undo { transaction }
    }
}

/// A transaction that has not yet been processed
///
/// Contains all the information a normal transaction does, but doesn't have a transaction ID
/// associated with it, and has not yet been processed.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RawTransaction {
    /// What this transaction is actually doing
    transaction_type: TransactionType,
    /// Who did the transaction
    owner: Uuid,
    /// When they did the transaction
    time: DateTime<FixedOffset>,
    /// Where the transaction takes place
    ///
    /// Stored in (x,y,z) format
    coords: (u32, u32, u32),
}
