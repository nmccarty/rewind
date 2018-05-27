//! Contains the heart and soul of the module, the rewind data structure
extern crate chrono;
extern crate im;
extern crate uuid;

pub mod data;
pub mod storage;

use data::*;
use im::*;

/// The heart and soul of the library, the Rewind datastructre
///
/// Rewind provides a fully retroactive view of a minecraft world.
///
/// Rewinds can be either mutable or immutable, and once set to immutable,
/// can not be made mutable again without a forking operation.
///
/// An immutable rewind always shows the same view of a world, and is locked
/// to a specific branch id and transaction id.
///
/// All rewinds along a given world tree share a common, add-only database of transactions
/// and branch ids
#[derive(Clone)]
pub struct Rewind;

/// Contains and manages the list of transactions in a world
#[derive(Clone)]
struct WorldLine {
    /// The list of transactions is stored as an OrdMap to allow lookup by transaction id
    /// when there have been inserted transaction revisions
    transactions: OrdMap<TransactionID, Transaction>,
}

impl WorldLine {
    /// Creates a new WorldLine, with an empty transaction log
    fn new() -> WorldLine {
        WorldLine {
            transactions: OrdMap::new(),
        }
    }

    /// Adds a transaction to the worldline
    fn add_transaction(&mut self, transaction: RawTransaction) -> Transaction {
        // Get the TransactionID of the last transaction in the worldline
        let last_transaction = self.transactions.get_max();
        let id = match last_transaction {
            Some((t, _)) => t.increment_major(),
            None => TransactionID::new(),
        };

        let new_transaction = Transaction::new(transaction, id);

        // Add the new transaction to the list
        self.transactions = self.transactions.insert(id, new_transaction);

        new_transaction
    }
}
