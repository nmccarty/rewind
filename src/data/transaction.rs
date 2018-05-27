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
    ///
    /// Resets minor id to 0
    pub fn increment_major(&self) -> TransactionID {
        let id = self.id + 1;
        TransactionID { id, sub_id: 0 }
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
        block_set: MetaBlock,
    },
    Replace {
        block_current: MetaBlock,
        block_set: MetaBlock,
    },
    Undo {
        transaction: TransactionID,
    },
}

impl TransactionType {
    /// Creates a new set transaction
    ///
    /// Takes the block to set to
    pub fn new_set(block: MetaBlock) -> TransactionType {
        TransactionType::Set { block_set: block }
    }

    /// Creates a new Replace transaction
    ///
    /// Takes the before and the after block
    ///
    /// This transaction type will only succueed if the block that is being set is still
    /// in the orignal state when the transaction is being processed
    pub fn new_replace(original: MetaBlock, replacement: MetaBlock) -> TransactionType {
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
///
/// This has several optional or defaulting behavior fields, so it the builder should be used
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RawTransaction {
    /// What this transaction is actually doing
    transaction_type: TransactionType,
    /// Who did the transaction
    ///
    /// This will default to the null Uuid, but this should be avoided if at all possible.
    /// If there is any information avaible about who performed a transaction, even a best guess,
    /// it would be wise to use the Uuid associated with that entitiy.
    owner: Uuid,
    /// When they did the transaction
    time: Option<DateTime<FixedOffset>>,
    /// Where the transaction takes place
    ///
    /// Stored in (x,y,z) format
    ///
    /// This is optional because some transactions don't explicitly refrence a block, like an
    /// undo, and others may refrence large sections of blocks, like a SetCuboid
    coords: Option<(i32, i32, i32)>,
}

impl RawTransaction {
    /// Returns the TransactionType
    pub fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type
    }

    /// Returns the owner of the transaction
    pub fn get_owner(&self) -> Uuid {
        self.owner
    }

    /// Returns the wall-clock time the transaction occured at
    pub fn get_time(&self) -> Option<DateTime<FixedOffset>> {
        self.time
    }

    /// Returns the coordinantes of the block this transaction effects
    pub fn get_coords(&self) -> Option<(i32, i32, i32)> {
        self.coords
    }
}

/// A builder for transactions
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RawTransactionBuilder {
    transaction_type: TransactionType,
    owner: Option<Uuid>,
    time: Option<DateTime<FixedOffset>>,
    coord_x: Option<i32>,
    coord_y: Option<i32>,
    coord_z: Option<i32>,
}

impl RawTransactionBuilder {
    /// Creates a new, empty RawTransactionBuilder
    ///
    /// Requries a Transaction type, as that is the only non-optional data
    pub fn new(transaction_type: TransactionType) -> RawTransactionBuilder {
        RawTransactionBuilder {
            transaction_type,
            owner: None,
            time: None,
            coord_x: None,
            coord_y: None,
            coord_z: None,
        }
    }

    /// Converts the builder into a transaction
    ///
    /// Does not consume
    pub fn build_transaction(&self) -> Option<RawTransaction> {
        let transaction_type = self.transaction_type;
        // If an owner was not provided, we are forced to default to the null Uuid
        let owner = self.owner
            .unwrap_or(Uuid::parse_str("0000000000000000000000000000000").unwrap());
        let time = self.time;
        let coords: Option<(i32, i32, i32)> = match (self.coord_x, self.coord_y, self.coord_z) {
            (Some(x), Some(y), Some(z)) => Some((x, y, z)),
            _ => None,
        };

        let transaction = RawTransaction {
            transaction_type,
            owner,
            time,
            coords,
        };

        // Fail the build if the transaction requires coordinates, but does not have them
        match transaction_type {
            TransactionType::Set { .. } => if coords.is_some() {
                Some(transaction)
            } else {
                None
            },
            TransactionType::Replace { .. } => if coords.is_some() {
                Some(transaction)
            } else {
                None
            },
            TransactionType::Undo { .. } => Some(transaction),
        }
    }

    /// Sets the owner of the transaction
    pub fn set_owner(&mut self, owner: Uuid) -> &mut Self {
        self.owner = Some(owner);
        self
    }

    /// Sets the time wall-clock time the transaction took place at
    pub fn set_time(&mut self, time: DateTime<FixedOffset>) -> &mut Self {
        self.time = Some(time);
        self
    }

    /// Sets the wall-clock time of the transaction to now
    pub fn set_time_now(&mut self) -> &mut Self {
        let local_time = Local::now();
        self.time = Some(local_time.with_timezone(local_time.offset()));
        self
    }

    /// Sets the x coordinate the transaction takes place at
    pub fn set_x_coord(&mut self, x: i32) -> &mut Self {
        self.coord_x = Some(x);
        self
    }

    /// Sets the x coordinate the transaction takes place at
    pub fn set_y_coord(&mut self, y: i32) -> &mut Self {
        self.coord_y = Some(y);
        self
    }

    /// Sets the x coordinate the transaction takes place at
    pub fn set_z_coord(&mut self, z: i32) -> &mut Self {
        self.coord_z = Some(z);
        self
    }
}

/// A transaction that has been commited to the world and has been assigned a transaction ID
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Transaction {
    /// The details about the transaction are stored in the corrosponding RawTransaction
    transaction: RawTransaction,
    /// The transaction id assinged to this transaction
    id: TransactionID,
}

impl Transaction {
    /// Creates a new Transaction from a RawTransaction and a TransactionID
    pub fn new(transaction: RawTransaction, id: TransactionID) -> Transaction {
        Transaction { transaction, id }
    }

    /// Returns the RawTransaction
    pub fn get_transaction(&self) -> RawTransaction {
        self.transaction
    }

    /// Returns the transaction id
    pub fn get_id(&self) -> TransactionID {
        self.id
    }
}
