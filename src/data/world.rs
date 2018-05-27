//! Provides a world stored as a concpetutally infinte 2D array of chunks
//!
//! Worlds are persistent and immutable

use data::*;
use im::*;
use std::sync::Arc;

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
#[derive(Clone)]
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
        let result = self.chunks.get(&index);
        match result {
            Some(x) => Some(&*x),
            None => None,
        }
    }

    /// Returns true if the chunk at the specificed index exists
    pub fn has_chunk_at(&self, x: i32, y: i32) -> bool {
        let index = self.get_chunk_index(x, y);
        self.chunks.contains_key(&index)
    }

    /// Creates a new chunk at the specificed index
    fn create_chunk(&self, x: i32, y: i32) -> HashMap<(i32, i32), Chunk> {
        let mut new_chunks = self.chunks.clone();
        new_chunks.insert((x, y), Chunk::new(*self.default_block.get_block()));
        new_chunks
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

    /// Attempts to get the specified block
    ///
    /// Will return the default if that block does not exist
    pub fn get_block_defaulting(&self, x: i32, y: i32, z: i32) -> MetaBlock {
        let maybe_block = self.get_block_at(x, y, z);
        if let Some(block) = maybe_block {
            block
        } else {
            self.default_block
        }
    }

    /// Returns true if the chunk a block is in exists, false otherwise
    pub fn block_exists(&self, x: i32, y: i32, _z: i32) -> bool {
        let (x, y) = self.get_chunk_index(x, y);
        self.has_chunk_at(x, y)
    }

    /// Sets the block at the specified location, creating the chunk if it
    /// doesnt exist
    pub fn set_block_defaulting(&self, x: i32, y: i32, z: i32, block: MetaBlock) -> World {
        let index = self.get_chunk_index(x, y);
        let (cx, cy, cz) = self.convert_coords(x, y, z);
        let empty_chunk = Chunk::new(*self.default_block.get_block());
        let old_chunk = self.chunks.get(&index).unwrap_or(Arc::new(empty_chunk));
        let new_chunks = self.chunks.insert(index, old_chunk.set_block(cx, cy, cz, block));

        World {
            chunks: new_chunks,
            default_block: self.default_block,
            chunk_size: self.chunk_size,
        }
    }
}
