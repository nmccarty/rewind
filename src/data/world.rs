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
    chunks: HashMap<(i32, i32), Chunk>,
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

    /// Gets the index of the provided corrdinate
    pub fn get_chunk_index(&self, x: i32, y: i32) -> (i32, i32) {
        let chunk_size = self.chunk_size as i32;
        (x - (x % chunk_size), y - (y % chunk_size))
    }

    /// Gets the chunk at a specified index
    pub fn get_chunk_at(&self, x: i32, y: i32) -> Option<&Chunk> {
        let index = self.get_chunk_index(x, y);
        self.chunks.get(&index)
    }

    /// Takes coordianates and turns them into their in chunks version
    fn convert_coords(&self, x: i32, y: i32, z: i32) -> (usize, usize, usize) {
        let x = (x.abs() as usize) % self.chunk_size;
        let y = (y.abs() as usize) % self.chunk_size;
        let z = (z.abs() as usize) % self.chunk_size;
        (x, y, z)
    }

    /// Gets the block at a specified index, if it exists
    pub fn get_block_at(&self, x: i32, y: i32, z: i32) -> Option<MetaBlock> {
        let chunk = self.get_chunk_at(x, y);
        let (x, y, z) = self.convert_coords(x, y, z);
        if let Some(chunk) = chunk {
            Some(chunk.get_block(x, y, z))
        } else {
            None
        }
    }
}
