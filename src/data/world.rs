//! Provides a world stored as a concpetutally infinte 2D array of chunks
//!
//! Worlds are persistent and immutable

// TODO: Use a persistent hashmap for better efficency
use std::collections::HashMap;
use data::chunk::*;
use data::block::*;

/// Persistent World
///
/// Stores the world as a conceptually infinite 2D array of chunks.
///
/// For simplicity, all chunks must currently be the same size. (By default
/// CHUNK_SIZE).
///
/// Chunks are indexed by the offset applied to each individual block.
/// E.g. with a chunk size of 10, the chunk with corners (10,0) and (20,10)
/// would be indexed with (10,0)
pub struct World {
    chunks: HashMap<(isize,isize),Chunk>,
    default_block: MetaBlock,
    chunk_size: usize,
}

impl World {
    /// Creates a new world with the provided default block
    ///
    /// Defaults to CHUNK_SIZE chunks
    pub fn new(default_block: MetaBlock) -> World {
        World {
            chunks: HashMap::new(),
            default_block: default_block,
            chunk_size: CHUNK_SIZE,
        }
    }
}


