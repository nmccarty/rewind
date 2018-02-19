//! Provides an efficent, immutable repusentation of a chunk in memory

use storage::cuboid::*;
use std::sync::Arc;
use data::block::*;

/// Persistent chunk
pub struct Chunk {
    /// Dictonary provided to this chunk by the world
    ///
    /// Use Arc for thread saftey. Uses Arc instead of mutex due to immutability
    dictonary: Arc<BlockDictonary>,
    /// Collection of blocks making up this chunk
    blocks: Cuboid<Block>,
    /// Default block for this cunk
    default_block: Block,
    /// x size of this chunk
    x_size: usize,
    /// y size of this chunk
    y_size: usize,
    /// z size of this chunk
    z_size: usize,
}
