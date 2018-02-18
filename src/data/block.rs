//! Provides an implementation of a single block
//!
//! Uses minecraft stile "table name":"key" format

use std::collections::HashMap;

/// Structure that stores a single Block
/// Needs to be paired with a BlockDictonary to get useful values
pub struct Block {
    provider: u16,
    blockid: u16,
}

/// Provides a dictonary from provider:blockname values to u16:u16 values
pub struct BlockDictonary {
    provider_to_blocktable: HashMap<&str, BlockTable>,
}

/// Provides the table for a single block provider
pub struct BlockTable {
    provider_name: &str,
    name_to_val: HashMap<&str, u16>,
    val_to_name: HashMap<u16, &str>,
}
