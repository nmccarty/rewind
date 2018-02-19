//! Provides an efficent, immutable repusentation of a chunk in memory

use storage::cuboid::*;
use data::block::*;
use std::sync::Arc;

/// Persistent chunk
///
/// Chunks can have any x,y,z
#[derive(Clone)]
pub struct Chunk {
    /// Dictonary provided to this chunk by the world
    ///
    /// Use Arc for thread saftey. Uses Arc instead of mutex due to immutability.
    /// Option is used as it does make sense to have a Chunk without a
    /// dictonary, if you only care about numerical IDs and not the
    /// minecraft style provider:id names.
    dictonary: Option<Arc<BlockDictonary>>,
    /// Collection of blocks making up this chunk
    blocks: Cuboid<Block>,
    /// MetaData belonging to those Blocks
    meta_data: Cuboid<MetaData>,
    /// Default block for this cunk
    default_block: Block,
    /// x size of this chunk
    x_size: usize,
    /// y size of this chunk
    y_size: usize,
    /// z size of this chunk
    z_size: usize,
}

/// Default size of a chunk (chunks default to cubes)
const CHUNK_SIZE: usize = 256;

impl Chunk {
    /// Creates a new chunk with the specificed default block
    ///
    /// Defaults to a chunk size of 256x256x256.
    /// Defaults to no dictionary.
    pub fn new(default_block: Block) -> Chunk {
        let blank_meta = MetaData::new();
        Chunk {
            dictonary: None,
            blocks: Cuboid::new(CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE, &default_block),
            meta_data: Cuboid::new(CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE, &blank_meta),
            default_block: default_block,
            x_size: CHUNK_SIZE,
            y_size: CHUNK_SIZE,
            z_size: CHUNK_SIZE,
        }
    }

    /// Sets the dictionary to be used by this chunk
    pub fn set_dict(&self, dictonary: &Arc<BlockDictonary>) -> Chunk {
        let mut new_chunk = self.clone();
        new_chunk.dictonary = Some(dictonary.clone());
        new_chunk
    }

    /// Gets the block at a specificed location, by value
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> MetaBlock {
        let block = self.blocks.get(x, y, z).clone();
        let meta = self.meta_data.get(x, y, z).clone();
        MetaBlock::fuse(block, meta)
    }

    /// Sets the block at a specified location, by value
    pub fn set_block(&self, x: usize, y: usize, z: usize, block: MetaBlock) -> Chunk {
        let mut new_chunk = self.clone();
        new_chunk.blocks = self.blocks
            .set(x,y,z,block.get_block())
            .unwrap_or(self.blocks.clone());
        new_chunk.meta_data = self.meta_data
            .set(x,y,z,block.get_meta_data())
            .unwrap_or(self.meta_data.clone());
        new_chunk
    }
}
