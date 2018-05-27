//! Contains the heart and soul of the module, the rewind data structure
extern crate chrono;
extern crate im;
extern crate uuid;

pub mod data;
pub mod storage;

use data::*;
use im::*;
use std::sync::{Arc, Mutex, RwLock};

/// The heart and soul of the library, the Rewind datastructre
///
/// Rewind provides a fully retroactive view of a minecraft world.
///
/// An immutable view of the world can be obtained with get_world_state.
///
/// All rewinds derived from the same initial object share a common state.
#[derive(Clone)]
pub struct Rewind {
    world_line: Arc<RwLock<WorldLine>>,
    world: Arc<RwLock<World>>,
    default_block: MetaBlock,
}

impl Rewind {
    /// Creates a new Rewind with an empty worldline and an empty world
    pub fn new(default_block: MetaBlock) -> Rewind {
        let world_line = WorldLine::new();
        let world = World::new(default_block);
        Rewind {
            world_line: Arc::new(RwLock::new(world_line)),
            world: Arc::new(RwLock::new(world)),
            default_block,
        }
    }

    /// Returns an immutable view of the world
    ///
    /// Will block until the RwLock on world becomes free
    pub fn get_world_state(&self) -> World {
        let world = self.world.read().unwrap();
        (*world).clone()
    }

    /// Will attempt to apply the given RawTransaction to the world
    ///
    /// If the transaction is sucsufully applied, a full Transaction will be returned,
    /// otherwise a None will be returned
    ///
    /// This function will obtain write locks on both world and world_line, and will block until they
    /// are avaible
    pub fn apply_transaction(&self, transaction: RawTransaction) -> Option<Transaction> {
        // First obtain the locks for the world and the world_line
        let mut world = self.world.write().unwrap();
        let mut world_line = self.world_line.write().unwrap();

        // Unwrap and process the transaction
        let transaction_type = transaction.get_transaction_type();
        match transaction_type {
            TransactionType::Set { block_set } => {
                if let Some((x, y, z)) = transaction.get_coords() {
                    *world = world.set_block_defaulting(x, y, z, block_set);
                    Some(world_line.add_transaction(transaction))
                } else {
                    None
                }
            }
            TransactionType::Replace {
                block_current,
                block_set,
            } => {
                if let Some((x, y, z)) = transaction.get_coords() {
                    let old_block = world.get_block_defaulting(x, y, z);
                    if old_block == block_current {
                        *world = world.set_block_defaulting(x, y, z, block_set);
                        Some(world_line.add_transaction(transaction))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            TransactionType::Undo { transaction: tid } => {
                // Make sure the transaction exists
                if let Some(_) = world_line.lookup_transaction(tid) {
                    // Add the Undo transaction to history first
                    let final_trans = world_line.add_transaction(transaction);
                    // Get the undone block
                    let (x, y, z) = world_line.get_undone_block(tid).unwrap();
                    // run the history
                    let history: Vec<Transaction> = world_line.get_block_history(x, y, z);
                    let new_block = run_history((&history).into_iter(), self.default_block);

                    *world = world.set_block_defaulting(x, y, z, new_block);

                    Some(final_trans)
                } else {
                    None
                }
            }
        }
    }

    /// Returns the history of the block
    ///
    /// A history is a list of (MetaBlock, Transaction) pairs, describing the state of the block, and the
    /// transaction that resulted in that state. Pairs are arranged incronological order, with the
    /// oldest first
    ///
    /// This function aquires a readlock on the world line, and will block until it is available
    pub fn get_block_history(&self, x: i32, y: i32, z: i32) -> Vec<(MetaBlock, Transaction)> {
        // Aquire the readlock on the world_line
        let world_line = self.world_line.read().unwrap();
        let transactions: Vec<Transaction> = world_line.get_block_history(x, y, z);

        let mut output = Vec::new();

        for (i, transaction) in (&transactions).into_iter().enumerate() {
            let history = (&transactions).into_iter().take(i);
            let block: MetaBlock = run_history(history, self.default_block);
            output.push((block, *transaction));
        }

        output
    }
}

/// Runs history on a slice of transactions
fn run_history<'a>(
    history: impl Iterator<Item = &'a Transaction>,
    default_block: MetaBlock,
) -> MetaBlock {
    // Vector to hold history
    let history: Vector<Transaction> = history.collect();
    // History without any of the Undos present
    let new_history: Vec<Transaction> = (&history)
        .into_iter()
        .filter(|x| !x.is_undo())
        .map(|x| (*x).clone())
        .collect();
    // Only the undos, and only the IDs
    let undos: Vec<TransactionID> = (&history)
        .into_iter()
        .filter(|x| x.is_undo())
        .map(|x| x.get_id())
        .collect();
    // Remove the undone transactions
    let final_history: Vec<Transaction> = new_history
        .into_iter()
        .filter(|x| !undos.contains(&x.get_id()))
        .collect();

    // Actually run history on the slice
    let mut block = default_block;
    for transaction in final_history.into_iter() {
        let transaction_type = transaction.get_transaction().get_transaction_type();
        match transaction_type {
            TransactionType::Set { block_set } => {
                block = block_set;
            }
            TransactionType::Replace {
                block_set,
                block_current,
            } => {
                if block == block_current {
                    block = block_set;
                }
            }
            _ => (),
        }
    }
    block
}

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

    /// Get a particular transaction
    fn lookup_transaction(&self, transaction_id: TransactionID) -> Option<Transaction> {
        if let Some(x) = self.transactions.get(&transaction_id) {
            Some(*x)
        } else {
            None
        }
    }

    /// Checks to see if a transaction has been undone, and then returns the Undo transaction
    fn get_undo(&self, transaction_id: TransactionID) -> Option<TransactionID> {
        let transactions = self.transactions.clone();
        // Check for a transaction that undoes this one
        for (k, v) in transactions.into_iter() {
            match v.get_transaction().get_transaction_type() {
                TransactionType::Undo { transaction } => {
                    if transaction == transaction_id {
                        // We can safely return the first undo, as undoing a transaction multiple
                        // times has the same effect as undoing it once
                        return Some(transaction);
                    }
                }
                _ => (),
            }
        }
        None
    }

    /// Returns the entire undo history for a transaction
    fn get_undo_history(&self, transaction_id: TransactionID) -> Vec<TransactionID> {
        let mut history = Vec::new();

        let mut transaction = Some(transaction_id);
        while let Some(t) = transaction {
            history.push(t);
            transaction = self.get_undo(t);
        }

        history
    }

    /// Returns a set of transactions that have been applied to a particular block
    ///
    /// Does not include Undos
    fn get_transactions_for_block(&self, x: i32, y: i32, z: i32) -> OrdSet<TransactionID> {
        let mut set = OrdSet::new();
        let coords = (x, y, z);

        let transactions = self.transactions.clone();
        for (k, v) in transactions.into_iter() {
            if v.get_transaction().get_coords() == Some(coords) {
                set = set.insert(k);
            }
        }

        set
    }

    /// Returns the history of all transactions to affect this particular block
    ///
    /// In chronological order, oldest first
    fn get_block_history(&self, x: i32, y: i32, z: i32) -> Vec<Transaction> {
        // Get the initial list of transactions
        let mut set = self.get_transactions_for_block(x, y, z);

        // Check to see if any of the transactions have been undone
        // and insert them into the set
        for k in set.clone().into_iter() {
            let vec = self.get_undo_history(*k);
            for v in vec {
                set = set.insert(v);
            }
        }

        // Look up the transactions and add them to the output
        let mut output: Vec<Transaction> = Vec::new();
        for id in set.into_iter() {
            let transaction = self.lookup_transaction(*id).unwrap();
            output.push(transaction);
        }

        output
    }

    /// Returns the block affected by this undo
    ///
    /// FIXME: Will break when we upgrade to affected block sets
    fn get_undone_block(&self, transaction: TransactionID) -> Option<(i32, i32, i32)> {
        // Make sure the transaction exists
        if let Some(t) = self.lookup_transaction(transaction) {
            match t.get_transaction().get_transaction_type() {
                TransactionType::Undo { transaction: tid } => self.get_undone_block(tid),
                x => t.get_transaction().get_coords(),
            }
        } else {
            None
        }
    }
}
