//! Contains the heart and soul of the module, the rewind data structure
extern crate im;
extern crate uuid;

pub mod data;
pub mod storage;


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

pub struct Rewind;
